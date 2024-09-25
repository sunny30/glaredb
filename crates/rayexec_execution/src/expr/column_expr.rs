use rayexec_bullet::datatype::DataType;
use rayexec_error::{RayexecError, Result};

use crate::logical::binder::bind_context::{BindContext, TableRef};
use std::fmt;

/// Reference to a column in a query.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ColumnExpr {
    /// Scope this column is in.
    pub table_scope: TableRef,
    /// Column index within the table.
    pub column: usize,
}

impl ColumnExpr {
    pub fn datatype(&self, bind_context: &BindContext) -> Result<DataType> {
        let table = bind_context.get_table(self.table_scope)?;
        table
            .column_types
            .get(self.column)
            .cloned()
            .ok_or_else(|| RayexecError::new(format!("Missing column in bind context: {self}")))
    }
}

impl fmt::Display for ColumnExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.table_scope, self.column)
    }
}