# Setup Lokal Awal (Yang Benar-Benar Dipakai Sekarang)

Dokumen ini khusus tahap awal supaya project bisa langsung jalan lokal.
Hal yang belum dipakai sekarang (misalnya Docker) sengaja belum diaktifkan.

## 1) Prasyarat Wajib

Install tools ini:
- Node.js `>= 20`
- pnpm `>= 10`
- Rust toolchain `>= 1.80` (`rustup`, `cargo`, `rustc`)
- Git

Cek versi:

```bash
node -v
pnpm -v
cargo -V
rustc -V
```

## 2) Struktur Minimum yang Dipakai

```txt
.
|-- apps/
|   |-- web/    # Next.js
|   `-- api/    # Rust Axum
|-- docs/
|   `-- setup-local.md
|-- package.json
|-- turbo.json
`-- pnpm-workspace.yaml
```

## 3) Install Dependency Project

Dari root repo:

```bash
pnpm install
```

Catatan:
- Command ini menginstall dependency frontend workspace `apps/web`.
- Dependency Rust akan di-resolve otomatis saat menjalankan command API (`pnpm dev:api`, `pnpm test:api`, atau `cargo run`, `cargo test`).

## 4) Environment Default

- Frontend: `http://localhost:3000`
- API: `http://localhost:8080`
- Health check API: `GET /health`
- Demo endpoint API: `GET /hello`

Setup file `.env`:

1. Copy file web:
   ```powershell
   Copy-Item apps/web/.env.example apps/web/.env
   ```
2. Copy file api:
   ```powershell
   Copy-Item apps/api/.env.example apps/api/.env
   ```
3. Edit kedua file `.env` dan ganti semua placeholder (`your_..._here`) dengan nilai environment lokal kamu.

Variabel yang dipakai:
- Web (`apps/web/.env`):
  - `NEXT_PUBLIC_API_BASE_URL`
- API (`apps/api/.env`):
  - `API_HOST`
  - `API_PORT`
  - `WEB_ORIGIN` (dipakai CORS allow-origin)
  - `RUST_LOG`

## 5) Menjalankan Backend Rust

Dari root repo:

```bash
pnpm dev:api
```

Atau langsung:

```bash
cargo run --manifest-path apps/api/Cargo.toml
```

Verifikasi backend:

```bash
curl http://localhost:8080/health
```

Respons yang diharapkan:

```json
{"status":"ok"}
```

## 6) Menjalankan Frontend Next.js

Dari root repo:

```bash
pnpm dev:web
```

Buka:
- `http://localhost:3000`

Menjalankan web + api sekaligus dengan Turbo:

```bash
pnpm dev
```

## 7) Menjalankan Mode Production

Build semua app dulu:

```bash
pnpm build
```

Jalankan web + api production:

```bash
pnpm start
```

Jalankan per app (opsional):

```bash
pnpm start:web
pnpm start:api
```

## 8) Command Validasi Awal

Unit test backend:

```bash
pnpm test:api
```

Lint frontend:

```bash
pnpm lint:web
```

Typecheck frontend:

```bash
pnpm typecheck:web
```

Format frontend:

```bash
pnpm format:web
```

Cek format frontend:

```bash
pnpm format:check:web
```

Build frontend:

```bash
pnpm build:web
```

Validasi full (yang dipakai juga oleh hook `pre-push`):

```bash
pnpm verify
```

Menjalankan semua test workspace via Turbo:

```bash
pnpm test
```

Menjalankan semua build workspace via Turbo:

```bash
pnpm build
```

## 9) Git Hook (Husky)

Husky sudah disetup di project ini.

- Saat `pnpm install`, script `prepare` akan mengaktifkan hook.
- Hook `pre-commit` menjalankan:

```bash
pnpm verify:api
pnpm lint-staged:web
pnpm typecheck:web
```

- Hook `commit-msg` memvalidasi format pesan commit:

```txt
type(scope): subject
```

Contoh valid:

```txt
feat(api): add hello endpoint
fix(web): handle api error state
docs(readme): update setup docs
chore(ci): enforce strict hooks
```

Contoh invalid:

```txt
update readme
fix: missing scope
Feat(API): wrong case
```

Aturannya ada di:
- `commitlint.config.ts`

- Hook `pre-push` menjalankan verifikasi ketat (wajib lolos sebelum push):

```bash
pnpm verify
```

## 10) Troubleshooting Cepat

- Jika `pnpm install` gagal karena network, pastikan akses ke `registry.npmjs.org` terbuka.
- Jika `cargo run` gagal download crate, cek koneksi ke `crates.io`.
- Jika port bentrok:
  - Web: ubah port di script `apps/web/package.json`
  - API: ubah port di `apps/api/src/main.rs`

## 11) Referensi Tambahan

- Guide kontribusi: `CONTRIBUTING.md`
