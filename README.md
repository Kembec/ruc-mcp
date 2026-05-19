# ruc-mcp
[![npm](https://img.shields.io/npm/v/@kembec/ruc-mcp)](https://www.npmjs.com/package/@kembec/ruc-mcp)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

Busca contribuyentes peruanos en el padrón reducido de SUNAT (~9M registros) por RUC. Binario Rust estático, sin runtime.

## Installation

```bash
npm install -g @kembec/ruc-mcp
```

Or without installing: `npx @kembec/ruc-mcp`

## Configuration

No credentials required. Queries the public API at `https://ruc.kembec.com`.

## Cursor

`~/.cursor/mcp.json`:

```json
{
  "mcpServers": {
    "ruc": { "command": "npx", "args": ["@kembec/ruc-mcp"] }
  }
}
```

## Claude Desktop

`claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "ruc": { "command": "npx", "args": ["@kembec/ruc-mcp"] }
  }
}
```

## Codex CLI

`~/.codex/config.toml`:

```toml
[mcp_servers.ruc]
command = "npx"
args = ["@kembec/ruc-mcp"]
```

## Tools

### buscar_ruc

Busca información de un contribuyente por RUC de 11 dígitos.

**Parameters:** `ruc` (string, required) — 11 dígitos numéricos.

**Returns:** razón social, estado, condición de domicilio, dirección fiscal.

## Building from source

```bash
git clone https://github.com/Kembec/ruc-mcp
cd ruc-mcp
cargo build --release
./target/release/ruc-mcp
```

## License

Apache-2.0
