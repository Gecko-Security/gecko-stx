#![allow(unused_variables)]

use crate::analysis::ast_visitor::{traverse, ASTVisitor};
use crate::repl::DEFAULT_EPOCH;
use clarity::types::StacksEpochId;
pub use clarity::vm::analysis::types::ContractAnalysis;
use clarity::vm::analysis::{CheckErrors, CheckResult};
use clarity::vm::ast::ContractAST;
use clarity::vm::representations::{SymbolicExpression, TraitDefinition};
use clarity::vm::types::signatures::CallableSubtype;
use clarity::vm::types::{
    FunctionSignature, PrincipalData, QualifiedContractIdentifier, SequenceSubtype,
    TraitIdentifier, TypeSignature, Value,
};
use clarity::vm::{ClarityName, ClarityVersion, SymbolicExpressionType};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};

use super::ast_visitor::TypedVar;

lazy_static! {
    pub static ref DEFAULT_NAME: ClarityName = ClarityName::from("placeholder");
}

pub struct ASTDependencyDetector<'a> {
    dependencies: BTreeMap<QualifiedContractIdentifier, DependencySet>,
    current_clarity_version: Option<&'a ClarityVersion>,
    current_contract: Option<&'a QualifiedContractIdentifier>,
    defined_functions:
        BTreeMap<(&'a QualifiedContractIdentifier, &'a ClarityName), Vec<TypeSignature>>,
    defined_traits: BTreeMap<
        (&'a QualifiedContractIdentifier, &'a ClarityName),
        BTreeMap<ClarityName, FunctionSignature>,
    >,
    defined_contract_constants: BTreeMap<
        (&'a QualifiedContractIdentifier, &'a ClarityName),
        &'a QualifiedContractIdentifier,
    >,
    pending_function_checks: BTreeMap<
        (&'a QualifiedContractIdentifier, &'a ClarityName),
        Vec<(&'a QualifiedContractIdentifier, &'a [SymbolicExpression])>,
    >,
    pending_trait_checks: BTreeMap<
        &'a TraitIdentifier,
        Vec<(
            &'a QualifiedContractIdentifier,
            &'a ClarityName,
            &'a [SymbolicExpression],
        )>,
    >,
    params: Option<Vec<TypedVar<'a>>>,
    top_level: bool,
    preloaded: &'a BTreeMap<QualifiedContractIdentifier, (ClarityVersion, ContractAST)>,
}

#[derive(Clone, Debug, Eq)]
pub struct Dependency {
    pub contract_id: QualifiedContractIdentifier,
    pub required_before_publish: bool,
}

impl PartialEq for Dependency {
    fn eq(&self, other: &Self) -> bool {
        self.contract_id == other.contract_id
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for Dependency {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.contract_id.partial_cmp(&other.contract_id)
    }
}

impl Ord for Dependency {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.contract_id.cmp(&other.contract_id)
    }
}

fn deep_check_callee_type(
    arg_type: &TypeSignature,
    expr: &SymbolicExpression,
    dependencies: &mut BTreeSet<QualifiedContractIdentifier>,
) {
    match arg_type {
        TypeSignature::CallableType(CallableSubtype::Trait(_))
        | TypeSignature::TraitReferenceType(_) => {
            if let Some(Value::Principal(PrincipalData::Contract(contract))) =
                expr.match_literal_value()
            {
                dependencies.insert(contract.clone());
            }
        }
        TypeSignature::OptionalType(inner_type) => {
            if let Some(expr) = expr.match_list().and_then(|l| l.get(1)) {
                deep_check_callee_type(inner_type, expr, dependencies);
            }
        }
        TypeSignature::ResponseType(inner_type) => {
            if let Some(expr) = expr.match_list().and_then(|l| l.get(1)) {
                deep_check_callee_type(&inner_type.0, expr, dependencies);
            }
            if let Some(expr) = expr.match_list().and_then(|l| l.get(2)) {
                deep_check_callee_type(&inner_type.1, expr, dependencies);
            }
        }
        TypeSignature::TupleType(inner_type) => {
            let type_map = inner_type.get_type_map();
            if let Some(tuple) = expr.match_list() {
                for key_value in tuple.iter().skip(1) {
                    if let Some((arg_type, expr)) = key_value
                        .match_list()
                        .and_then(|kv| Some((type_map.get(kv.first()?.match_atom()?)?, kv.get(1)?)))
                    {
                        deep_check_callee_type(arg_type, expr, dependencies);
                    }
                }
            }
        }
        TypeSignature::SequenceType(SequenceSubtype::ListType(inner_type)) => {
            let item_type = inner_type.get_list_item_type();
            if let Some(list) = expr.match_list() {
                for item in list.iter().skip(1) {
                    deep_check_callee_type(item_type, item, dependencies);
                }
            }
        }
        _ => (),
    }
}

#[derive(Debug, Clone, Default)]
pub struct DependencySet {
    pub set: BTreeSet<Dependency>,
}

impl DependencySet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_dependency(
        &mut self,
        contract_id: QualifiedContractIdentifier,
        required_before_publish: bool,
    ) {
        let dep = Dependency {
            contract_id,
            required_before_publish,
        };
        if required_before_publish {
            self.set.remove(&dep);
        }

        self.set.insert(dep);
    }

    pub fn has_dependency(&self, contract_id: &QualifiedContractIdentifier) -> Option<bool> {
        self.set
            .get(&Dependency {
                contract_id: contract_id.clone(),
                required_before_publish: false,
            })
            .map(|dep| dep.required_before_publish)
    }
}

impl<'a> ASTDependencyDetector<'a> {
    pub fn detect_dependencies(
        contract_asts: &'a BTreeMap<QualifiedContractIdentifier, (ClarityVersion, ContractAST)>,
        preloaded: &'a BTreeMap<QualifiedContractIdentifier, (ClarityVersion, ContractAST)>,
    ) -> Result<
        BTreeMap<QualifiedContractIdentifier, DependencySet>,
        (
            BTreeMap<QualifiedContractIdentifier, DependencySet>,
            Vec<QualifiedContractIdentifier>,
        ),
    > {
        let mut detector = Self {
            dependencies: BTreeMap::new(),
            current_clarity_version: None,
            current_contract: None,
            defined_functions: BTreeMap::new(),
            defined_traits: BTreeMap::new(),
            defined_contract_constants: BTreeMap::new(),
            pending_function_checks: BTreeMap::new(),
            pending_trait_checks: BTreeMap::new(),
            params: None,
            top_level: true,
            preloaded,
        };

        let mut preloaded_visitor = PreloadedVisitor {
            detector: &mut detector,
            current_clarity_version: None,
            current_contract: None,
        };

        for (contract_identifier, (clarity_version, ast)) in preloaded {
            preloaded_visitor.current_clarity_version = Some(clarity_version);
            preloaded_visitor.current_contract = Some(contract_identifier);
            traverse(&mut preloaded_visitor, &ast.expressions);
        }

        for (contract_identifier, (clarity_version, ast)) in contract_asts {
            detector
                .dependencies
                .insert(contract_identifier.clone(), DependencySet::new());
            detector.current_clarity_version = Some(clarity_version);
            detector.current_contract = Some(contract_identifier);
            traverse(&mut detector, &ast.expressions);
        }

        let mut unresolved: Vec<QualifiedContractIdentifier> = detector
            .pending_function_checks
            .into_keys()
            .map(|(contract_id, _)| contract_id.clone())
            .collect();
        unresolved.append(
            &mut detector
                .pending_trait_checks
                .into_keys()
                .map(|trait_id| trait_id.contract_identifier.clone())
                .collect(),
        );
        if unresolved.is_empty() {
            Ok(detector.dependencies)
        } else {
            Err((detector.dependencies, unresolved))
        }
    }

    pub fn order_contracts<'deps>(
        dependencies: &'deps BTreeMap<QualifiedContractIdentifier, DependencySet>,
        contract_epochs: &HashMap<QualifiedContractIdentifier, StacksEpochId>,
    ) -> CheckResult<Vec<&'deps QualifiedContractIdentifier>> {
        let mut lookup = BTreeMap::new();
        let mut reverse_lookup = Vec::new();

        if dependencies.is_empty() {
            return Ok(vec![]);
        }

        for (index, (contract, _)) in dependencies.iter().enumerate() {
            lookup.insert(contract, index);
            reverse_lookup.push(contract);
        }

        let mut graph = Graph::new();
        for (contract, contract_dependencies) in dependencies {
            let contract_id = lookup.get(contract).unwrap();
            let contract_epoch = contract_epochs
                .get(contract)
                .unwrap_or(&StacksEpochId::Epoch20);
            graph.add_node(*contract_id);
            for dep in contract_dependencies.iter() {
                let dep_epoch = contract_epochs
                    .get(&dep.contract_id)
                    .unwrap_or(&StacksEpochId::Epoch20);
                if contract_epoch < dep_epoch {
                    return Err(CheckErrors::NoSuchContract(dep.contract_id.to_string()).into());
                }
                let dep_id = match lookup.get(&dep.contract_id) {
                    Some(id) => id,
                    None => {
                        continue;
                    }
                };
                graph.add_directed_edge(*contract_id, *dep_id);
            }
        }

        let mut walker = GraphWalker::new();
        let sorted_indexes = walker.get_sorted_dependencies(&graph);

        let cyclic_deps = walker.get_cycling_dependencies(&graph, &sorted_indexes);
        if let Some(deps) = cyclic_deps {
            let mut contracts = vec![];
            for index in deps.iter() {
                let contract = reverse_lookup[*index];
                contracts.push(contract.name.to_string());
            }
            return Err(CheckErrors::CircularReference(contracts).into());
        }

        Ok(sorted_indexes
            .iter()
            .map(|index| reverse_lookup[*index])
            .collect())
    }

    fn add_dependency(
        &mut self,
        from: &QualifiedContractIdentifier,
        to: &QualifiedContractIdentifier,
    ) {
        if self.preloaded.contains_key(from) {
            return;
        }

        if to.name.starts_with("__") {
            return;
        }

        if let Some(set) = self.dependencies.get_mut(from) {
            set.add_dependency(to.clone(), self.top_level);
        } else {
            let mut set = DependencySet::new();
            set.add_dependency(to.clone(), self.top_level);
            self.dependencies.insert(from.clone(), set);
        }
    }

    fn add_defined_function(
        &mut self,
        contract_identifier: &'a QualifiedContractIdentifier,
        name: &'a ClarityName,
        param_types: Vec<TypeSignature>,
    ) {
        if let Some(pending) = self
            .pending_function_checks
            .remove(&(contract_identifier, name))
        {
            for (caller, args) in pending {
                for dependency in self.check_callee_type(&param_types, args) {
                    self.add_dependency(caller, &dependency);
                }
            }
        }

        self.defined_functions
            .insert((contract_identifier, name), param_types);
    }

    fn add_pending_function_check(
        &mut self,
        caller: &'a QualifiedContractIdentifier,
        callee: (&'a QualifiedContractIdentifier, &'a ClarityName),
        args: &'a [SymbolicExpression],
    ) {
        if let Some(list) = self.pending_function_checks.get_mut(&callee) {
            list.push((caller, args));
        } else {
            self.pending_function_checks
                .insert(callee, vec![(caller, args)]);
        }
    }

    fn add_defined_trait(
        &mut self,
        contract_identifier: &'a QualifiedContractIdentifier,
        name: &'a ClarityName,
        trait_definition: BTreeMap<ClarityName, FunctionSignature>,
    ) {
        if let Some(pending) = self.pending_trait_checks.remove(&TraitIdentifier {
            name: name.clone(),
            contract_identifier: contract_identifier.clone(),
        }) {
            for (caller, function, args) in pending {
                for dependency in self.check_trait_dependencies(&trait_definition, function, args) {
                    self.add_dependency(caller, &dependency);
                }
            }
        }

        self.defined_traits
            .insert((contract_identifier, name), trait_definition);
    }

    fn add_pending_trait_check(
        &mut self,
        caller: &'a QualifiedContractIdentifier,
        callee: &'a TraitIdentifier,
        function: &'a ClarityName,
        args: &'a [SymbolicExpression],
    ) {
        if let Some(list) = self.pending_trait_checks.get_mut(callee) {
            list.push((caller, function, args));
        } else {
            self.pending_trait_checks
                .insert(callee, vec![(caller, function, args)]);
        }
    }

    fn check_callee_type(
        &self,
        arg_types: &[TypeSignature],
        args: &'a [SymbolicExpression],
    ) -> BTreeSet<QualifiedContractIdentifier> {
        let mut dependencies = BTreeSet::new();
        for (i, arg_type) in arg_types.iter().enumerate() {
            if let Some(expr) = args.get(i) {
                deep_check_callee_type(arg_type, expr, &mut dependencies);
            }
        }
        dependencies
    }

    fn check_trait_dependencies(
        &self,
        trait_definition: &BTreeMap<ClarityName, FunctionSignature>,
        function_name: &ClarityName,
        args: &'a [SymbolicExpression],
    ) -> BTreeSet<QualifiedContractIdentifier> {
        let function_signature = match trait_definition.get(function_name) {
            Some(signature) => signature,
            None => return BTreeSet::new(),
        };
        self.check_callee_type(&function_signature.args, args)
    }
    fn get_param_trait(&self, name: &ClarityName) -> Option<&'a TraitIdentifier> {
        let params = match &self.params {
            None => return None,
            Some(params) => params,
        };
        for param in params {
            if param.name == name {
                if let SymbolicExpressionType::TraitReference(_, trait_def) = &param.type_expr.expr
                {
                    return match trait_def {
                        TraitDefinition::Defined(identifier) => Some(identifier),
                        TraitDefinition::Imported(identifier) => Some(identifier),
                    };
                } else {
                    return None;
                }
            }
        }
        None
    }

    fn get_contract_constant(
        &self,
        name: &'a ClarityName,
    ) -> Option<&'a QualifiedContractIdentifier> {
        self.defined_contract_constants
            .get(&(self.current_contract.unwrap(), name))
            .copied()
    }
}

struct Graph {
    pub adjacency_list: Vec<Vec<usize>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            adjacency_list: Vec::new(),
        }
    }

    fn add_node(&mut self, _expr_index: usize) {
        self.adjacency_list.push(vec![]);
    }

    fn add_directed_edge(&mut self, src_expr_index: usize, dst_expr_index: usize) {
        let list = self.adjacency_list.get_mut(src_expr_index).unwrap();
        list.push(dst_expr_index);
    }

    fn get_node_descendants(&self, expr_index: usize) -> Vec<usize> {
        self.adjacency_list[expr_index].clone()
    }

    fn has_node_descendants(&self, expr_index: usize) -> bool {
        !self.adjacency_list[expr_index].is_empty()
    }

    fn nodes_count(&self) -> usize {
        self.adjacency_list.len()
    }
}

struct GraphWalker {
    seen: HashSet<usize>,
}

impl GraphWalker {
    fn new() -> Self {
        Self {
            seen: HashSet::new(),
        }
    }

    fn sort_dependencies_recursion(
        &mut self,
        tle_index: usize,
        graph: &Graph,
        branch: &mut Vec<usize>,
    ) {

        self.seen.insert(tle_index);
        if let Some(list) = graph.adjacency_list.get(tle_index) {
            for neighbor in list.iter() {
                self.sort_dependencies_recursion(*neighbor, graph, branch);
            }
        }
        branch.push(tle_index);
    }

    fn get_cycling_dependencies(
        &mut self,
        graph: &Graph,
        sorted_indexes: &[usize],
    ) -> Option<Vec<usize>> {
        let mut tainted: HashSet<usize> = HashSet::new();

        if tainted.len() == sorted_indexes.len() {
            return None;
        }

        let nodes = HashSet::from_iter(sorted_indexes.iter().cloned());
        let deps = nodes.difference(&tainted).copied().collect();
        Some(deps)
    }
}
