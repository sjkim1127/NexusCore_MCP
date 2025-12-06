use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

pub mod common;
pub mod malware;
pub mod game;
pub mod iot;
pub mod firmware;
pub mod wrappers; // Fixed missing wrappers module
pub mod system;   // Added system module

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    async fn execute(&self, args: Value) -> Result<Value>;
}
