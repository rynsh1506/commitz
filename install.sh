#!/usr/bin/env bash
set -e

BIN_DIR="$HOME/.local/bin"
mkdir -p "$BIN_DIR"

# Tentukan OS
OS_NAME=$(uname -s)
case "$OS_NAME" in
    Linux*)   FILE_NAME="commitz-linux";;
    Darwin*)  FILE_NAME="commitz-mac";;
    MINGW*|MSYS*)        echo "Unsupported OS: $OS_NAME"; exit 1;;
esac

BINARY_PATH="$BIN_DIR/commitz"

# Hapus binary lama
if [ -f "$BINARY_PATH" ]; then
    echo "🗑️ Removing old commitz binary..."
    rm -f "$BINARY_PATH"
fi

echo "⬇️ Downloading latest $FILE_NAME..."
curl -L "https://github.com/rynsh1506/commitz/releases/download/v0.1.1/$FILE_NAME" -o "$BINARY_PATH"
chmod +x "$BINARY_PATH"

# Setup PATH di shell
SHELL_NAME=$(basename "$SHELL")
RC_FILE=""
if [ "$SHELL_NAME" = "bash" ]; then
    RC_FILE="$HOME/.bashrc"
elif [ "$SHELL_NAME" = "zsh" ]; then
    RC_FILE="$HOME/.zshrc"
else
    RC_FILE="$HOME/.profile" 
fi

if ! echo "$PATH" | grep -q "$BIN_DIR"; then
    echo "export PATH=\"$BIN_DIR:\$PATH\"" >> "$RC_FILE"
    echo "🔧 Added $BIN_DIR to PATH in $RC_FILE. Restart terminal or run: source $RC_FILE"
fi

echo "✅ commitz installed! Run: commitz"
