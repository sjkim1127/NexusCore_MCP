use nexuscore_mcp::tools::malware::sandbox_submit::CapeSubmitter;
use nexuscore_mcp::tools::malware::wrappers::external::{DieTool, CapaTool};
use nexuscore_mcp::tools::Tool;
use serde_json::json;

#[tokio::test]
async fn test_cape_submitter_metadata() {
    let tool = CapeSubmitter;
    assert_eq!(tool.name(), "cape_submit");
}

#[tokio::test]
async fn test_cape_submitter_missing_args() {
    let tool = CapeSubmitter;
    // Missing base_url
    let result = tool.execute(json!({"file_path": "test.exe"})).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_die_tool_metadata() {
    let tool = DieTool;
    assert_eq!(tool.name(), "die_scan");
}

#[tokio::test]
async fn test_capa_tool_metadata() {
    let tool = CapaTool;
    assert_eq!(tool.name(), "capa_scan");
}
