use crate::logical::{
    binder::{bind_context::BindContext, bind_query::BoundQuery},
    logical_scan::{LogicalScan, ScanSource},
    operator::{LocationRequirement, LogicalOperator, Node},
    planner::plan_select::SelectPlanner,
};
use rayexec_error::Result;

use super::plan_setop::SetOpPlanner;

#[derive(Debug)]
pub struct QueryPlanner;

impl QueryPlanner {
    pub fn plan(
        &self,
        bind_context: &mut BindContext,
        query: BoundQuery,
    ) -> Result<LogicalOperator> {
        match query {
            BoundQuery::Select(select) => SelectPlanner.plan(bind_context, select),
            BoundQuery::Setop(setop) => SetOpPlanner.plan(bind_context, setop),
            BoundQuery::Values(values) => {
                let table = bind_context.get_table(values.expressions_table)?;

                Ok(LogicalOperator::Scan(Node {
                    node: LogicalScan {
                        table_ref: values.expressions_table,
                        types: table.column_types.clone(),
                        names: table.column_names.clone(),
                        projection: (0..table.num_columns()).collect(),
                        source: ScanSource::ExpressionList { rows: values.rows },
                    },
                    location: LocationRequirement::Any,
                    children: Vec::new(),
                }))
            }
        }
    }
}
