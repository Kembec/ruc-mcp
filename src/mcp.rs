use crate::tools;
use anyhow::Result;
use serde_json::{json, Value};
use std::sync::Arc;

const PARSE_ERROR: i32 = -32700;
const METHOD_NOT_FOUND: i32 = -32601;
const INVALID_PARAMS: i32 = -32602;

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

fn ok(id: Value, result: Value) -> String {
    serde_json::to_string(&json!({ "jsonrpc": "2.0", "id": id, "result": result }))
        .unwrap_or_default()
}

fn err(id: Value, code: i32, message: &str) -> String {
    serde_json::to_string(&json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": { "code": code, "message": message }
    }))
    .unwrap_or_default()
}

pub async fn handle_line(state: Arc<ServerState>, line: &str) -> Option<String> {
    let req: Value = match serde_json::from_str(line) {
        Ok(v) => v,
        Err(_) => return Some(err(Value::Null, PARSE_ERROR, "invalid JSON")),
    };

    let id = req.get("id").cloned().unwrap_or(Value::Null);
    let method = match req.get("method").and_then(|v| v.as_str()) {
        Some(m) => m,
        None => return Some(err(id, PARSE_ERROR, "missing method")),
    };

    match method {
        "initialize" => Some(ok(
            id,
            json!({
                "protocolVersion": "2024-11-05",
                "capabilities": { "tools": {} },
                "serverInfo": { "name": "ruc-mcp", "version": env!("CARGO_PKG_VERSION") }
            }),
        )),
        "notifications/initialized" => None,
        "ping" => Some(ok(id, json!({}))),
        "tools/list" => Some(ok(id, json!({ "tools": [tools::schema()] }))),
        "tools/call" => {
            let params = match req.get("params") {
                Some(p) => p,
                None => return Some(err(id, INVALID_PARAMS, "missing params")),
            };
            let name = match params.get("name").and_then(|v| v.as_str()) {
                Some(n) => n,
                None => return Some(err(id, INVALID_PARAMS, "missing tool name")),
            };
            let args = params.get("arguments").cloned().unwrap_or(json!({}));
            match name {
                "buscar_ruc" => Some(ok(id, tools::buscar_ruc(&state.client, &args).await)),
                other => Some(err(
                    id,
                    METHOD_NOT_FOUND,
                    &format!("unknown tool `{}`", other),
                )),
            }
        }
        other => {
            if id == Value::Null {
                None
            } else {
                Some(err(
                    id,
                    METHOD_NOT_FOUND,
                    &format!("unknown method `{}`", other),
                ))
            }
        }
    }
}
