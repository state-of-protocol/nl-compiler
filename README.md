# NEO Language (NL) – Pengkompil Bahasa Pengaturcaraan Sistem Moden

![NEO Language](https://img.shields.io/badge/NEO_Language-v0.1.0-blue)
![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)
![C](https://img.shields.io/badge/Output-C-green)
![Lesen](https://img.shields.io/badge/Lesen-MIT-brightgreen)
![Status](https://img.shields.io/badge/Status-MVP%20Siap-success)

**NEO Language (NL)** ialah bahasa pengaturcaraan sistem yang dibina khas untuk ekosistem **NEO OS**. Ia menggandingkan **keselamatan ala Rust** dengan **kesederhanaan ala Go**, memberikan pengalaman pembangunan yang pantas, selamat, dan mudah difahami. NL direka sebagai bahasa peringkat tinggi yang dapat menyasarkan pelbagai pelantar sasaran melalui penjanaan kod C.

---

## Visi

> "Membina jambatan antara keselamatan ingatan yang ketat dan sintaks yang mudah, untuk mempercepat pembangunan sistem dalam ekosistem NEO OS."

Komponen utama pengkompil NL adalah:

- **Lekser** – Mengecam token dalam kod sumber (kata kunci Melayu seperti `biar`, `jika`, `sementara` dan operator biasa).
- **Penghurai** – Membina Pokok Sintaks Abstrak (AST) dengan tatabahasa yang jelas.
- **Penganalisis Semantik** – Menyemak pengisytiharan pemboleh ubah dan penggunaan simbol.
- **Penjana Kod** – Mentranslasikan AST kepada kod C yang boleh dikompil oleh mana-mana pengkompil C standard.

Untuk memahami hala tuju jangka panjang (termasuk sasaran untuk kernel NEO OS), sila rujuk **[vision_process.md](vision_process.md)**.

---

## 🚀 Bermula

### Prasyarat

1.  **Rust & Cargo** – Pasang melalui [rustup.rs](https://rustup.rs/).
2.  **Microsoft C++ Build Tools** (Windows) atau **GCC/Clang** (Linux/macOS) – Diperlukan untuk mengkompil kod C yang dijana.
3.  **Git** (pilihan) – Untuk klon dan sumbangan.

### Membina & Menjalankan Pengkompil

1.  **Dapatkan kod sumber**:

    ```bash
    git clone https://github.com/state-of-protocol/nl-compiler.git
    cd nl-compiler
    ```

2.  **Bina pengkompil**:

    ```bash
    cargo build --release
    ```

3.  **Laksanakan pengkompil** dengan aturcara NL contoh:

    ```bash
    cargo run -- examples/hello.nl
    ```

    Ini akan menghasilkan kod C di terminal. Untuk menyimpan ke fail dan mengkompilnya:

    ```bash
    cargo run -- examples/hello.nl > output.c
    cl output.c /Fe:hello.exe   # Windows (msvc)
    # atau
    gcc output.c -o hello        # Linux/macOS
    ./hello
    ```

### 📖 Contoh Ringkas

Berikut ialah aturcara NL yang mudah, `hello.nl`:

```nl
fn main() {
    biar x = 10;
    jika x < 20 {
        println("Hello, NEO Language! x = ", x);
    }
}
```

Keluaran selepas kompilasi:

```
Hello, NEO Language! x = 10
```

---

## 🧱 Struktur Projek

Projek ini menggunakan **ruang kerja Cargo** (workspace) untuk pemisahan modul yang kemas:

```
nl-compiler/
├── Cargo.toml                # Ruang kerja Cargo
├── crates/
│   ├── nl-lexer/             # Tokenizer
│   │   ├── token.rs
│   │   └── lexer.rs
│   ├── nl-ast/               # Definisi Pokok Sintaks Abstrak
│   │   └── node.rs
│   ├── nl-parser/            # Penghurai rekursif-menurun
│   │   └── parser.rs
│   ├── nl-semantic/          # Penganalisis semantik asas
│   │   └── resolver.rs
│   ├── nl-codegen/           # Penjana kod C
│   │   └── codegen.rs
│   └── nl-driver/            # Binari CLI pengkompil
│       └── main.rs
└── examples/                 # Aturcara NL contoh
    └── hello.nl
```

Setiap peti (crate) boleh diuji secara berasingan, contohnya:

```bash
cargo test -p nl-lexer
```

---

## 📚 Dokumentasi Lanjutan

- **[log_workflow.md](log_workflow.md)** – Rekod penuh perjalanan pembangunan dari awal sehingga kini.
- **[vision_process.md](vision_process.md)** – Visi strategik untuk menjadikan NEO Language asas kepada sistem pengendalian NEO OS sepenuhnya.
- **[Illustration of NL-Compiler.md](Illustration%20of%20NL-Compiler.md)** – Rajah dan penjelasan seni bina pengkompil.

---

## 🛠️ Teknologi Di Sebalik Tabir

- **Bahasa Pelaksana**: Rust
- **Analisis Leksikal**: Manual (tanpa penjana lekser)
- **Penghuraian**: *Recursive descent* dengan *Pratt parser* untuk ungkapan.
- **Penjanaan Kod**: C99 – untuk keserasian maksimum dengan GCC, Clang, MSVC.
- **Pengurusan Memori**: Dirancang menyokong pemilikan (seperti Rust) melalui analisis statik pada masa depan.

---

## 🚧 Pelan Hala Tuju (Roadmap)

- [x] Lekser & Penghurai asas
- [x] Penjanaan kod C (MVP)
- [x] Fungsi terbina `println`
- [ ] Fungsi terbina tambahan (`input`, `assert`, dll.)
- [ ] Sokongan jenis data eksplisit dan inferens jenis
- [ ] Penambahbaikan pengurusan ingatan
- [ ] *Backend* LLVM untuk prestasi optimum
- [ ] Pustaka standard NL
- [ ] Alat CLI `nl` yang lengkap (bina, uji, format)

---

## 📜 Lesen

Diedarkan di bawah [Lesen MIT](LICENSE). Lihat fail `LICENSE` untuk maklumat lanjut.

---

## 👥 Kolaborasi

Kami mengalu-alukan sumbangan! Sila ajukan isu atau *pull request* di repositori utama. Untuk perubahan besar, sila bentuk perbincangan dahulu melalui isu.

Bersama kita bangunkan bahasa untuk masa depan NEO OS. 🇲🇾
