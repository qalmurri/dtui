# DTUI

**DTUI** adalah proyek desktop berbasis terminal (*Text User
Interface/TUI*) untuk Linux. Tujuannya adalah menyediakan lingkungan
kerja CLI yang lebih teratur dan mudah digunakan, terutama pada
instalasi Linux minimal yang tidak menggunakan desktop grafis.

DTUI dikembangkan dengan fokus pada tiga prinsip utama:

1.  **Stabilitas** --- PTY, proses shell, window, workspace, dan
    pemulihan terminal harus bekerja dengan benar.
2.  **Efisiensi** --- input responsif, penggunaan CPU dan RAM rendah,
    serta dependensi seminimal mungkin.
3.  **Kemudahan penggunaan** --- shortcut konsisten, konfigurasi yang
    masuk akal, dan mudah dijalankan pada instalasi Linux CLI yang masih
    fresh.

> DTUI masih dalam tahap pengembangan. Fitur dan perilaku dapat berubah
> seiring proses pematangan fondasi.

## Tujuan Proyek

DTUI ditujukan untuk pengguna yang ingin bekerja langsung dari Linux TTY
tanpa harus menjalankan lingkungan desktop grafis seperti Xorg atau
Wayland.

Fokus pengembangannya bukan menambahkan fitur sebanyak mungkin,
melainkan membuat pengalaman CLI yang sederhana, ringan, stabil, dan
praktis.

## Prinsip Pengembangan

### 1. Stabilitas

-   Mengelola sesi PTY dan proses shell dengan benar.
-   Menjaga perilaku window dan workspace tetap konsisten.
-   Menangani penutupan window dan proses terminal tanpa meninggalkan
    proses yang tidak diperlukan.
-   Memastikan terminal dipulihkan dengan benar ketika DTUI keluar.
-   Menguji perilaku aplikasi sebelum menambahkan fitur baru.

### 2. Efisiensi

-   Menjaga respons input tetap cepat.
-   Menghindari pekerjaan berulang yang tidak diperlukan pada event loop
    dan proses rendering.
-   Mengupayakan penggunaan CPU dan RAM yang rendah, termasuk saat tidak
    ada aktivitas.
-   Menjaga jumlah dependensi tetap sesuai kebutuhan proyek.

### 3. Kemudahan Penggunaan

-   Menyediakan shortcut yang konsisten dan mudah dipelajari.
-   Menjaga konfigurasi tetap jelas dan mudah diubah.
-   Mempertimbangkan kondisi instalasi Linux minimal.
-   Tidak mengasumsikan semua aplikasi eksternal sudah tersedia.
-   Memberikan pesan yang jelas ketika suatu aplikasi atau kebutuhan
    sistem belum tersedia.

## Teknologi

Teknologi utama yang digunakan saat ini:

-   **Rust** --- bahasa pemrograman utama.
-   **Ratatui** --- antarmuka terminal.
-   **Crossterm** --- input keyboard dan pengelolaan terminal.
-   **portable-pty** --- pembuatan dan pengelolaan pseudo-terminal.
-   **vt100** --- pemrosesan output terminal untuk ditampilkan di dalam
    window.
-   **TOML** --- format konfigurasi.
-   **Lua (opsional)** --- dukungan scripting yang direncanakan melalui
    `mlua`.

Daftar tersebut menggambarkan teknologi proyek saat ini; implementasi
dan kebutuhan dependensi dapat berubah selama pengembangan.

## Status Fitur

  -----------------------------------------------------------------------
  Area                                Status
  ----------------------------------- -----------------------------------
  Antarmuka TUI dasar                 Sudah tersedia

  Window manager dasar                Sudah tersedia

  Workspace                           Sudah tersedia

  Terminal melalui PTY                Sudah terintegrasi; masih perlu
                                      pengujian stabilitas

  Penutupan window saat shell keluar  Sudah tersedia

  Konfigurasi TOML                    Fondasi tersedia; terus dimatangkan

  Pemulihan session                   Fondasi tersedia; perlu pengujian
                                      pada berbagai kondisi

  Penggunaan langsung pada Linux TTY  Menjadi sasaran pengujian dan
                                      pematangan

  Scripting Lua                       Dukungan direncanakan/opsional;
                                      tingkat integrasi dapat berubah
  -----------------------------------------------------------------------

Status di atas bukan jaminan bahwa semua kondisi sudah teruji.
Stabilitas dan kompatibilitas tetap perlu diverifikasi pada lingkungan
penggunaan nyata.

## Persyaratan

Untuk pengembangan, siapkan:

-   Linux dengan toolchain Rust dan Cargo.
-   Compiler serta tool sistem yang diperlukan untuk membangun
    dependensi Rust.
-   Terminal yang mendukung fitur yang digunakan Crossterm.

DTUI ditujukan untuk lingkungan Linux CLI. Namun, persyaratan runtime
minimum dan kompatibilitas lintas distribusi masih perlu ditetapkan
melalui pengujian.

## Menjalankan DTUI

Setelah repository tersedia secara lokal dan toolchain Rust terpasang:

``` bash
cargo run
```

Untuk memeriksa kompilasi dan pengujian:

``` bash
cargo fmt --check
cargo check
cargo test
```

Untuk menjalankan build release:

``` bash
cargo build --release
```

Perintah di atas merupakan alur pengembangan Cargo. Cara instalasi
sistem, autostart saat login, dan integrasi dengan Linux TTY akan
ditentukan setelah pengujian stabilitasnya matang.

## Fokus Pengujian

Sebelum menambahkan fitur besar, prioritaskan pengujian berikut:

-   Membuka beberapa window terminal secara bersamaan.
-   Mengetik cepat dan memeriksa respons input.
-   Menjalankan shell interaktif serta aplikasi seperti `vim`, `less`,
    dan `top` jika tersedia.
-   Mengubah ukuran dan memindahkan window.
-   Menutup satu terminal tanpa mengganggu terminal lainnya.
-   Memastikan proses shell ditangani dengan benar saat window ditutup.
-   Berpindah workspace dan memeriksa fokus window.
-   Keluar dari DTUI dan memastikan terminal kembali dalam kondisi
    normal.
-   Memulai ulang DTUI dan memeriksa pemulihan session.
-   Mengamati penggunaan CPU dan RAM ketika DTUI idle maupun aktif.

Aplikasi pengujian seperti `vim`, `less`, dan `top` merupakan contoh
alat uji, bukan dependensi wajib DTUI.

## Arah Pengembangan

Pengembangan DTUI dilakukan bertahap:

1.  Mematangkan stabilitas PTY, proses shell, window, workspace, dan
    pemulihan terminal.
2.  Mengoptimalkan respons input, rendering, penggunaan sumber daya, dan
    dependensi.
3.  Merapikan shortcut, konfigurasi, pesan kesalahan, dan pengalaman
    penggunaan.
4.  Menguji DTUI pada Linux TTY dan instalasi Linux minimal.
5.  Menentukan fitur berikutnya berdasarkan kebutuhan nyata, bukan
    menambah kompleksitas tanpa alasan.

## Prinsip yang Dijaga

-   **Sederhana lebih baik daripada kompleks tanpa kebutuhan.**
-   **Stabilitas didahulukan daripada penambahan fitur.**
-   **Efisiensi adalah persyaratan utama, bukan optimasi belakangan.**
-   **Lingkungan Linux minimal harus menjadi pertimbangan sejak awal.**
-   **Fitur baru harus memberi manfaat nyata bagi pengguna.**

## Kontribusi

Untuk saat ini, prioritas utama adalah mematangkan fondasi dan menguji
perilaku DTUI. Perubahan yang kecil, terarah, dan mudah diuji lebih
diutamakan daripada refaktor besar tanpa kebutuhan yang jelas.

## Lisensi

Lisensi proyek belum dicantumkan di README ini. Tambahkan informasi
lisensi setelah lisensi repository DTUI ditetapkan.
