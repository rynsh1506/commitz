#!/usr/bin/env bash

mkdir -p ~/.local/bin

curl -L https://github.com/rynsh1506/commitz/raw/refs/heads/main/bin/commitz -o ~/.local/bin/commitz
chmod +x ~/.local/bin/commitz

# Deteksi shell
SHELL_NAME=$(basename "$SHELL")
RC_FILE=""

if [ "$SHELL_NAME" = "bash" ]; then
    RC_FILE="$HOME/.bashrc"
elif [ "$SHELL_NAME" = "zsh" ]; then
    RC_FILE="$HOME/.zshrc"
else
    RC_FILE="$HOME/.profile" # fallback
fi

# Tambahkan ~/.local/bin ke PATH kalau belum ada
if ! echo "$PATH" | grep -q "$HOME/.local/bin"; then
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$RC_FILE"
    echo "Added ~/.local/bin to PATH in $RC_FILE. Restart terminal or run 'source $RC_FILE'."
fi

echo "✅ commitz installed! Just run: commitz"
