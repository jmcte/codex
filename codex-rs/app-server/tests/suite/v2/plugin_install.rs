use std::time::Duration;

use anyhow::Result;
use app_test_support::McpProcess;
use codex_app_server_protocol::PluginInstallParams;
use codex_app_server_protocol::RequestId;
use pretty_assertions::assert_eq;
use tempfile::TempDir;
use tokio::time::timeout;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

#[tokio::test]
async fn plugin_install_returns_internal_error_for_missing_marketplace_file() -> Result<()> {
    let codex_home = TempDir::new()?;
    let mut mcp = McpProcess::new(codex_home.path()).await?;
    timeout(DEFAULT_TIMEOUT, mcp.initialize()).await??;

    let request_id = mcp
        .send_plugin_install_request(PluginInstallParams {
            marketplace_path: codex_home.path().join("missing-marketplace.json"),
            plugin_name: "missing-plugin".to_string(),
        })
        .await?;

    let err = timeout(
        DEFAULT_TIMEOUT,
        mcp.read_stream_until_error_message(RequestId::Integer(request_id)),
    )
    .await??;

    assert_eq!(err.error.code, -32603);
    assert!(err.error.message.contains("failed to install plugin"));
    Ok(())
}
