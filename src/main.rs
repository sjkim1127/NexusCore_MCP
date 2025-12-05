use anyhow::Result;
use dotenv::dotenv;
use nexuscore_mcp::server;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok(); // Load .env if present

    // Initialize logging
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .init();

    tracing::info!("Starting NexusCore MCP Server...");

    // Create server instance
    let _ = server::create_server();

    // Start transport (stdio) - Placeholder
    // rmcp might use a different mechanism.
    tracing::info!("Server initialized. Ready to receive MCP requests via Stdio.");
    
    // Placeholder to keep tokio main happy
    std::future::pending::<()>().await;
    Ok(())
}
