use crate::tools;
use anyhow::Result;
use serde_json::{json, Value};
use std::sync::Arc;

pub struct ServerState {
    pub client: reqwest::Client,
}

impl ServerState {
    pub fn new() -> Result<Self> {
        let client = reqwest::Client::builder()
            .https_only(true)
            .user_agent(concat!("ruc-mcp/", env!("CARGO_PKG_VERSION")))
            .build()?;
        Ok(Self { client })
    }
}

pub async fn handle_line(state: Arc<ServerState>, line: &str) -> Option<String> {
    let req: Value = serde_json::from_str(line).ok()?;
    let id = req.get("id").cloned().unwrap_or(Value::Null);
    let method = req.get("method")?.as_str()?;

    let result: Option<Value> = match method {
        "initialize" => Some(json!({
            "protocolVersion": "2024-11-05",
            "capabilities": { "tools": {} },
            "serverInfo": { "name": "ruc-mcp", "version": env!("CARGO_PKG_VERSION") }
        })),
        "notifications/initialized" => return None,
        "ping" => Some(json!({})),
        "tools/list" => Some(json!({ "tools": [tools::schema()] })),
        "tools/call" => {
            let params = req.get("params")?;
            let name = params.get("name")?.as_str()?;
            let args = params.get("arguments").cloned().unwrap_or(json!({}));
            match name {
                "buscar_ruc" => Some(tools::buscar_ruc(&state.client, &args).await),
                _ => None,
            }
        }
        _ => None,
    };

    result.map(|r| {
        serde_json::to_string(&json!({ "jsonrpc": "2.0", "id": id, "result": r }))
            .unwrap_or_default()
    })
}
