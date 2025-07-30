use rmcp::{ServiceExt, transport::stdio};
use snafu::ResultExt;
use todolist_mcp::TodoMcpServer;
use tracing::{debug, info};
use tracing_subscriber::{self, EnvFilter};

#[tokio::main]
#[snafu::report]
async fn main() -> Result<(), snafu::Whatever> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("todolist_mcp=debug,rmcp=debug")),
        )
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    info!("TodoList MCP Server starting");
    debug!("Args: {:?}", std::env::args().collect::<Vec<_>>());
    debug!("Current dir: {:?}", std::env::current_dir());
    debug!(
        "Stdin is terminal: {}",
        std::io::IsTerminal::is_terminal(&std::io::stdin())
    );
    debug!(
        "Stdout is terminal: {}",
        std::io::IsTerminal::is_terminal(&std::io::stdout())
    );

    let service = TodoMcpServer::new()
        .serve(stdio())
        .await
        .whatever_context("Failed to start MCP server")?;

    info!("MCP server started, waiting for requests");
    service
        .waiting()
        .await
        .whatever_context("Service task failed")?;

    info!("MCP server terminated successfully");
    Ok(())
}
