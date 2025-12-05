#[cfg(test)]
mod tests {
    use nexuscore_mcp::tools::process::SpawnProcess;
    use nexuscore_mcp::tools::Tool;

    #[tokio::test]
    async fn test_spawn_process_metadata() {
        let tool = SpawnProcess;
        assert_eq!(tool.name(), "spawn_process");
        assert!(tool.description().contains("Spawns"));
    }
}
