use async_trait::async_trait;
use crate::extensions::error::Result;
use serde_json::Value;

#[async_trait]
pub trait PanelSkill: Send + Sync {
    async fn on_message(&self, panel_id: &str, message: Value) -> Result<Option<Value>>;
    async fn on_visible(&self, panel_id: &str) -> Result<()>;
    async fn on_hidden(&self, panel_id: &str) -> Result<()>;
}
