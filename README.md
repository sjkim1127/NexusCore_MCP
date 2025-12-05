# NexusCore MCP

NexusCore MCP is a **Model Context Protocol (MCP)** server tailored for **advanced malware analysis** and **dynamic instrumentation**. It empowers AI agents to perform deep security analysis by providing a unified interface to powerful tools like Frida, Scylla, YARA, and CAPEv2.

## 🚀 Features

### 🛡️ Malware Analysis Domain
- **Execution Control**: `spawn_process` (Suspeneded spawn via Frida) for anti-debugging bypass.
- **Unpacking**: `find_oep` (OEP detection using `iced-x86`), `pe_fixer` (PE header reconstruction).
- **IAT Recovery**: `iat_fixer` (Import Address Table reconstruction via Scylla).
- **Sandboxing**: `cape_submit` (Automated submission to CAPEv2 Sandbox).
- **Static Analysis**: `yara_scan` (Memory/File scanning), `die_scan` (Compiler/Packer detection), `capa_scan` (Capability analysis).

### 🔧 Dynamic Instrumentation (Frida)
- **Process Control**: Spawn, Attach, Resume.
- **Memory**: Read/Write/Search process memory.
- **Hooking**: Install dynamic hooks (JavaScript/C) into running processes.

### 🌐 Network & System
- **Traffic**: `network_capture` (Tshark integration).
- **Events**: `etw_monitor` (Windows Event Tracing).

## 📂 Architecture

- `src/engine`: Core logic (Frida handler).
- `src/tools`:
  - `common`: Basic process/memory/network tools.
  - `malware`: Specialized analysis tools (Unpacker, IAT, Sandbox, Yara).
- `src/sandbox`: Integration with external sandboxes (CAPEv2).
- `scripts/`: Helper scripts for environment setup.

## 🛠️ Usage

### 1. Easy Setup (All-in-One)
We provide a PowerShell script to automatically install all necessary dependencies (Sysinternals, DIE, Capa, Tshark, etc.) and configure the environment.

Open PowerShell as **Administrator** and run:
```powershell
./scripts/setup_tools.ps1
```

### 2. Build & Run
```bash
cargo run --release
```

### 3. AI Integration
Provide the `nexuscore_mcp.exe` path to your MCP-compatible AI agent configuration.

## 📦 Tools List

| Tool | Description |
|------|-------------|
| `spawn_process` | Spawns process in suspended state (Frida) |
| `find_oep` | Analyzes entry point code to find OEP |
| `iat_fixer` | Rebuilds Import Address Table |
| `cape_submit` | Submits file to CAPEv2 sandbox |
| `yara_scan` | Scans file or process memory with YARA |
| `die_scan` | Detects compiler/packer signatures |

## License
MIT License