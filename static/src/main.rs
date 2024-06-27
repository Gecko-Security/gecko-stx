#![allow(unused_variables)]

use crate::analysis::{self, AnalysisPass, AnalysisResult};
use clarity::vm::analysis::analysis_db::AnalysisDatabase;
use clarity::vm::analysis::types::ContractAnalysis;
use clarity::vm::representations::Span;
use clarity::vm::representations::SymbolicExpressionType::*;
use clarity::vm::types::TypeSignature;
use clarity::vm::{ClarityName, ClarityVersion, SymbolicExpression};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::collections::HashSet;
use tree_sitter::{Language, Node, Parser, Tree, TreeCursor};
use std::fmt;
use std::sync::Once;

static INIT: Once = Once::new();
static mut CLARITY: Option<Language> = None;
pub struct CheckError;
const TIMES: usize = 3;

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

#[derive(Debug, Clone)]
struct Finding {
    marked_nodes: (Node<'static>, Node<'static>),
    visitor: String,
    source: String,
    msg: String,
    help_msg: Option<String>,
    footnote: Option<String>,
    location: Option<Location>,
}

struct Visitor {
    source: Option<String>,
    msg: String,
    help_msg: Option<String>,
    footnote: Option<String>,
    findings: Vec<Finding>,
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

    fn new() -> Self {
        Visitor {
            source: None,
            msg: String::new(),
            help_msg: None,
            footnote: None,
            findings: Vec::new(),
        }
    }

    fn add_source(&mut self, src: String) {
        self.source = Some(src);
    }

    fn visit_node(&self, _node: &Node, _round: usize) {
        eprintln!("visit_node not implemented");
        std::process::exit(1);
    }

    fn get_contract_code_lines(&self) -> Vec<String> {
        self.source
            .as_ref()
            .unwrap()
            .lines()
            .map(|line| line.to_string())
            .collect()
    }

    fn add_finding(&mut self, node: Node, specific_node: Node) -> Vec<Finding> {
        // Assume `pretty_print_warn` is implemented elsewhere
        // pretty_print_warn(self, &node, &specific_node, &self.msg, &self.help_msg, &self.footnote);

        let parent = node.parent().unwrap();
        let line_number = parent.start_position().row + 1;
        let line_code = self.get_contract_code_lines()[line_number - 1].clone();
        let location = Location {
            lineno: parent.start_position().row + 1,
            start_tabs: line_code.chars().filter(|&c| c == '\t').count() + 1,
            span: (node.start_position().column, node.end_position().column),
            line_code: line_code.clone(),
        };
        let finding = Finding {
            marked_nodes: (node, specific_node),
            visitor: self.name().to_string(),
            source: self.source.clone().unwrap(),
            msg: self.msg.clone(),
            help_msg: self.help_msg.clone(),
            footnote: self.footnote.clone(),
            location: Some(location),
        };
        self.findings.push(finding);
        self.findings.clone()
    }

    fn get_findings(&self) -> Vec<Finding> {
        self.findings.clone()
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
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

struct NodeIterator<'a> {
    root_node: Node<'a>,
    cursor: TreeCursor<'a>,
    visited: HashSet<Node<'a>>,
}

impl<'a> NodeIterator<'a> {
    fn new(node: Node<'a>) -> Self {
        let mut cursor = node.walk();
        while cursor.goto_first_child() {}

        NodeIterator {
            root_node: node,
            cursor,
            visited: HashSet::new(),
        }
    }

    fn next(&mut self) -> Option<Node<'a>> {
        loop {
            let node = self.node();

            if !self.visited.contains(&node) {
                if self.cursor.goto_first_child() {
                    continue;
                }
                self.visited.insert(node.clone());
                return Some(node);
            }

            if self.cursor.goto_next_sibling() {
                while self.cursor.goto_first_child() {}
            } else {
                if !self.cursor.goto_parent() {
                    return None;
                }
                let parent_node = self.cursor.node();
                self.visited.insert(parent_node.clone());
                return Some(parent_node);
            }
        }
    }

    fn node(&self) -> Node<'a> {
        self.cursor.node()
    }
}

impl<'a> Iterator for NodeIterator<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

struct LinterRunner<'a> {
    source: String,
    tree: Tree,
    root_node: Node<'a>,
    iterator: NodeIterator<'a>,
    lints: Vec<Box<dyn VisitorTrait + 'a>>,
    round_number: usize,
}

impl<'a> LinterRunner<'a> {
    fn new(source: String) -> Self {
        unsafe {
            INIT.call_once(|| {
                CLARITY = Some(Language::new("tree-sitter-clarity"));
            });

            let parser = Parser::new(CLARITY.as_ref().unwrap());
            let tree = parser.parse(source.as_bytes(), None).unwrap();
            let root_node = tree.root_node();
            let iterator = NodeIterator::new(root_node);

            LinterRunner {
                source,
                tree,
                root_node,
                iterator,
                lints: Vec::new(),
                round_number: 0,
            }
        }
    }

    fn run_lints(&mut self, node: &Node) {
        for lint in &self.lints {
            lint.visit_node(node, self.round_number);
        }
    }

    fn add_lint(&mut self, lint: Box<dyn VisitorTrait + 'a>) {
        self.lints.push(lint);
    }

    fn add_lints(&mut self, lint_classes: Vec<Box<dyn VisitorTrait + 'a>>) {
        for lint_class in lint_classes {
            let mut lint = lint_class;
            lint.add_source(self.source.clone());
            self.lints.push(lint);
        }
    }

    fn reset_cursor(&mut self) {
        self.iterator = NodeIterator::new(self.root_node);
    }

    fn run(&mut self) -> Vec<Finding> {
        for _ in 0..TIMES {
            self.round_number += 1;
            while let Some(v) = self.iterator.next() {
                self.run_lints(&v);
            }
            self.reset_cursor();
        }

        self.lints
            .iter()
            .flat_map(|lint| lint.get_findings())
            .collect()
    }
}

trait VisitorTrait {
    fn visit_node(&self, node: &Node, round: usize);
    fn add_source(&mut self, src: String);
    fn get_findings(&self) -> Vec<Finding>;
}

impl VisitorTrait for Visitor {
    fn visit_node(&self, node: &Node, round: usize) {
        Visitor::visit_node(self, node, round);
    }

    fn add_source(&mut self, src: String) {
        Visitor::add_source(self, src);
    }

    fn get_findings(&self) -> Vec<Finding> {
        Visitor::get_findings(self)
    }
}


