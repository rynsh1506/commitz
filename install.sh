#!/usr/bin/env bash

# Download latest commitz binary
mkdir -p ~/.local/bin
curl -L https://example.com/bin/commitz -o ~/.local/bin/commitz
chmod +x ~/.local/bin/commitz

# Pastikan ~/.local/bin ada di PATH
if ! echo "$PATH" | grep -q "$HOME/.local/bin"; then
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
    echo "Added ~/.local/bin to PATH. Restart terminal or run 'source ~/.bashrc'."
fi

echo "✅ commitz installed! Just run: commitz"