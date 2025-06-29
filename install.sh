#!/bin/sh
set -e

REPO="Nickbot606/clenv"
BINARY_NAME="clenv"

# Get latest version
VERSION=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep '"tag_name":' | sed -E 's/.*"v([^"]+)".*/v\1/')

# Detect OS
OS=$(uname -s)
ARCH=$(uname -m)

case "$OS" in 
    Linux) PLATFORM="linux" ;;
    Darwin) PLATFORM="macos" ;;
    MINGW*|MSYS*|CYGWIN*) PLATFORM="windows" ;;
    *) echo "Unsupported OS: $OS"; exit 1 ;;
esac

case "$ARCH" in
    x86_64|amd64) ARCH="x86_64" ;;
    arm64|aarch64) ARCH="aarch64" ;;
    *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

EXT=""
[ "$PLATFORM" = "windows" ] && EXT=".exe"

BINARY_URL="https://github.com/$REPO/releases/download/$VERSION/${BINARY_NAME}-${PLATFORM}-${ARCH}${EXT}"

echo "Downloading $BINARY_NAME from $BINARY_URL..."

curl -L "$BINARY_URL" -o "/tmp/${BINARY_NAME}${EXT}"
chmod +x "/tmp/${BINARY_NAME}${EXT}"

# Install
INSTALL_PATH="/usr/local/bin/${BINARY_NAME}"
if [ "$PLATFORM" = "windows" ]; then
    INSTALL_PATH="$HOME/.cargo/bin/${BINARY_NAME}.exe"
    mkdir -p "$(dirname "$INSTALL_PATH")"
fi

sudo mv "/tmp/${BINARY_NAME}${EXT}" "$INSTALL_PATH"

echo "Installed $BINARY_NAME to $INSTALL_PATH"
"$INSTALL_PATH" --help
