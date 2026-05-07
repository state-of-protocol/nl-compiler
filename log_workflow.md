# Log Kerja (Workflow) Projek NEO Language (NL) Compiler PART 1 [7 MAY 2026] 6.00 PM - 11.30 PM

Log ini merakam perjalanan pembangunan pengkompil NEO Language, daripada perbincangan konsep hingga kepada pengkompil berfungsi yang menolak kod ke GitHub.

---

## Fasa 0: Konsep & Perancangan Awal

**Perbincangan**:  
- Memutuskan untuk membina bahasa pengaturcaraan sistem bernama NEO Language (NL) untuk ekosistem NEO OS.  
- Inspirasi: Keselamatan Rust + kesederhanaan Go.  
- Kata kunci direka dalam Bahasa Melayu: `fn`, `biar`, `jika`, `lain`, `untuk`, `sementara`.  
- Lekser (tokenizer) ditulis tangan dalam Rust, menggunakan enum `TokenKind`.  
- Projek distrukturkan sebagai ruang kerja Cargo (workspace) untuk modulariti.

**Langkah**:  
- Mereka bentuk struktur projek: `crates/nl-lexer`, `nl-ast`, `nl-parser`, `nl-semantic`, `nl-codegen`, `nl-driver`.  
- Memilih format sumber: UTF-8, ASCII, JSON untuk metadata.

---

## Fasa 1: Lekser (Tokenizer) Awal

**Objektif**: Mengecam token asas: kata kunci, pengenal, integer, rentetan, operator.

**Fail**:  
- `crates/nl-lexer/src/token.rs` – definisi `TokenKind` dan `Token`.  
- `crates/nl-lexer/src/lexer.rs` – pelaksanaan `Lexer` dengan fungsi `tokenize()`.

**Ujian**:  
- Menggunakan `main.rs` untuk menguji tokenisasi terus.

**Isu**:  
- Tiada.

**Status**: ✅ Selesai (versi awal).

---

## Fasa 2: Pengembangan Lekser (Token Lengkap)

**Penambahan**:  
- Operator dua aksara: `<=`, `>=`, `==`, `!=`, `..`.  
- Simbol tambahan: `{`, `}`, `(`, `)`, `,`, `;`, `.`, `:`.  
- Penjejakan baris dan lajur untuk setiap token.

**Fail**:  
- `token.rs` dikemas kini dengan varian baru.  
- `lexer.rs` dikemas kini untuk mengendalikan semua token.

**Status**: ✅ Selesai.

---

## Fasa 3: Pokok Sintaks Abstrak (AST)

**Peti baru**: `nl-ast`

**Definisi**:  
- `Program`, `Function`, `Statement`, `Expression`, `BinaryOp`, `UnaryOp`, `Type`.  
- Varian `Statement`: Let, Assign, Expression, If, For, While, Return.  
- Varian `Expression`: Ident, IntLiteral, StringLiteral, Binary, Unary, Call.

**Fail**:  
- `crates/nl-ast/src/node.rs`  
- `crates/nl-ast/src/lib.rs`

**Status**: ✅ Selesai (versi asas).

---

## Fasa 4: Penghurai Rekursif Menurun (Parser)

**Peti baru**: `nl-parser`

**Pelaksanaan**:  
- `Parser` menggunakan kaedah *recursive descent* + Pratt parser untuk ungkapan.  
- Menyokong: fungsi, blok, `biar`, `jika`/`lain`, `untuk`, `sementara`, tugasan, panggilan fungsi.

**Fail**:  
- `crates/nl-parser/src/parser.rs` (logik penghuraian)  
- `crates/nl-parser/src/lib.rs`

**Isu Utama**:  
- **Ralat sintaks pertama**: Fungsi `expect()` gagal mengenal pasti `Ident("main")` sebagai `Ident` kerana perbandingan terus `token.kind == expected` menyemak nilai di dalam varian. Ini menyebabkan ralat `dijangka Ident(""), dijumpai Ident("main")`.  
- **Penyelesaian**: Menggunakan `std::mem::discriminant` untuk membandingkan hanya varian enum, bukan data. Ini memerlukan pengubahsuaian fungsi `expect()`.

**Status**: ✅ Selesai selepas pembetulan.

---

## Fasa 5: Penganalisis Semantik Asas

**Peti baru**: `nl-semantic`

**Fungsi**:  
- Menyemak kewujudan pemboleh ubah (pengikatan nama).  
- Membenarkan fungsi terbina `println`, `input`, `assert` tanpa diisytiharkan.  
- Mengembalikan jenis asas `Int`, `String`, `Void`, `Unknown`.

**Fail**:  
- `crates/nl-semantic/src/resolver.rs`  
- `crates/nl-semantic/src/lib.rs`

**Isu**:  
- Tiada isu besar.

**Status**: ✅ Selesai.

---

## Fasa 6: Penjana Kod C

**Peti baru**: `nl-codegen`

**Perjalanan**:  
- Versi awal hanya menterjemah terus kepada C tanpa fungsi terbina.  
- Selepas ujian `hello.nl`, berjaya menghasilkan kod C tetapi tiada output (kerana tiada fungsi cetak).  
- **Penambahan `println`**:  
   - Pertama, cuba menjana `printf` dengan format string dan `%d`.  
   - Isu: kod C yang dijana menyebabkan ralat `newline in constant` dan `syntax error`.  
   - **Penyelesaian**: Pembetulan `gen_expr` untuk `println` dengan membina format string yang betul, menggabungkan literal rentetan dan `%d`, serta menambah `\n` dengan betul.
- Penambahan `input` dan `assert` sebagai fungsi terbina kemudian (dirancang dalam fasa seterusnya).

**Fail**:  
- `crates/nl-codegen/src/codegen.rs`  
- `crates/nl-codegen/src/lib.rs`

**Ujian**:  
- Selepas pembetulan, `examples/hello2.nl` dengan `println("Hello, NEO Language! x = ", x)` berjaya dikompil dengan `cl` dan mengeluarkan output yang betul.

**Status**: ✅ Selesai (berfungsi dengan `println`).

---

## Fasa 7: CLI & Integrasi

**Peti**: `nl-driver`

**Peringkat Awal**:  
- Program `main()` mudah yang menerima fail `.nl`, menjalankan lekser, penghurai, analisis semantik, dan penjana kod ke `stdout`.

**Peningkatan**:  
- Akan datang: menggunakan `clap` untuk sub-arahan `build`, `run`, `test`.

**Fail**:  
- `crates/nl-driver/src/main.rs`

**Isu**:  
- Tiada isu dalam fasa ini.

**Status**: ✅ Selesai versi minimum.

---

## Fasa 8: Persediaan Persekitaran & Pemasangan Alat

**Isu Utama**:  
- `cargo` tidak dikenali di PowerShell (Windows).  
  - **Penyebab**: Rust belum dipasang.  
  - **Penyelesaian**: Pasang Rust melalui `rustup.rs`.  
- Visual C++ Build Tools diperlukan untuk `link.exe`.  
  - **Isu**: `link.exe` versi x86 ditemui, tetapi Rust (target x64) memerlukan `link.exe` x64.  
  - **Penyelesaian**: Memasang beban kerja "Desktop development with C++" melalui Visual Studio Build Tools (dengan komponen x64).  
  - Mengarahkan `PATH` secara manual ke direktori `Hostx64/x64`.  
- Ralat `'Get-ChildItem' is not recognized` ketika cuba mencari `link.exe` dari Command Prompt.  
  - **Penyebab**: Menggunakan CMD bukannya PowerShell.  
  - **Penyelesaian**: Beralih ke PowerShell untuk arahan tersebut.

**Status**: ✅ Selesai – alat kompilasi berfungsi.

---

## Fasa 9: Pengurusan Versi & GitHub

**Langkah**:  
1. Memulakan repo Git tempatan (`git init`).  
2. Mencipta `.gitignore` untuk mengecualikan direktori `target/`, fail objek, dan binari.  
3. Melakukan komit pertama dengan semua kod sumber.  
4. Mencipta repositori di GitHub (`state-of-protocol/nl-compiler`).  
5. Isu pengesahan semasa `git push`: "Password authentication is not supported".  
   - **Penyelesaian**: Menggunakan Token Akses Peribadi (PAT) dan menyimpannya dalam URL remote.  
6. Selepas berjaya tolak, halaman GitHub menunjukkan repositori kosong.  
   - **Sebab**: Tiada komit pada cawangan `main` atau `master`.  
   - **Penyelesaian**: Melakukan `git commit` dahulu, kemudian `git push -u origin main`.  
7. Setelah berjaya, pembersihan repositori: mengalih keluar fail binari, `.obj`, `.c` daripada penjejakan menggunakan `git rm --cached`.  
   - Kemas kini `.gitignore` untuk mengabaikan fail binaan pada masa depan.

**Status**: ✅ Repositori GitHub bersih dengan struktur projek sahaja.

---

## Fasa 10: Dokumentasi & Penyediaan Awal Seterusnya

**Penambahan**:  
- `README.md` profesional dalam Bahasa Melayu, menerangkan visi, prasyarat, cara membina, contoh, dan pelan hala tuju.  
- Fail `log_workflow.md` ini untuk rakaman perjalanan projek.

**Status**: ✅ Baru selesai.

---

## 🔜 Pelan Seterusnya (Roadmap)

1. **Fungsi terbina tambahan**: `input`, `assert`, manipulasi rentetan.  
2. **Sistem jenis eksplisit**: Sokongan `string`, `bool`, dan anotasi jenis (`: int`).  
3. **Alat CLI yang lebih kaya**: `nl build`, `nl run`, `nl test` menggunakan `clap`.  
4. **Penjana kod alternatif**: LLVM IR / WASM.  
5. **Pustaka Standard NL**: Modul `io`, `math`, `os`.  

---

## 🔧 Teknologi & Alatan

- **Bahasa**: Rust  
- **Sistem Binaan**: Cargo  
- **Kawalan Versi**: Git + GitHub  
- **Penyunting**: VS Code / Notepad++  
- **Pengkompil C luaran**: Microsoft C/C++ (Windows) / GCC (Linux)  

---

## 🤝 Penyumbang

- Pembangun Utama: @state-of-protocol

---

*Log ini akan dikemas kini secara berkala seiring kemajuan projek.*