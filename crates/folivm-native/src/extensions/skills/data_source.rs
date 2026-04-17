use async_trait::async_trait;
use crate::extensions::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataField {
    pub name: String,
    pub label: String,
    pub description: Option<String>,
    pub field_type: String, // "text" | "number" | "date"
}

#[async_trait]
pub trait DataSourceSkill: Send + Sync {
    async fn fields(&self) -> Result<Vec<DataField>>;
    async fn resolve(&self, field: String, document_id: String) -> Result<Option<String>>;
}
