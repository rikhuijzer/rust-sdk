use anyhow::Result;
use common::counter::Counter;
use rmcp::{ServiceExt, transport::stdio};
use tracing_subscriber::{self, EnvFilter};
use rmcp::handler::server::tool::{ToolCallContext};
use futures::future::BoxFuture;
use rmcp::Error as McpError;
use rmcp::model::CallToolResult;
mod common;
/// npx @modelcontextprotocol/inspector cargo run -p mcp-server-examples --example std_io

// C: Fn(ToolCallContext<'_, S>) -> BoxFuture<'_, Result<CallToolResult, crate::Error>>
//            + Send
//            + Sync
            //+ 'static,

fn toolcall(context: ToolCallContext<'_, Counter>) -> BoxFuture<'_, Result<CallToolResult, McpError>> {
    todo!()
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the tracing subscriber with file and stdout logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting MCP server");

    let counter = Counter::new();
    let mut server = rmcp::Server::<Counter>::new();
    let schema = "{}";
    server.tool("increment", "Increment the counter by 1", toolcall);
    // Create an instance of our counter router
    let service = Counter::new().serve(stdio()).await.inspect_err(|e| {
        tracing::error!("serving error: {:?}", e);
    })?;

    service.waiting().await?;
    Ok(())
}
