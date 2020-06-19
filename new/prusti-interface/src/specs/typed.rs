use prusti_specs::specifications::common;
use prusti_specs::specifications::json;
use rustc_hir::BodyId;
use std::collections::HashMap;

pub use common::{ExpressionId, SpecType, SpecificationId};

// TODO: Replace with a proper one.
type Arg = ();

/// A specification that has no types associated with it.
pub type Specification = common::Specification<ExpressionId, BodyId, Arg>;
/// A set of untyped specifications associated with a single element.
pub type SpecificationSet = common::SpecificationSet<ExpressionId, BodyId, Arg>;
/// A set of untyped specifications associated with a loop.
pub type LoopSpecification = common::LoopSpecification<ExpressionId, BodyId, Arg>;
/// A set of untyped specifications associated with a procedure.
pub type ProcedureSpecification = common::ProcedureSpecification<ExpressionId, BodyId, Arg>;
/// A map of untyped specifications for a specific crate.
pub type SpecificationMap = HashMap<common::SpecificationId, SpecificationSet>;
/// An assertion that has no types associated with it.
pub type Assertion = common::Assertion<ExpressionId, BodyId, Arg>;
/// An assertion kind that has no types associated with it.
pub type AssertionKind = common::AssertionKind<ExpressionId, BodyId, Arg>;
/// An expression that has no types associated with it.
pub type Expression = common::Expression<ExpressionId, BodyId>;
/// A trigger set that has not types associated with it.
pub type TriggerSet = common::TriggerSet<ExpressionId, BodyId>;

pub trait StructuralToTyped<Target> {
    fn to_typed(self, typed_expressions: &HashMap<String, rustc_hir::BodyId>) -> Target;
}

impl StructuralToTyped<Expression> for json::Expression {
    fn to_typed(self, typed_expressions: &HashMap<String, rustc_hir::BodyId>) -> Expression {
        let body_id = typed_expressions[&format!("{}_{}", self.spec_id, self.expr_id)];
        Expression {
            spec_id: self.spec_id,
            id: self.expr_id,
            expr: body_id,
        }
    }
}

impl StructuralToTyped<AssertionKind> for json::AssertionKind {
    fn to_typed(self, typed_expressions: &HashMap<String, rustc_hir::BodyId>) -> AssertionKind {
        use json::AssertionKind::*;
        match self {
            Expr(expr) => AssertionKind::Expr(expr.to_typed(typed_expressions)),
        }
    }
}

impl StructuralToTyped<Assertion> for json::Assertion {
    fn to_typed(self, typed_expressions: &HashMap<String, rustc_hir::BodyId>) -> Assertion {
        Assertion {
            kind: box self.kind.to_typed(typed_expressions),
        }
    }
}