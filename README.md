# commitz

`commitz` adalah CLI sederhana untuk membuat commit message yang konsisten (mirip commitizen) dengan antarmuka TUI berbasis `crossterm`.

---

## 🚀 Install

### Linux / macOS / Windows (Git Bash)

Jalankan:

```bash
bash <(curl -s https://raw.githubusercontent.com/rynsh1506/commitz/main/install.sh)
```

Lalu cek dengan:

```bash
commitz
```

---

### Windows (PowerShell)

Jalankan:

```powershell
irm https://raw.githubusercontent.com/rynsh1506/commitz/main/install.ps1 | iex
```

Lalu cek dengan:

```powershell
commitz
```

---

## 📝 Cara Pakai

1. Pastikan sudah berada di folder project Git dan jalankan:

```bash
git init

```

2. Pastikan ada file yang sudah di-`git add`.
3. Jalankan:

```bash
commitz
```

4. Pilih jenis commit dengan panah atas/bawah.
5. Isi promt pesan commit.
6. Ketik **y**/**n**, untuk menjalankan `git commit` atau membatalkan .

---

## 📂 Struktur Project

```
commitz/
├── Cargo.toml
├── README.md
├── install.sh
├── install.ps1
├── src/
│   ├── lib.rs            # expose semua module
│   ├── main.rs           # CLI entrypoint
│   └── commit/
│       ├── mod.rs        # pub mod commit { ... }
│       ├── types.rs      # definisi CommitType, dsb
│       ├── reader.rs     # baca commit config/json
│       ├── renderer.rs   # render ke layar pakai crossterm
│       ├── navigation.rs # handle arrow key / pointer
│       ├── validator.rs  # validasi input (Y/n, dsb)
│       └── signal.rs     # handle ctrl+c / exit clean
```

---

## 🤝 Kontribusi

Kontribusi sangat terbuka 🚀

1. **Fork** repository ini
2. Buat branch baru:

   ```bash
   git checkout -b feature/nama-fitur
   ```

3. Commit perubahanmu:

   ```bash
   git commit -m "feat: tambah fitur X"
   ```

4. Push ke branch:

   ```bash
   git push origin feature/nama-fitur
   ```

5. Buat **Pull Request** 🎉

---

Dibuat dengan ❤️ menggunakan Rust + Crossterm.
