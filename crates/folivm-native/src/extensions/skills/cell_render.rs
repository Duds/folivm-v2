use async_trait::async_trait;
use crate::extensions::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum CellRenderResult {
    Svg { content: String },
    Html { content: String, height: f32 },
    Text { content: Value }, // Inline[] from core
    Error { message: String },
}

#[async_trait]
pub trait CellRenderSkill: Send + Sync {
    async fn render(&self, cell_type: &str, attributes: Value, source: Option<&str>) -> Result<CellRenderResult>;
}
