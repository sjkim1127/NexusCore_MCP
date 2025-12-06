use anyhow::Result;
use frida::{DeviceManager, RemoteDeviceOptions};

pub struct FridaHandler;

impl FridaHandler {
    pub fn new() -> Self {
        Self {}
    }

    /// Spawns a process in suspended state, injects script, and resumes
    pub async fn spawn_and_instrument(&self, path: &str, script_content: &str) -> Result<u32> {
        let device_manager = DeviceManager::new(frida::LocalDevice::new());
        let device = device_manager.get_local_device()?;
        
        // 1. Spawn suspended
        let pid = device.spawn(path, &frida::SpawnOptions::default())?;
        
        // 2. Attach
        let session = device.attach(pid)?;
        
        // 3. Load Script (if any)
        if !script_content.is_empty() {
             let script = session.create_script(script_content, &mut frida::ScriptOption::default())?;
             script.load()?;
             tracing::info!("Script loaded for PID {}", pid);
             // Note: Session dropping here might unload script. 
             // Ideally we need a session manager. For MVP/Stealth, some hooks persist or we rely on race.
        }

        // 4. Resume
        device.resume(pid)?;
        
        Ok(pid)
    }

    /// Attaches to a running process
    pub async fn attach_process(&self, pid: u32) -> Result<u32> {
        let device_manager = DeviceManager::new(frida::LocalDevice::new());
        let device = device_manager.get_local_device()?;
        let _session = device.attach(pid)?;
        Ok(pid)
    }

    /// Resumes a process
    pub async fn resume_process(&self, pid: u32) -> Result<()> {
        let device_manager = DeviceManager::new(frida::LocalDevice::new());
        let device = device_manager.get_local_device()?;
        device.resume(pid)?;
        Ok(())
    }

    /// Injects an arbitrary JS script into an existing process
    pub async fn inject_script(&self, pid: u32, script_content: &str) -> Result<()> {
         // Create local device manager
         let device_manager = DeviceManager::new(frida::LocalDevice::new());
         let device = device_manager.get_local_device()?;
         let session = device.attach(pid)?;
         
         if !script_content.is_empty() {
             let script = session.create_script(script_content, &mut frida::ScriptOption::default())?;
             script.load()?;
             tracing::info!("Injected custom script into PID {}", pid);
         }
         Ok(())
    }
}
