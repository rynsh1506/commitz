#!/usr/bin/env bash

mkdir -p ~/.local/bin

curl -L https://github.com/rynsh1506/commitz/raw/refs/heads/main/bin/commitz -o ~/.local/bin/commitz

chmod +x ~/.local/bin/commitz

if ! echo "$PATH" | grep -q "$HOME/.local/bin"; then
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
    echo "Added ~/.local/bin to PATH. Restart terminal or run 'source ~/.bashrc'."
fi

echo "✅ commitz installed! Just run: commitz"


