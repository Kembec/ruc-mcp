use anyhow::Result;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

mod mcp;
mod tools;

#[tokio::main]
async fn main() -> Result<()> {
    eprintln!("ruc-mcp starting");
    let state = match mcp::ServerState::new() {
        Ok(s) => Arc::new(s),
        Err(e) => {
            eprintln!("ruc-mcp: failed to initialize: {e}");
            std::process::exit(1);
        }
    };
    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin).lines();
    let mut stdout = tokio::io::stdout();
    while let Some(line) = reader.next_line().await? {
        if line.trim().is_empty() {
            continue;
        }
        if let Some(response) = mcp::handle_line(state.clone(), &line).await {
            stdout.write_all(response.as_bytes()).await?;
            stdout.write_all(b"\n").await?;
            stdout.flush().await?;
        }
    }
    Ok(())
}
