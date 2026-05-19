use serde_json::{json, Value};

const API_BASE: &str = "https://ruc.kembec.com/api/search";

pub fn schema() -> Value {
    json!({
        "name": "buscar_ruc",
        "description": "Busca un contribuyente en el padrón reducido de SUNAT por RUC de 11 dígitos.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "ruc": {
                    "type": "string",
                    "description": "RUC del contribuyente (11 dígitos numéricos)",
                    "pattern": "^\\d{11}$"
                }
            },
            "required": ["ruc"],
            "additionalProperties": false
        }
    })
}

pub async fn buscar_ruc(client: &reqwest::Client, args: &Value) -> Value {
    let ruc = match args.get("ruc").and_then(|v| v.as_str()) {
        Some(r) => r,
        None => return error_content("Parámetro 'ruc' requerido."),
    };
    if ruc.len() != 11 || !ruc.chars().all(|c| c.is_ascii_digit()) {
        return error_content(&format!(
            "RUC inválido '{}': debe tener exactamente 11 dígitos.",
            ruc
        ));
    }
    let url = format!("{}/{}", API_BASE, ruc);
    let resp = match client.get(&url).send().await {
        Ok(r) => r,
        Err(e) => return error_content(&format!("Error de red: {}", e)),
    };
    let rows: Vec<Value> = match resp.json().await {
        Ok(v) => v,
        Err(e) => return error_content(&format!("Error al parsear respuesta: {}", e)),
    };
    if rows.is_empty() {
        return json!({
            "content": [{
                "type": "text",
                "text": format!("No se encontró información para el RUC {}.", ruc)
            }]
        });
    }
    json!({
        "content": [{
            "type": "text",
            "text": format_row(&rows[0])
        }]
    })
}

fn error_content(msg: &str) -> Value {
    json!({
        "content": [{ "type": "text", "text": msg }],
        "isError": true
    })
}

fn format_row(row: &Value) -> String {
    let get = |key: &str| {
        row.get(key)
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim()
            .to_string()
    };
    let mut lines = vec![
        format!("RUC: {}", get("ruc")),
        format!("Razón social: {}", get("nombre-o-razon-social")),
        format!("Estado: {}", get("estado-del-contribuyente")),
        format!("Condición de domicilio: {}", get("condicion-de-domicilio")),
    ];
    let addr = build_address(row);
    if !addr.is_empty() {
        lines.push(format!("Dirección: {}", addr));
    }
    let dpto = get("departamento");
    if !dpto.is_empty() {
        lines.push(format!("Departamento: {}", dpto));
    }
    if let Some(ubigeo) = row.get("ubigeo").and_then(|v| v.as_i64()) {
        lines.push(format!("Ubigeo: {}", ubigeo));
    }
    lines.join("\n")
}

fn build_address(row: &Value) -> String {
    let get = |key: &str| {
        row.get(key)
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim()
            .to_string()
    };
    let mut parts = Vec::new();
    let tipo_via = get("tipo-de-via");
    let nombre_via = get("nombre-de-via");
    if !tipo_via.is_empty() || !nombre_via.is_empty() {
        parts.push(format!("{} {}", tipo_via, nombre_via).trim().to_string());
    }
    for (label, key) in [
        ("Nro.", "numero"),
        ("Int.", "interior"),
        ("Lote", "lote"),
        ("Mz.", "manzana"),
        ("Km.", "kilometro"),
    ] {
        let val = get(key);
        if !val.is_empty() {
            parts.push(format!("{} {}", label, val));
        }
    }
    let tipo_zona = get("tipo-de-zona");
    let cod_zona = get("codigo-de-zona");
    if !tipo_zona.is_empty() || !cod_zona.is_empty() {
        parts.push(format!("{} {}", tipo_zona, cod_zona).trim().to_string());
    }
    parts.join(" ")
}
