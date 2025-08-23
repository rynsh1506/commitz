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

if ! echo "$PATH" | grep -q "$HOME/.local/bin"; then
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
    echo "Added ~/.local/bin to PATH. Restart terminal or run 'source ~/.bashrc'."
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
