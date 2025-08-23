# commitz

`commitz` adalah CLI sederhana untuk membuat commit message yang konsisten (mirip commitizen) dengan antarmuka TUI berbasis `crossterm`.

---

## 🚀 Install (Linux)

Cukup jalankan satu command ini:

```bash
bash <(curl -s https://raw.githubusercontent.com/rynsh1506/commitz/main/install.sh)
```

> Script `install.sh` akan melakukan:
>
> 1. Download binary `commitz` untuk Linux
> 2. Simpan ke `~/.local/bin/commitz`
> 3. Set executable (`chmod +x`)
> 4. Tambahkan `~/.local/bin` ke `$PATH` kalau belum ada

Contoh isi `install.sh`:

```bash
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

```

Setelah itu langsung bisa pakai:

```bash
commitz
```

---

## 📝 Cara Pakai

1. Pastikan ada file yang sudah di-`git add`.
2. Jalankan:

```bash
commitz
```

3. Pilih jenis commit dengan panah atas/bawah.
4. Ketik pesan commit.
5. Tekan **Enter**, `git commit` akan dijalankan otomatis.

---

## 📂 Struktur Project

```
.
├── src/              # kode sumber (untuk developer)
├── Cargo.toml
├── bin/
│   └── commitz       # binary Linux siap pakai
├── install.sh        # script untuk install one-liner
└── README.md
```

---

Dibuat dengan ❤️ menggunakan Rust + Crossterm.
