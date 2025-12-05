# NexusCore MCP

NexusCore MCP is a dynamic analysis Model Context Protocol (MCP) server built with **Rust**, leveraging **Frida** for powerful instrumentation and process manipulation.

It is designed to be a highly modular and secure environment for analyzing binaries, detecting vulnerabilities, and inspecting runtime memory, tailored for security researchers and automated analysis pipelines.

## Features

### 🔍 Dynamic Analysis
- **Spawn & Attach**: Launch new processes in a suspended state or attach to running ones.
- **Memory Inspection**: Read and search process memory (Boyer-Moore pattern matching).
- **Instrumentation**: Inject JavaScript hooks via Frida to intercept function calls and modify behavior.

### 🛡️ Security Tools
- **DefenderBot**: Automated security checker for common misconfigurations (ASLR, DEP, etc.).
- **CodeQL Integration**: Trigger static analysis scans using CodeQL (requires CodeQL CLI).

## Architecture
The server is built on a plugin-based architecture using Rust traits:
- **Engine**: Core instrumentation logic powered by `frida-rs`.
- **Tools**: Modular capabilities implementing the `Tool` trait.
- **Protocol**: Compliant with the Model Context Protocol (MCP) for seamless integration with AI agents.

## Installation

### Prerequisites
- **Rust Toolchain**: [Install Rust](https://www.rust-lang.org/tools/install)
- **Frida**: Ensure your environment supports Frida (usually handled by the crate, but LLVM/Clang might be required for binding generation).

### Build
```bash
git clone https://github.com/sjkim1127/Nexuscore_MCP.git
cd Nexuscore_MCP
cargo build --release
```

## Usage

Run the server directly (it uses stdio for MCP transport):
```bash
./target/release/nexuscore_mcp
```

### Configuration in MCP Client
Add the server to your MCP client configuration (e.g., `claude_desktop_config.json`):

```json
{
  "mcpServers": {
    "nexuscore": {
      "command": "path/to/nexuscore_mcp.exe",
      "args": []
    }
  }
}
```

## Tools Available
| Tool Name | Description |
|-----------|-------------|
| `spawn_process` | Spawns a target process in a suspended state. |
| `attach_process` | Attaches to a running process by PID. |
| `read_memory` | Reads raw bytes from a specific memory address. |
| `search_memory` | Scans memory for specific byte patterns. |
| `install_hook` | Injects a JavaScript hook into a function. |
| `defender_bot` | Checks for security features and hardening. |
| `codeql_scan` | Runs a CodeQL database creation and query. |

## Contribution
1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/amazing-tool`).
3. Commit your changes (`git commit -m 'Add amazing tool'`).
4. Push to the branch (`git push origin feature/amazing-tool`).
5. Open a Pull Request.

## License
MIT License