use anyhow::Result;
use serde_json::Value;
use crate::tools::Tool;
use async_trait::async_trait;
use winreg::enums::*;
use winreg::RegKey;
use std::path::Path;

pub struct PersistenceHunter;

#[async_trait]
impl Tool for PersistenceHunter {
    fn name(&self) -> &str { "scan_persistence" }
    fn description(&self) -> &str { "Scans registry Run keys and Startup folders for persistence mechanisms. No args." }
    
    async fn execute(&self, _args: Value) -> Result<Value> {
        let mut results = Vec::new();

        // 1. Registry Run Keys (HKCU & HKLM)
        let keys_to_check = [
            (HKEY_CURRENT_USER, r"Software\Microsoft\Windows\CurrentVersion\Run"),
            (HKEY_CURRENT_USER, r"Software\Microsoft\Windows\CurrentVersion\RunOnce"),
            (HKEY_LOCAL_MACHINE, r"Software\Microsoft\Windows\CurrentVersion\Run"),
            (HKEY_LOCAL_MACHINE, r"Software\Microsoft\Windows\CurrentVersion\RunOnce"),
            (HKEY_LOCAL_MACHINE, r"Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Run"), // 32-bit apps on 64-bit OS
        ];

        for (hive, path) in keys_to_check {
            if let Ok(root) = RegKey::predef(hive).open_subkey(path) {
                for (name, value) in root.enum_values().map(|x| x.unwrap_or(("Error".to_string(), RegValue { vtype: REG_NONE, bytes: vec![] }))) {
                    if !name.is_empty() {
                         results.push(serde_json::json!({
                            "type": "registry",
                            "hive": if hive == HKEY_CURRENT_USER { "HKCU" } else { "HKLM" },
                            "path": path,
                            "name": name,
                            "value": value.to_string()
                        }));
                    }
                }
            }
        }

        // 2. Startup Folders (User & Common)
        // Using Environment variables to find paths
        let startup_vars = ["APPDATA", "PROGRAMDATA"];
        let startup_suffix = r"Microsoft\Windows\Start Menu\Programs\Startup";

        for var in startup_vars {
            if let Ok(base_path) = std::env::var(var) {
                let full_path =  Path::new(&base_path).join(startup_suffix);
                if full_path.exists() {
                     if let Ok(entries) = std::fs::read_dir(full_path) {
                         for entry in entries.flatten() {
                             if let Ok(file_type) = entry.file_type() {
                                 if file_type.is_file() {
                                     results.push(serde_json::json!({
                                         "type": "file",
                                         "path": entry.path().to_string_lossy(),
                                         "name": entry.file_name().to_string_lossy()
                                     }));
                                 }
                             }
                         }
                     }
                }
            }
        }

        Ok(serde_json::json!({
            "count": results.len(),
            "items": results
        }))
    }
}
