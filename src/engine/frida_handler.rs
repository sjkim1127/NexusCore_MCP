use anyhow::Result;
use frida::{DeviceManager, Device};

pub fn spawn(path: &str, args: &[String]) -> Result<u32> {
    let manager = DeviceManager::new();
    let local_device = manager.get_local_device()?;
    // args needs to be referenced correctly for frida crate
    // basic signature: spawn(program, argv, envp, cwd)
    let pid = local_device.spawn(path, &args.iter().map(|s| s.as_str()).collect::<Vec<_>>(), &[], &[])?;
    Ok(pid)
}

pub fn attach(pid: u32) -> Result<()> {
    let manager = DeviceManager::new();
    let local_device = manager.get_local_device()?;
    let _session = local_device.attach(pid)?;
    // Session needs to be kept alive in a real app, here we just check if it attaches.
    // In a real implementation, we'd store the session in a global/state.
    Ok(())
}

pub fn resume(pid: u32) -> Result<()> {
    let manager = DeviceManager::new();
    let local_device = manager.get_local_device()?;
    local_device.resume(pid)?;
    Ok(())
}

pub fn execute_script(pid: u32, source: &str) -> Result<String> {
    let manager = DeviceManager::new();
    let local_device = manager.get_local_device()?;
    let session = local_device.attach(pid)?;
    let script = session.create_script(source, &mut |msg| {
        // Handle messages (log, send) - simplified for now
        println!("Message from script: {}", msg);
    }, None)?;
    
    script.load()?;
    // For synchronous operations, we might wait for a message or just load it.
    // If we need a return value, we'd need a channel to receive the message back.
    // This is a complex part of Frida-RS.
    // For now, returning "Loaded" to indicate success.
    
    Ok("Script loaded".to_string())
}
