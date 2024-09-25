use rayexec_bullet::field::Schema;
use rayexec_io::location::FileLocation;

use crate::explain::explainable::{ExplainConfig, ExplainEntry, Explainable};
use crate::functions::copy::CopyToFunction;

use super::binder::bind_context::TableRef;
use super::operator::{LogicalNode, Node};

#[derive(Debug, Clone, PartialEq)]
pub struct LogicalCopyTo {
    /// Schema of input operator.
    ///
    /// Stored on this operator since the copy to sinks may need field names
    /// (e.g. writing out a header in csv).
    pub source_schema: Schema,
    pub location: FileLocation,
    pub copy_to: Box<dyn CopyToFunction>,
}

impl Explainable for LogicalCopyTo {
    fn explain_entry(&self, _conf: ExplainConfig) -> ExplainEntry {
        ExplainEntry::new("CopyTo")
    }
}

impl LogicalNode for Node<LogicalCopyTo> {
    fn get_output_table_refs(&self) -> Vec<TableRef> {
        Vec::new()
    }
}