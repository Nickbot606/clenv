#!/bin/sh

set -e

REPO_URL="https://github.com/Nickbot606/clenv"
VERSION="v0.0.0"
BINARY_NAME="clenv"

# Detect the operating system
OS=$(uname -s)
ARCH=$(uname -m)

case "$OS" in 
    Linux) PLATFORM="linux" ;;
    Darwin) PLATFORM="macos" ;;
    *) echo "Unsupported OS: $OS"; exit 1 ;;
esac

case "$ARCH" in
    X86_64) ARCH="x86_64" ;;
    arm64) ARCH="aarch64" ;;
    *) echo "Unsupported OS: $OS"; exit 1 ;;
esac

BINARY_URL="$REPO_URL/releases/download/$VERSION/${BINARY_NAME}-${PLATFORM}-${ARCH}"

echo "Downloading $BINARY_NAME from $BINARY_URL..."

curl -L "$BINARY_URL" - o "/tmp/$BINARY_NAME"
chmod +x "/tmp/$BINARY_NAME"
sudo mv "/tmp/$BINARY_NAME" /usr/local/bin/$BINARY_NAME

echo "Installed $BINARY_NAME to /usr/local/bin/"
echo "try out $BINARY_NAME --help"