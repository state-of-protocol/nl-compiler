# Visi & Proses Membangunkan NEO Language ke Arah NEO OS Sepenuhnya

Dokumen ini menggariskan laluan strategik untuk mengubah NEO Language (NL) daripada bahasa pengaturcaraan sistem yang ringkas kepada sebuah sistem pengendalian moden yang berfungsi sepenuhnya – setanding dengan macOS, Ubuntu, atau Windows.


## 1. Tiga Fasa Utama Pembangunan

Berikut adalah tiga fasa besar yang akan dilalui oleh NEO Language dan ekosistemnya:

### Fasa 1: Pengkompil Pemula (Selesai ✅)
- **Penerangan:** Pengkompil NL dibina dalam Rust dan menghasilkan kod C sebagai perantaraan.
- **Output:** Aturcara pengguna ringkas yang dikompil melalui GCC/Clang/MSVC.
- **Status:** Selesai. Mampu menulis aturcara CLI dengan fungsi `println`, `input`, dan kawalan aliran.

### Fasa 2: Pengkompil Bertenaga Penuh & Self-Hosting (Akan Datang)
- **Penjana Kod LLVM:** Pengkompil NL akan menjana kod mesin terus melalui LLVM, tanpa bergantung kepada C.
- **Self-Hosting:** Pengkompil NL itu sendiri akan ditulis semula dalam NL. Ini membuktikan kematangan bahasa.
- **Runtime Library (`libnl`):** Pustaka asas untuk pengurusan memori, I/O, dan antara muka sistem.

### Fasa 3: Kernel NEO OS & Ekosistem Lengkap
- **Kernel Mikro:** Kernel pertama NEO OS ditulis dalam NL, dijalankan pada perkakasan sebenar atau mesin maya.
- **Pustaka Sistem:** Perpustakaan seperti `libnl-core` (libc gantian) yang menyokong pemacu peranti, sistem fail, dan protokol rangkaian.
- **Ruang Pengguna & GUI:** Shell, utiliti asas, pelayan grafik, dan pengurus tetingkap – semuanya dibina dalam NL.
- **Self-Hosting OS:** NEO OS boleh membangun dan mengkompil dirinya sendiri sepenuhnya di atas NL.


## 2. Laluan Realistik untuk Mencapai OS Penuh

Untuk menjadi seperti macOS/Ubuntu/Windows, projek ini mesti melalui pencapaian tambahan:

| Pencapaian | Penerangan |
|------------|-------------|
| **Pengkompil Stabil & Dioptimumkan** | Mampu menjana kod mesin berkualiti tinggi untuk pelbagai seni bina (x86_64, ARM64, RISC-V). |
| **Pustaka Runtime (`libnl`)** | Menyediakan fungsi asas: pengurusan memori, pengendalian rentetan, operasi fail, dan sokongan benang. |
| **Kernel Minimum (NEDO)** | Kernel mikro yang boleh boot, mengurus proses, dan menyokong pemacu papan kekunci/paparan ringkas. |
| **Alat Binaan & Pengurus Pakej** | Sistem `nlpkg` atau `nl-get` untuk memasang, mengemas kini, dan mengurus perisian NL. |
| **GUI & Persekitaran Desktop Asas** | Pelayan grafik ringkas, pustaka widget, dan pengurus tetingkap untuk interaksi visual. |
| **Ekosistem Aplikasi** | Penyunting teks, pelayar fail, emulator terminal, dan akhirnya penyemak imbas. |


## 3. Anggaran Masa & Tenaga

Ini adalah anggaran kasar berdasarkan projek sumber terbuka yang serupa:

- **1–2 tahun:** Pustaka runtime `libnl` + penjana LLVM + self-hosting awal.
- **2–4 tahun:** Kernel mikro pertama + bootloader + shell ringkas.
- **5–10 tahun:** GUI, rangkaian, sistem fail, dan ekosistem aplikasi yang mencukupi untuk penggunaan harian.

Tempoh ini boleh dipendekkan dengan sumbangan komuniti yang aktif dan tumpuan kepada seni bina modular.


## 4. Perjalanan Bertahap: Fokus Kecil, Kejayaan Besar

Pendekatan pembangunan akan sentiasa berperingkat:

1. **Prototaip Pantas** – Mulakan dengan ciri yang boleh diuji dalam beberapa minggu.
2. **Integrasi Berterusan** – Setiap komponen baru disepadukan dan diuji serta-merta.
3. **Dokumentasi Telus** – Setiap langkah dicatat dalam `log_workflow.md` untuk rujukan.
4. **Kitaran Maklum Balas** – Mengumpul maklum balas daripada pengguna awal sebelum memperluas skop.
5. **Rayakan Pencapaian Kecil** – Setiap fasa yang siap adalah asas kepada fasa berikutnya.


## 5. Satu Perjalanan, Bukan Satu Lompatan

Membangunkan sistem pengendalian penuh adalah maraton, bukan pecut. Projek NEO OS tidak perlu bersaing dengan Windows atau macOS dalam sekelip mata. Ia bermula sebagai:

- **Alat pendidikan** untuk memahami pengkompil, kernel, dan sistem pengendalian.
- **Platform eksperimen** untuk menguji idea baru dalam reka bentuk OS dan bahasa.
- **Ekosistem khusus** untuk komuniti yang menghargai kedaulatan teknologi dan bahasa tempatan.

Dengan masa dan dedikasi, ia boleh berkembang menjadi sistem pengendalian yang berdikari. Setiap baris kod NL yang ditulis hari ini adalah batu asas kepada masa depan itu.

---

*"Jangan pandang rendah kepada permulaan yang kecil. Setiap pokok oak bermula daripada biji benih."*

---

**Ditulis pada:** 8 Mei 2026  
**Oleh:** 0xAiman@state-of-protocol  
**Repositori:** [nl-compiler](https://github.com/state-of-protocol/nl-compiler)