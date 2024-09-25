use crate::{
    explain::explainable::{ExplainConfig, ExplainEntry, Explainable},
    expr::{
        comparison_expr::{ComparisonExpr, ComparisonOperator},
        Expression,
    },
};
use std::fmt;

use super::{
    binder::bind_context::{MaterializationRef, TableRef},
    operator::{LogicalNode, Node},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinType {
    /// Standard LEFT join.
    Left,
    /// Standard RIGHT join.
    Right,
    /// Standard INNER join.
    Inner,
    /// Standard full/outer join.
    Full,
    /// Left semi join.
    Semi,
    /// Left anti join.
    Anti,
    /// A left join that emits all rows on the left side joined with a column
    /// that indicates if there was a join partner on the right.
    ///
    /// These essentially exposes the left visit bitmaps to other operators.
    ///
    /// Idea taken from duckdb.
    LeftMark {
        /// The table ref to use in logical planning the reference the visit
        /// bitmap output.
        ///
        /// This should have a single column of type bool.
        table_ref: TableRef,
    },
}

impl JoinType {
    /// Helper for determining the output refs for a given node type.
    fn output_refs<T>(self, node: &Node<T>) -> Vec<TableRef> {
        if let JoinType::LeftMark { table_ref } = self {
            let mut refs = node
                .children
                .first()
                .map(|c| c.get_output_table_refs())
                .unwrap_or_default();
            refs.push(table_ref);
            refs
        } else {
            node.get_children_table_refs()
        }
    }
}

impl fmt::Display for JoinType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inner => write!(f, "INNER"),
            Self::Left => write!(f, "LEFT"),
            Self::Right => write!(f, "RIGHT"),
            Self::Full => write!(f, "FULL"),
            Self::Semi => write!(f, "SEMI"),
            Self::Anti => write!(f, "ANTI"),
            Self::LeftMark { table_ref } => write!(f, "LEFT MARK (ref = {table_ref})"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComparisonCondition {
    /// Expression containing column references from the left side.
    pub left: Expression,
    /// Expression containing column references from the right side.
    pub right: Expression,
    /// Comparision operator.
    pub op: ComparisonOperator,
}

impl ComparisonCondition {
    pub fn into_expression(self) -> Expression {
        Expression::Comparison(ComparisonExpr {
            left: Box::new(self.left),
            right: Box::new(self.right),
            op: self.op,
        })
    }

    pub fn flip_sides(&mut self) {
        self.op = self.op.flip();
        std::mem::swap(&mut self.left, &mut self.right);
    }
}

impl fmt::Display for ComparisonCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.left, self.op, self.right)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalComparisonJoin {
    pub join_type: JoinType,
    pub conditions: Vec<ComparisonCondition>,
}

impl Explainable for LogicalComparisonJoin {
    fn explain_entry(&self, _conf: ExplainConfig) -> ExplainEntry {
        ExplainEntry::new("ComparisonJoin")
            .with_values("conditions", &self.conditions)
            .with_value("join_type", self.join_type)
    }
}

impl LogicalNode for Node<LogicalComparisonJoin> {
    fn get_output_table_refs(&self) -> Vec<TableRef> {
        self.node.join_type.output_refs(self)
    }
}

/// A magic join behaves the same as a comparison join, is only used to
/// demarcate a join node that was created as a result of subquery
/// decorrelation.
///
/// A separate type allows us to more easily run certain optimization steps
/// since we'll have a bit more information.
///
/// The left child will be a materialization scan, and the right child will be a
/// normal operator tree with some number of "magic" materialization scans that
/// read deduplicated values from a materialized plan that's referenced on the
/// left.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalMagicJoin {
    /// The materialization reference for the left child.
    ///
    /// Any "magic" materialization scan we see on the right we can assume was
    /// part of the same decorrelation step that created this node.
    pub mat_ref: MaterializationRef,
    /// The join type, behaves the same as a comparison join.
    pub join_type: JoinType,
    /// Conditions, same as comparison join.
    pub conditions: Vec<ComparisonCondition>,
}

impl Explainable for LogicalMagicJoin {
    fn explain_entry(&self, _conf: ExplainConfig) -> ExplainEntry {
        ExplainEntry::new("MagicJoin")
            .with_values("conditions", &self.conditions)
            .with_value("join_type", self.join_type)
    }
}

impl LogicalNode for Node<LogicalMagicJoin> {
    fn get_output_table_refs(&self) -> Vec<TableRef> {
        self.node.join_type.output_refs(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalArbitraryJoin {
    pub join_type: JoinType,
    pub condition: Expression,
}

impl Explainable for LogicalArbitraryJoin {
    fn explain_entry(&self, _conf: ExplainConfig) -> ExplainEntry {
        ExplainEntry::new("ArbitraryJoin")
            .with_value("join_type", self.join_type)
            .with_value("condition", &self.condition)
    }
}

impl LogicalNode for Node<LogicalArbitraryJoin> {
    fn get_output_table_refs(&self) -> Vec<TableRef> {
        self.node.join_type.output_refs(self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LogicalCrossJoin;

impl Explainable for LogicalCrossJoin {
    fn explain_entry(&self, _conf: ExplainConfig) -> ExplainEntry {
        ExplainEntry::new("CrossJoin")
    }
}

impl LogicalNode for Node<LogicalCrossJoin> {
    fn get_output_table_refs(&self) -> Vec<TableRef> {
        self.get_children_table_refs()
    }
}

#[cfg(test)]
mod tests {
    use rayexec_bullet::scalar::ScalarValue;

    use crate::expr::literal_expr::LiteralExpr;

    use super::*;

    #[test]
    fn flip_comparison() {
        let a = Expression::Literal(LiteralExpr {
            literal: ScalarValue::Int8(1),
        });
        let b = Expression::Literal(LiteralExpr {
            literal: ScalarValue::Int8(2),
        });

        // (original, flipped)
        let tests = [
            (
                ComparisonCondition {
                    left: a.clone(),
                    right: b.clone(),
                    op: ComparisonOperator::Lt,
                },
                ComparisonCondition {
                    left: b.clone(),
                    right: a.clone(),
                    op: ComparisonOperator::Gt,
                },
            ),
            (
                ComparisonCondition {
                    left: a.clone(),
                    right: b.clone(),
                    op: ComparisonOperator::LtEq,
                },
                ComparisonCondition {
                    left: b.clone(),
                    right: a.clone(),
                    op: ComparisonOperator::GtEq,
                },
            ),
        ];

        for (mut original, flipped) in tests {
            original.flip_sides();
            assert_eq!(flipped, original);
        }
    }
}