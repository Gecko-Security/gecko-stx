#![allow(unused_variables)]

use crate::analysis::annotation::{Annotation, AnnotationKind, WarningKind};
use crate::analysis::ast_visitor::{traverse, ASTVisitor, TypedVar};
use crate::analysis::{self, AnalysisPass, AnalysisResult};
use crate::repl::DEFAULT_EPOCH;
use clarity::vm::analysis::analysis_db::AnalysisDatabase;
use clarity::vm::analysis::types::ContractAnalysis;
use clarity::vm::diagnostic::{DiagnosableError, Diagnostic, Level};
use clarity::vm::functions::define::DefineFunctions;
use clarity::vm::functions::NativeFunctions;
use clarity::vm::representations::Span;
use clarity::vm::representations::SymbolicExpressionType::*;
use clarity::vm::types::TypeSignature;
use clarity::vm::{ClarityName, ClarityVersion, SymbolicExpression};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub struct CheckError;

impl DiagnosableError for CheckError {
    fn message(&self) -> String {
        message.to_string()
    }
    fn suggestion(&self) -> Option<String> {
        None
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
enum Node<'a> {
    Symbol(&'a ClarityName),
    Expr(u64),
}

#[derive(Clone, Debug)]
struct TaintSource<'a> {
    span: Span,
    children: HashSet<Node<'a>>,
}

#[derive(Clone, Debug)]
struct TaintedNode<'a> {
    sources: HashSet<Node<'a>>,
}

struct FunctionInfo {
    unchecked_params: Vec<bool>,
    filtered_params: Vec<bool>,
}

pub struct Gecko<'a> {
    taint_sources: HashMap<Node<'a>, TaintSource<'a>>,
    tainted_nodes: HashMap<Node<'a>, TaintedNode<'a>>,
    diagnostics: HashMap<u64, Vec<Diagnostic>>,
    annotations: &'a Vec<Annotation>,
    active_annotation: Option<usize>,
    public_funcs: HashSet<&'a ClarityName>,
    user_funcs: HashMap<&'a ClarityName, FunctionInfo>,
    in_as_contract: bool,
}

impl<'a> Gecko<'a> {
    fn new(annotations: &'a Vec<Annotation>) -> Gecko<'a> {
        Self {
            taint_sources: HashMap::new(),
            tainted_nodes: HashMap::new(),
            diagnostics: HashMap::new(),
            annotations,
            active_annotation: None,
            public_funcs: HashSet::new(),
            user_funcs: HashMap::new(),
            in_as_contract: false,
        }
    }

    fn run(mut self, contract_analysis: &'a ContractAnalysis) -> AnalysisResult {
        // First traverse the entire AST
        traverse(&mut self, &contract_analysis.expressions);
        // Collect all of the vecs of diagnostics into a vector
        let mut diagnostics: Vec<Vec<Diagnostic>> = self.diagnostics.into_values().collect();
        // Order the sets by the span of the error (the first diagnostic)
        diagnostics.sort_by(|a, b| a[0].spans[0].cmp(&b[0].spans[0]));
        // Then flatten into one vector
        Ok(diagnostics.into_iter().flatten().collect())
    }

    fn add_taint_source(&mut self, node: Node<'a>, span: Span) {
        let source_node = self.taint_sources.insert(
            node,
            TaintSource {
                span,
                children: HashSet::new(),
            },
        );
        let mut sources = HashSet::new();
        sources.insert(node);
        self.tainted_nodes.insert(node, TaintedNode { sources });
    }

    fn add_taint_source_symbol(&mut self, name: &'a ClarityName, span: Span) {
        self.add_taint_source(Node::Symbol(name), span);
    }

    fn add_tainted_node_to_sources(&mut self, node: Node<'a>, sources: &HashSet<Node<'a>>) {
        for source_node in sources {
            let source = self.taint_sources.get_mut(source_node).unwrap();
            source.children.insert(node);
        }
    }

    fn add_tainted_expr(&mut self, expr: &'a SymbolicExpression, sources: HashSet<Node<'a>>) {
        let node = Node::Expr(expr.id);
        self.add_tainted_node_to_sources(node, &sources);
        self.tainted_nodes.insert(node, TaintedNode { sources });
    }

    fn add_tainted_symbol(&mut self, name: &'a ClarityName, sources: HashSet<Node<'a>>) {
        let node = Node::Symbol(name);
        self.add_tainted_node_to_sources(node, &sources);
        self.tainted_nodes.insert(node, TaintedNode { sources });
    }

    fn taint_check(&mut self, expr: &'a SymbolicExpression) {
        if self.tainted_nodes.contains_key(&Node::Expr(expr.id)) {
            self.diagnostics
                .insert(expr.id, self.generate_diagnostics(expr));
        }
    }

    fn filter_source(&mut self, source_node: &Node<'a>, rollback: bool) {
        if let Some(source) = self.taint_sources.remove(source_node) {
            self.tainted_nodes.remove(source_node);
            for child in &source.children {
                if let Some(mut child_node) = self.tainted_nodes.remove(child) {
                    child_node.sources.remove(source_node);
                    if !child_node.sources.is_empty() {
                        self.tainted_nodes.insert(*child, child_node);
                    } else if rollback {
                        if let Node::Expr(id) = child {
                            // Remove any prior diagnostics for this node
                            self.diagnostics.remove(id);
                        }
                    }
                } else if rollback {
                    if let Node::Expr(id) = child {
                        // Remove any prior diagnostics for this node
                        self.diagnostics.remove(id);
                    }
                }
            }
        }
    }

    fn filter_taint(&mut self, expr: &SymbolicExpression, rollback: bool) {
        let node = Node::Expr(expr.id);
        if let Some(removed_node) = self.tainted_nodes.remove(&node) {
            for source_node in &removed_node.sources {
                self.filter_source(source_node, rollback);
            }
        }
    }

    impl<'a> Gecko<'a> {
        //lol
        fn detect_unsafe_authentication(&mut self, expr: &'a SymbolicExpression) {
            if let SymbolicExpressionType::List(exprs) = &expr.expr {
                if exprs.len() == 3 {
                    if let Some(first) = exprs.get(0).and_then(|e| e.match_atom()) {
                        if first == "asserts!" {
                            if let Some(cond_expr) = exprs.get(1) {
                                if let SymbolicExpressionType::List(cond_exprs) = &cond_expr.expr {
                                    if cond_exprs.len() == 3 {
                                        if let Some(cond_func) = cond_exprs.get(0).and_then(|e| e.match_atom()) {
                                            if cond_func == "is-eq" {
                                                if let Some(left) = cond_exprs.get(1).and_then(|e| e.match_atom()) {
                                                    if left == "tx-sender" {
                                                        self.diagnostics.insert(
                                                            expr.id,
                                                            vec![Diagnostic {
                                                                level: Level::Warning,
                                                                message: UnsafeAuthenticationError.message(),
                                                                spans: vec![expr.span.clone()],
                                                                suggestion: UnsafeAuthenticationError.suggestion(),
                                                            }],
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    

    fn filter_all(&mut self) {
        self.tainted_nodes.clear();
    }

    fn process_annotations(&mut self, span: &Span) {
        self.active_annotation = None;

        for (i, annotation) in self.annotations.iter().enumerate() {
            if annotation.span.start_line == (span.start_line - 1) {
                self.active_annotation = Some(i);
                return;
            } else if annotation.span.start_line >= span.start_line {
                return;
            }
        }
    }

    fn allow_unchecked_data(&self) -> bool {
        if let Some(idx) = self.active_annotation {
            let annotation = &self.annotations[idx];
            return matches!(
                annotation.kind,
                AnnotationKind::Allow(WarningKind::UncheckedData)
            );
        }
        false
    }

    fn allow_unchecked_params(&self) -> bool {
        if let Some(idx) = self.active_annotation {
            let annotation = &self.annotations[idx];
            return matches!(
                annotation.kind,
                AnnotationKind::Allow(WarningKind::UncheckedParams)
            );
        }
        false
    }

    fn apply_filters(&mut self) {
        if let Some(n) = self.active_annotation {
            let params = match &self.annotations[n].kind {
                AnnotationKind::Filter(params) => params,
                &AnnotationKind::FilterAll => {
                    self.filter_all();
                    return;
                }
                _ => return,
            };
            for param in params {
                let source = Node::Symbol(param);
                self.filter_source(&source, false);
            }
        }
    }

    fn generate_diagnostics(&self, expr: &SymbolicExpression) -> Vec<Diagnostic> {
        let mut diagnostics: Vec<Diagnostic> = Vec::new();
        let diagnostic = Diagnostic {
            level: Level::Warning,
            message: "use of potentially unchecked data".to_string(),
            spans: vec![expr.span.clone()],
            suggestion: None,
        };
        diagnostics.push(diagnostic);

        let tainted = &self.tainted_nodes[&Node::Expr(expr.id)];
        let mut source_spans = vec![];
        for source in &tainted.sources {
            let span = self.taint_sources[source].span.clone();
            let pos = source_spans.binary_search(&span).unwrap_or_else(|e| e);
            source_spans.insert(pos, span);
        }
        for span in source_spans {
            let diagnostic = Diagnostic {
                level: Level::Note,
                message: "source of untrusted input here".to_string(),
                spans: vec![span],
                suggestion: None,
            };
            diagnostics.push(diagnostic);
        }
        diagnostics
    }
}

impl<'a> ASTVisitor<'a> for Gecko<'a> {

    fn traverse_define_public(
        &mut self,
        expr: &'a SymbolicExpression,
        name: &'a ClarityName,
        parameters: Option<Vec<TypedVar<'a>>>,
        body: &'a SymbolicExpression,
    ) -> bool {
        self.public_funcs.insert(name);

        self.taint_sources.clear();
        self.tainted_nodes.clear();

        if let Some(params) = parameters {
            for param in params {
                if !is_param_type_excluded_from_checked_requirement(&param) {
                    self.add_taint_source(Node::Symbol(param.name), param.decl_span);
                }
            }
        }
        self.traverse_expr(body)
    }

    fn visit_define_read_only(
        &mut self,
        expr: &'a SymbolicExpression,
        name: &'a ClarityName,
        parameters: Option<Vec<TypedVar<'a>>>,
        body: &'a SymbolicExpression,
    ) -> bool {
        self.public_funcs.insert(name);
        true
    }

    fn traverse_expr(&mut self, expr: &'a SymbolicExpression) -> bool {
        self.process_annotations(&expr.span);
        self.detect_unsafe_authentication(expr);
        if self.allow_unchecked_data() {
            return true;
        }
        let result = match &expr.expr {
            AtomValue(value) => self.visit_atom_value(expr, value),
            Atom(name) => self.visit_atom(expr, name),
            List(exprs) => self.traverse_list(expr, exprs),
            LiteralValue(value) => self.visit_literal_value(expr, value),
            Field(field) => self.visit_field(expr, field),
            TraitReference(name, trait_def) => self.visit_trait_reference(expr, name, trait_def),
        };

        self.apply_filters();
        result
    }


    fn traverse_define_private(
        &mut self,
        expr: &'a SymbolicExpression,
        name: &'a ClarityName,
        parameters: Option<Vec<TypedVar<'a>>>,
        body: &'a SymbolicExpression,
    ) -> bool {
        self.taint_sources.clear();
        self.tainted_nodes.clear();
        let mut info = FunctionInfo {
            unchecked_params: vec![],
            filtered_params: vec![],
        };

        let allow = self.allow_unchecked_params();
        if let Some(params) = &parameters {
            let mut unchecked_params = vec![false; params.len()];
            for (i, param) in params.iter().enumerate() {
                unchecked_params[i] = allow;
                if (allow)
                    && !is_param_type_excluded_from_checked_requirement(param)
                {
                    self.add_taint_source(Node::Symbol(param.name), param.decl_span.clone());
                }
            }
            info.unchecked_params = unchecked_params;
        }
        self.traverse_expr(body);

        self.taint_check(body);

        if let Some(params) = &parameters {
            let mut filtered = vec![false; params.len()];
            if allow {
                for (i, param) in params.iter().enumerate() {
                    if !self.taint_sources.contains_key(&Node::Symbol(param.name)) {
                        filtered[i] = true;
                    }
                }
            }
            info.filtered_params = filtered;
        }

        self.user_funcs.insert(name, info);
        true
    }

    fn traverse_if(
        &mut self,
        expr: &'a SymbolicExpression,
        cond: &'a SymbolicExpression,
        then_expr: &'a SymbolicExpression,
        else_expr: &'a SymbolicExpression,
    ) -> bool {
        self.traverse_expr(cond);
        self.filter_taint(cond, false);

        self.traverse_expr(then_expr);
        self.traverse_expr(else_expr);
        true
    }

    fn traverse_lazy_logical(
        &mut self,
        expr: &'a SymbolicExpression,
        function: NativeFunctions,
        operands: &'a [SymbolicExpression],
    ) -> bool {
        for operand in operands {
            self.traverse_expr(operand);
            self.filter_taint(operand, false);
        }
        true
    }

    fn traverse_let(
        &mut self,
        expr: &'a SymbolicExpression,
        bindings: &HashMap<&'a ClarityName, &'a SymbolicExpression>,
        body: &'a [SymbolicExpression],
    ) -> bool {
        for (name, val) in bindings {
            if !self.traverse_expr(val) {
                return false;
            }
            if let Some(tainted) = self.tainted_nodes.get(&Node::Expr(val.id)) {
                let sources = tainted.sources.clone();
                // If the expression is tainted, add it to the map
                self.add_taint_source_symbol(name, expr.span.clone());
                self.add_tainted_symbol(name, sources);
            }
        }

        for expr in body {
            if !self.traverse_expr(expr) {
                return false;
            }
        }

        if let Some(last_expr) = body.last() {
            if let Some(tainted) = self.tainted_nodes.get(&Node::Expr(last_expr.id)) {
                let sources = tainted.sources.clone();
                self.add_tainted_expr(expr, sources);
            }
        }

        for (name, val) in bindings {
            let node = Node::Symbol(name);
            self.taint_sources.remove(&node);
            self.tainted_nodes.remove(&node);
        }
        true
    }

    fn traverse_begin(
        &mut self,
        expr: &'a SymbolicExpression,
        statements: &'a [SymbolicExpression],
    ) -> bool {
        for stmt in statements {
            if !self.traverse_expr(stmt) {
                return false;
            }
        }

        if let Some(tainted) = &self.tainted_nodes.get(&Node::Expr(expr.id)) {
            let sources = tainted.sources.clone();
            self.add_tainted_expr(expr, sources);
        }

        true
    }

    fn traverse_as_contract(
        &mut self,
        expr: &'a SymbolicExpression,
        inner: &'a SymbolicExpression,
    ) -> bool {
        self.in_as_contract = true;
        let res = self.traverse_expr(inner) && self.visit_as_contract(expr, inner);
        self.in_as_contract = false;
        res
    }

    fn visit_list(&mut self, expr: &'a SymbolicExpression, list: &[SymbolicExpression]) -> bool {
        let mut sources = HashSet::new();

        if let Some((function_name, args)) = list.split_first() {
            if let Some(function_name) = function_name.match_atom() {
                if let Some(define_function) = DefineFunctions::lookup_by_name(function_name) {
                    return true;
                } else if let Some(native_function) = NativeFunctions::lookup_by_name_at_version(
                    function_name,
                    &ClarityVersion::latest(),
                ) {
                    use clarity::vm::functions::NativeFunctions::*;
                    match native_function {
                        Let => return true,
                        Begin => return true,
                        _ => {}
                    }
                }
            }
        }

        for child in list {
            if let Some(tainted) = self.tainted_nodes.get(&Node::Expr(child.id)) {
                sources.extend(tainted.sources.clone());
            }
        }
        if !sources.is_empty() {
            self.add_tainted_expr(expr, sources);
        }
        true
    }

    fn visit_var_set(
        &mut self,
        expr: &'a SymbolicExpression,
        name: &'a ClarityName,
        value: &'a SymbolicExpression,
    ) -> bool {
        self.taint_check(value);
        true
    }

    fn visit_comparison(
        &mut self,
        expr: &'a SymbolicExpression,
        func: NativeFunctions,
        operands: &'a [SymbolicExpression],
    ) -> bool {
        if func != NativeFunctions::Equals {
            return true;
        }

        if (self.trusted_sender
            && ((match_tx_sender(&operands[0])
                && !self.tainted_nodes.contains_key(&Node::Expr(operands[1].id)))
                || (match_tx_sender(&operands[1])
                    && !self.tainted_nodes.contains_key(&Node::Expr(operands[0].id)))))
            || (self.trusted_caller
                && ((match_contract_caller(&operands[0])
                    && !self.tainted_nodes.contains_key(&Node::Expr(operands[1].id)))
                    || (match_contract_caller(&operands[1])
                        && !self.tainted_nodes.contains_key(&Node::Expr(operands[0].id)))))
        {
            let sources = self.taint_sources.keys().cloned().collect();
            self.filter_all();
            self.tainted_nodes
                .insert(Node::Expr(expr.id), TaintedNode { sources });
        }
        true
    }
}

impl AnalysisPass for Gecko<'_> {
    fn run_pass(
        contract_analysis: &mut ContractAnalysis,
        analysis_db: &mut AnalysisDatabase,
        annotations: &Vec<Annotation>
    ) -> AnalysisResult {
        let checker = Gecko::new(annotations);
        checker.run(contract_analysis)
    }
}

fn match_tx_sender(expr: &SymbolicExpression) -> bool {
    if let Some(name) = expr.match_atom() {
        if name.as_str() == "tx-sender" {
            return true;
        }
    }
    false
}

fn match_contract_caller(expr: &SymbolicExpression) -> bool {
    if let Some(name) = expr.match_atom() {
        if name.as_str() == "contract-caller" {
            return true;
        }
    }
    false
}


