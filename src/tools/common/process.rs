use anyhow::Result;
use serde_json::Value;
use crate::tools::Tool;
use async_trait::async_trait;
use crate::engine::frida_handler::FridaHandler;

pub struct InjectFridaScript;

#[async_trait]
impl Tool for InjectFridaScript {
    fn name(&self) -> &str { "inject_frida_script" }
    fn description(&self) -> &str { "Injects a custom Frida script (JavaScript) into a running process. Args: pid (number), script (string)" }
    
    async fn execute(&self, args: Value) -> Result<Value> {
        let pid = args["pid"].as_u64().ok_or(anyhow::anyhow!("Missing pid"))? as u32;
        let script = args["script"].as_str().ok_or(anyhow::anyhow!("Missing script content"))?;
        
        let engine = FridaHandler::new();
        engine.inject_script(pid, script).await?;
        
        Ok(serde_json::json!({ 
            "status": "injected", 
            "pid": pid,
            "script_length": script.len()
        }))
    }
}

pub struct SpawnProcess;

#[async_trait]
impl Tool for SpawnProcess {
    fn name(&self) -> &str { "spawn_process" }
    fn description(&self) -> &str { "Spawns a process in suspended state (Frida) & injects stealth hooks. Args: path, stealth (bool, default true)" }
    
    async fn execute(&self, args: Value) -> Result<Value> {
        let path = args["path"].as_str().ok_or(anyhow::anyhow!("Missing path"))?;
        let stealth = args["stealth"].as_bool().unwrap_or(true);
        
        let engine = FridaHandler::new();
        
        let mut script_content = String::new();
        if stealth {
             script_content = include_str!("../../resources/scripts/stealth_unpacker.js").to_string();
        }

        let pid = engine.spawn_and_instrument(path, &script_content).await?;
        
        Ok(serde_json::json!({ 
            "pid": pid, 
            "status": "spawned_and_hooked",
            "stealth_mode": stealth
        }))
    }
}

pub struct AttachProcess;

#[async_trait]
impl Tool for AttachProcess {
    fn name(&self) -> &str { "attach_process" }
    fn description(&self) -> &str { "Attaches to a running process. Args: pid (number)" }
    
    async fn execute(&self, args: Value) -> Result<Value> {
        let pid = args["pid"].as_u64().ok_or(anyhow::anyhow!("Missing pid"))? as u32;
        let engine = FridaHandler::new();
        let _session = engine.attach_process(pid).await?;
        Ok(serde_json::json!({ "status": "attached", "pid": pid }))
    }
}

pub struct ResumeProcess;

#[async_trait]
impl Tool for ResumeProcess {
    fn name(&self) -> &str { "resume_process" }
    fn description(&self) -> &str { "Resumes a suspended process. Args: pid (number)" }
    
    async fn execute(&self, args: Value) -> Result<Value> {
        let pid = args["pid"].as_u64().ok_or(anyhow::anyhow!("Missing pid"))? as u32;
        let engine = FridaHandler::new();
        engine.resume_process(pid).await?;
        Ok(serde_json::json!({ "status": "resumed", "pid": pid }))
    }
}
