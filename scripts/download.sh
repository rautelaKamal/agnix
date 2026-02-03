#!/bin/bash
set -euo pipefail

# Download agnix binary for the current platform
# Environment variables:
#   AGNIX_VERSION - Version to download (default: latest)
#   BUILD_FROM_SOURCE - Set to "true" to build from source instead of downloading

REPO="avifenesh/agnix"
VERSION="${AGNIX_VERSION:-latest}"
BUILD_FROM_SOURCE="${BUILD_FROM_SOURCE:-false}"

# Create bin directory
BIN_DIR="${GITHUB_WORKSPACE:-$(pwd)}/.agnix-bin"
mkdir -p "${BIN_DIR}"

# Build from source if requested (useful for testing before releases exist)
if [ "${BUILD_FROM_SOURCE}" = "true" ]; then
    echo "Building agnix from source..."

    # Ensure Rust is available
    if ! command -v cargo &> /dev/null; then
        echo "Error: cargo not found. Install Rust to build from source." >&2
        exit 1
    fi

    # Build release binary
    cargo build --release --bin agnix

    # Copy to bin directory
    if [ -f "target/release/agnix" ]; then
        cp "target/release/agnix" "${BIN_DIR}/"
    elif [ -f "target/release/agnix.exe" ]; then
        cp "target/release/agnix.exe" "${BIN_DIR}/"
    else
        echo "Error: Could not find built binary" >&2
        exit 1
    fi

    chmod +x "${BIN_DIR}/agnix" 2>/dev/null || true
    echo "${BIN_DIR}" >> "${GITHUB_PATH:-/dev/null}"
    echo "agnix built from source and installed to ${BIN_DIR}"
    exit 0
fi

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

# Map to release artifact name
case "${OS}" in
    Linux)
        case "${ARCH}" in
            x86_64)
                TARGET="x86_64-unknown-linux-gnu"
                EXT="tar.gz"
                ;;
            *)
                echo "Error: Unsupported Linux architecture: ${ARCH}" >&2
                exit 1
                ;;
        esac
        ;;
    Darwin)
        case "${ARCH}" in
            x86_64)
                TARGET="x86_64-apple-darwin"
                EXT="tar.gz"
                ;;
            arm64)
                TARGET="aarch64-apple-darwin"
                EXT="tar.gz"
                ;;
            *)
                echo "Error: Unsupported macOS architecture: ${ARCH}" >&2
                exit 1
                ;;
        esac
        ;;
    MINGW*|MSYS*|CYGWIN*|Windows_NT)
        TARGET="x86_64-pc-windows-msvc"
        EXT="zip"
        ;;
    *)
        echo "Error: Unsupported OS: ${OS}" >&2
        exit 1
        ;;
esac

ARTIFACT_NAME="agnix-${TARGET}.${EXT}"

# Resolve version
if [ "${VERSION}" = "latest" ]; then
    echo "Fetching latest release version..."
    VERSION=$(curl -sL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    if [ -z "${VERSION}" ]; then
        echo "Error: Could not determine latest version. No releases found." >&2
        echo "Please ensure a release exists at https://github.com/${REPO}/releases" >&2
        echo "Or set BUILD_FROM_SOURCE=true to build from source." >&2
        exit 1
    fi
fi

echo "Downloading agnix ${VERSION} for ${TARGET}..."

# Download URL
DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/${ARTIFACT_NAME}"

# Download and extract
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "${TEMP_DIR}"' EXIT

echo "Downloading from ${DOWNLOAD_URL}..."
HTTP_CODE=$(curl -sL -w "%{http_code}" "${DOWNLOAD_URL}" -o "${TEMP_DIR}/${ARTIFACT_NAME}")

if [ "${HTTP_CODE}" != "200" ]; then
    echo "Error: Failed to download release (HTTP ${HTTP_CODE})" >&2
    echo "URL: ${DOWNLOAD_URL}" >&2
    exit 1
fi

echo "Extracting..."
case "${EXT}" in
    tar.gz)
        tar -xzf "${TEMP_DIR}/${ARTIFACT_NAME}" -C "${BIN_DIR}"
        ;;
    zip)
        unzip -q "${TEMP_DIR}/${ARTIFACT_NAME}" -d "${BIN_DIR}"
        ;;
esac

# Make executable
chmod +x "${BIN_DIR}/agnix" 2>/dev/null || true

# Add to PATH for subsequent steps
echo "${BIN_DIR}" >> "${GITHUB_PATH:-/dev/null}"

echo "agnix ${VERSION} installed to ${BIN_DIR}"
