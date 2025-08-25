#!/usr/bin/env bash

set -e

mkdir -p ~/.local/bin

if [ -f "$HOME/.local/bin/commitz" ]; then
    echo "🗑️ Menghapus binary commitz lama..."
    rm -f "$HOME/.local/bin/commitz"
fi

echo "⬇️ Downloading latest commitz binary..."
curl -L "https://github.com/rynsh1506/commitz/releases/download/v0.1.0/commitz" -o ~/.local/bin/commitz
chmod +x ~/.local/bin/commitz

# Deteksi shell
SHELL_NAME=$(basename "$SHELL")
RC_FILE=""

if [ "$SHELL_NAME" = "bash" ]; then
    RC_FILE="$HOME/.bashrc"
elif [ "$SHELL_NAME" = "zsh" ]; then
    RC_FILE="$HOME/.zshrc"
else
    RC_FILE="$HOME/.profile" 
fi

if ! echo "$PATH" | grep -q "$HOME/.local/bin"; then
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$RC_FILE"
    echo "🔧 Added ~/.local/bin to PATH in $RC_FILE. Restart terminal or run: source $RC_FILE"
fi

echo "✅ commitz installed! Run: commitz"
