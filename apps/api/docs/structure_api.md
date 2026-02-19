# API Structure Guide (Patokan Tetap)

Dokumen ini adalah pedoman resmi struktur folder `apps/api` untuk pengembangan berikutnya.
Tujuan: struktur tetap sederhana (KISS), minim duplikasi (DRY), dan mudah scale.

## 1) Struktur Folder Paten
```txt
apps/api/
  docs/
    /postman
    structure_api.md
  migration/
    Cargo.toml
    src/
      main.rs
      lib.rs
      mYYYYMMDD_HHMMSS_<migration_name>.rs
  src/
    main.rs
    app_state.rs
    common/
      mod.rs
      error.rs
      pagination.rs
      auth_claims.rs
    config/
      mod.rs
      database.rs
    infrastructure/
      mod.rs
      db/
        mod.rs
        connection.rs
    modules/
      health/
        handler.rs
        dto.rs
      auth/
        handler.rs
        dto.rs
        service.rs
        repository.rs
        entity.rs
      todo/
        handler.rs
        dto.rs
        service.rs
        repository.rs
        entity.rs
  Cargo.toml
  Dockerfile
  .env.example
```

Catatan:
- `modules/*` dipakai per domain fitur.
- `migration/*` adalah single source of truth schema database.
- `common/*` dipakai untuk komponen lintas domain agar tidak duplikasi.

## 2) Tanggung Jawab Tiap Layer
- `main.rs`
  - bootstrap app (load env, init tracing, init db, build router).
- `app_state.rs`
  - shared state aplikasi (mis. `DatabaseConnection`).
- `config/*`
  - baca + validasi konfigurasi dari environment.
- `infrastructure/db/*`
  - pembuatan koneksi DB, pool options, util koneksi.
- `modules/<domain>/handler.rs`
  - HTTP boundary (Axum extractor/response), tanpa query DB langsung.
- `modules/<domain>/service.rs`
  - business rule/use-case.
- `modules/<domain>/repository.rs`
  - akses data via SeaORM/query builder.
- `modules/<domain>/entity.rs`
  - model entity SeaORM (jika dipisah per domain).
- `modules/<domain>/dto.rs`
  - request/response DTO.

## 3) Aturan Wajib (KISS + DRY)
- Handler tidak boleh berisi business logic kompleks.
- Service tidak boleh tahu detail HTTP (status code/extractor).
- Repository tidak boleh tahu concern HTTP.
- Query database hanya di repository (atau helper query infrastructure jika generic).
- Parsing env hanya di `config/*`.
- Tidak boleh ada duplikasi constant env di banyak file.
- Semua perubahan schema wajib lewat migration baru (jangan edit migration yang sudah applied).
- `main.rs` adalah composition root saja (bootstrap, wiring, router).
- `modules/*` dilarang import balik ke `main.rs` (arah dependency satu arah).
- Komponen generic lintas domain wajib ditempatkan di `common/*`.

## 4) Konvensi Naming
- File/module: `snake_case`.
- Struct/enum/trait: `PascalCase`.
- Function/variable: `snake_case`.
- Migration file: `mYYYYMMDD_HHMMSS_<name>.rs`.
- Nama index/constraint/FK harus eksplisit:
  - `idx_<table>_<columns>`
  - `fk_<child>_<parent>`
  - `chk_<table>_<rule>`

## 5) Routing Pattern
- Seluruh route didaftarkan terpusat di bootstrap router.
- Base prefix API: `/api/*`.
- Route per domain dikelompokkan prefix:
  - `/api/auth/*`
  - `/api/todos/*`
  - `/api/health`
- Hindari route declaration tersebar di banyak tempat tanpa registry pusat.

## 5.1 Dependency Direction (Wajib)
- Arah dependency yang diizinkan:
  - `main/config/infrastructure` -> `modules` + `common`
  - `modules/handler` -> `modules/service` + `common`
  - `modules/service` -> `modules/repository` + `common`
  - `modules/repository` -> `infrastructure/db` + `common`
- Arah dependency yang dilarang:
  - `modules/*` -> `main`
  - import silang antar domain untuk logic internal tanpa lewat contract yang jelas.

## 5.2 Shared Kernel (`src/common/`)
- Tujuan:
  - menaruh concern generic yang dipakai banyak domain.
- Contoh isi:
  - `error.rs` (error type + mapping konsisten),
  - `pagination.rs` (request/response pagination),
  - `auth_claims.rs` (claims/context user terautentikasi),
  - util validation generic.
- Batasan:
  - `common` tidak boleh berisi business rule domain spesifik.
  - jika logic sudah spesifik ke satu domain, pindahkan ke `modules/<domain>`.

## 6) Testing Minimum
- Unit test:
  - logic murni di `service`.
- Integration test:
  - endpoint HTTP + DB interaction utama.
- Migration test:
  - `up` dan `down` harus tervalidasi.

## 7) Do / Don’t
- Do:
  - tambah domain baru dalam `modules/<domain>`.
  - jaga dependency flow: `handler -> service -> repository`.
  - redaksi secret di log.
- Don’t:
  - query DB langsung di handler.
  - hardcode kredensial di source.
  - campur file migration dengan business logic.

## 8) Rule Perubahan Struktur
- Perubahan struktur folder inti wajib update dokumen ini di PR yang sama.
- Jika ada konflik, dokumen ini menjadi referensi utama sampai direvisi resmi.

> 18 Februari 2025
