use async_trait::async_trait;
use crate::extensions::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportDocument {
    pub metadata: Value,
    pub blocks: Vec<Value>,
}

#[async_trait]
pub trait ExportHookSkill: Send + Sync {
    async fn on_before(&self, format: &str, document: ExportDocument) -> Result<ExportDocument>;
}
