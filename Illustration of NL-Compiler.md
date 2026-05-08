# Ilustrasi Seni Bina Pengkompil NEO Language (NL)

Rajah di bawah menunjukkan aliran penuh bagaimana kod sumber NEO Language (NL) diproses sehingga menjadi aturcara boleh laksana.

Kod Sumber NL (*.nl)
│
▼
┌────────────────────────────────────┐
│ PENGKOMPIL NL │
│ (Ditulis dalam Rust) │
│ │
│ ● Lekser (nl-lexer) │
│ ● Penghurai (nl-parser) │
│ ● Semantik (nl-semantic) │
│ ● Penjana Kod (nl-codegen) │
└────────────────┬───────────────────┘
│
▼
Kod C (program.c)
│
▼
┌────────────────────────────────────┐
│ PENGKOMPIL C LUARAN │
│ (GCC / Clang / MSVC) │
└────────────────┬───────────────────┘
│
▼
Aturcara Boleh Laksana
(hello.exe / a.out)
│
▼
Program berjalan bebas
tanpa kebergantungan kepada Rust

*Rajah 1: Seni Bina Semasa Pengkompil NL*

---

## Penjelasan Langkah Demi Langkah

1. **Kod Sumber NL (`*.nl`)**  
   Aturcara ditulis dalam bahasa NEO Language dengan sintaks ringkas dan kata kunci Melayu seperti `biar`, `jika`, `sementara`.

2. **Pengkompil NL**  
   - Dibina sepenuhnya dengan **Rust**.  
   - Terdiri daripada beberapa komponen utama:  
     * **Lekser** – Menukar aksara kepada token.  
     * **Penghurai** – Membina Pokok Sintaks Abstrak (AST).  
     * **Penganalisis Semantik** – Menyemak kesahihan pemboleh ubah dan jenis.  
     * **Penjana Kod** – Menghasilkan kod C yang setara.  

3. **Kod C (Pertengahan)**  
   Keluaran pengkompil NL ialah fail `.c` yang mengandungi kod C tulen. Ini membolehkan kita memanfaatkan pengkompil C yang matang dan tersedia di mana-mana.

4. **Pengkompil C Luaran**  
   Kod C yang dijana diserahkan kepada pengkompil C seperti **GCC**, **Clang**, atau **MSVC** untuk dikompil dan dipautkan menjadi fail boleh laksana.

5. **Aturcara Boleh Laksana**  
   Program yang telah siap boleh dijalankan terus pada sistem pengendalian **tanpa memerlukan sebarang alat Rust**. Pengguna akhir hanya perlu pengkompil C untuk membina dari sumber, atau terus menggunakan binari yang telah dibina.

---

## Kelebihan Seni Bina Ini

- **Kebolehgunaan Semula Infrastruktur C** – Pustaka sistem dan protokol yang ditulis dalam C dapat digunakan terus dari NL.
- **Kesederhanaan** – Pengkompil NL tidak perlu menjana kod mesin secara terus; ia kekal fokus pada terjemahan bahasa.
- **Mudah Diselenggara** – Setiap modul (lekser, penghurai, penjana) boleh diuji dan dikemas kini secara berasingan.
- **Laluan ke LLVM/WebAssembly** – Dengan menambah penjana kod alternatif, NL boleh menyasarkan pelantar lain tanpa mengubah bahagian depan pengkompil.

---

*Nota: Pada masa depan, pengkompil NL dijangka menyokong penjanaan terus kepada LLVM IR untuk prestasi lebih tinggi, tetapi kod C akan terus disokong sebagai pilihan serba guna.*
