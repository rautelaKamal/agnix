#!/usr/bin/env bash
# Sync version from Cargo.toml to all package manifests.
# Run before tagging a release or as part of CI.

set -euo pipefail

# Extract version from workspace Cargo.toml
VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')

if [ -z "$VERSION" ]; then
  echo "Error: Could not extract version from Cargo.toml"
  exit 1
fi

echo "Syncing version: $VERSION"

# VS Code extension
if [ -f editors/vscode/package.json ]; then
  sed -i.bak "s/\"version\": \"[^\"]*\"/\"version\": \"$VERSION\"/" editors/vscode/package.json
  rm -f editors/vscode/package.json.bak
  echo "  Updated editors/vscode/package.json"
fi

# npm package
if [ -f npm/package.json ]; then
  sed -i.bak "s/\"version\": \"[^\"]*\"/\"version\": \"$VERSION\"/" npm/package.json
  rm -f npm/package.json.bak
  echo "  Updated npm/package.json"
fi

echo "All versions synced to $VERSION"
