#!/usr/bin/env node
/**
 * CodexCTL - Wrapper that uses platform-specific binary from optionalDependencies
 * 
 * When npm installs this package, it automatically downloads the correct
 * @codexctl/{platform} package based on OS/arch. The binary is then available
 * in node_modules/@codexctl/{platform}/bin/
 */

const path = require('path');
const os = require('os');
const fs = require('fs');
const { spawn } = require('child_process');

'use strict';

const PLATFORMS = {
  'linux-x64': '@codexctl/linux-x64',
  'linux-arm64': '@codexctl/linux-arm64',
  'darwin-x64': '@codexctl/darwin-x64',
  'darwin-arm64': '@codexctl/darwin-arm64',
  'win32-x64': '@codexctl/win32-x64'
};

function getPlatformKey() {
  return `${os.platform()}-${os.arch()}`;
}

function resolvePlatformPackage() {
  const key = getPlatformKey();
  return PLATFORMS[key] || null;
}

const platformPackage = resolvePlatformPackage();
if (!platformPackage) {
  const supported = Object.keys(PLATFORMS).sort().join(', ');
  console.error(`Unsupported platform/architecture: ${getPlatformKey()}`);
  console.error(`Supported targets: ${supported}`);
  process.exit(1);
}

const binDir = path.join(__dirname, '..', platformPackage, 'bin');
const isWindows = os.platform() === 'win32';
const binaryName = isWindows ? 'codexctl.exe' : 'codexctl';
const binaryPath = path.join(binDir, binaryName);

if (!fs.existsSync(binaryPath)) {
  console.error(`codexctl binary not found at: ${binaryPath}`);
  console.error('Reinstall package to fetch the correct optional dependency for this platform.');
  process.exit(1);
}

const child = spawn(binaryPath, process.argv.slice(2), { 
  stdio: 'inherit', 
  windowsHide: true 
});

child.on('exit', (code) => process.exit(code ?? 0));
