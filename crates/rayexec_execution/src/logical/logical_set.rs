use rayexec_bullet::scalar::OwnedScalarValue;

use crate::config::vars::SessionVar;
use crate::explain::explainable::{ExplainConfig, ExplainEntry, Explainable};

use super::binder::bind_context::TableRef;
use super::operator::{LogicalNode, Node};

#[derive(Debug, Clone, PartialEq)]
pub struct LogicalSetVar {
    pub name: String,
    pub value: OwnedScalarValue,
}

impl Explainable for LogicalSetVar {
    fn explain_entry(&self, _conf: ExplainConfig) -> ExplainEntry {
        ExplainEntry::new("Set")
    }
}

impl LogicalNode for Node<LogicalSetVar> {
    fn get_output_table_refs(&self) -> Vec<TableRef> {
        Vec::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableOrAll {
    Variable(SessionVar),
    All,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LogicalResetVar {
    pub var: VariableOrAll,
}

impl Explainable for Node<LogicalResetVar> {
    fn explain_entry(&self, _conf: ExplainConfig) -> ExplainEntry {
        ExplainEntry::new("Reset")
    }
}

impl LogicalNode for Node<LogicalResetVar> {
    fn get_output_table_refs(&self) -> Vec<TableRef> {
        Vec::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LogicalShowVar {
    pub var: SessionVar,
}

impl Explainable for LogicalShowVar {
    fn explain_entry(&self, _conf: ExplainConfig) -> ExplainEntry {
        ExplainEntry::new("Show")
    }
}

impl LogicalNode for Node<LogicalShowVar> {
    fn get_output_table_refs(&self) -> Vec<TableRef> {
        Vec::new()
    }
}