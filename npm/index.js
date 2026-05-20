#!/usr/bin/env node
const { execFileSync } = require('child_process');

const PLATFORMS = {
  'darwin-arm64': '@kembec/ruc-mcp-darwin-arm64',
  'darwin-x64': '@kembec/ruc-mcp-darwin-x64',
  'linux-x64': '@kembec/ruc-mcp-linux-x64',
  'linux-arm64': '@kembec/ruc-mcp-linux-arm64',
  'win32-x64': '@kembec/ruc-mcp-win32-x64',
};

const key = `${process.platform}-${process.arch}`;
const pkg = PLATFORMS[key];
if (!pkg) {
  console.error(`ruc-mcp: unsupported platform ${key}`);
  process.exit(1);
}

let binPath;
try {
  const binName = process.platform === 'win32' ? 'ruc-mcp.exe' : 'ruc-mcp';
  binPath = require.resolve(`${pkg}/bin/${binName}`);
} catch {
  console.error(`ruc-mcp: platform package ${pkg} is not installed. Re-run \`npm install @kembec/ruc-mcp\`.`);
  process.exit(1);
}

try {
  execFileSync(binPath, process.argv.slice(2), { stdio: 'inherit' });
} catch (e) {
  process.exit(typeof e.status === 'number' ? e.status : 1);
}
