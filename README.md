# Todolist Rust + Next Monorepo

Monorepo awal untuk:
- `apps/web`: Next.js frontend
- `apps/api`: Rust API (Axum)

## Prasyarat Minimum

- Node.js `>= 20`
- pnpm `>= 10`
- Rust toolchain `>= 1.80` (`rustup`, `cargo`, `rustc`)
- Git

## Environment

- Web default URL: `http://localhost:3000`
- API default URL: `http://localhost:8080`
- Endpoint health: `GET /health`
- Endpoint demo: `GET /hello`

Setup `.env`:

1. Copy `apps/web/.env.example` -> `apps/web/.env`
2. Copy `apps/api/.env.example` -> `apps/api/.env`
3. Ganti semua placeholder (`your_..._here`) dengan value environment kamu.

## Quick Start

1. Install dependency:
   ```bash
   pnpm install
   ```
2. Jalankan web + api bersamaan (Turbo):
   ```bash
   pnpm dev
   ```
3. Jalankan migration DB API (SeaORM):
   ```bash
   pnpm db:migrate:up:api
   ```
4. Validasi setup awal:
   ```bash
   pnpm verify
   ```

## Run Modes

- Development (web + api):
  ```bash
  pnpm dev
  ```
- Production (web + api):
  ```bash
  pnpm build
  pnpm start
  ```
- Docker (web + api + postgres):
  ```bash
  pnpm docker:up
  ```

## Docker Quick Tutorial

1. Build + start semua service:
   ```bash
   pnpm docker:up
   ```
2. Cek status:
   ```bash
   pnpm docker:ps
   ```
3. Buka aplikasi:
   - `http://localhost:3000`
4. Kalau sudah selesai:
   ```bash
   pnpm docker:down
   ```
5. Besok mau lanjut lagi:
   ```bash
   pnpm docker:up
   ```

## Command Penting

- Test backend:
  ```bash
  pnpm test:api
  ```
- Migration API:
  ```bash
  pnpm db:migrate:up:api
  pnpm db:migrate:down:api
  pnpm db:migrate:fresh:api
  ```
- Lint frontend:
  ```bash
  pnpm lint:web
  ```
- Typecheck frontend:
  ```bash
  pnpm typecheck:web
  ```
- Format frontend:
  ```bash
  pnpm format:web
  ```
- Cek format frontend:
  ```bash
  pnpm format:check:web
  ```
- Build frontend:
  ```bash
  pnpm build:web
  ```
- Jalankan semua task `test` via Turbo:
  ```bash
  pnpm test
  ```
- Jalankan semua task `build` via Turbo:
  ```bash
  pnpm build
  ```
- Jalankan mode production per app:
  ```bash
  pnpm start:web
  pnpm start:api
  ```
- Jalankan stack Docker:
  ```bash
  pnpm docker:up
  pnpm docker:ps
  pnpm docker:restart
  pnpm docker:logs
  pnpm docker:down
  pnpm docker:reset
  ```

## Quality Gate (Husky)

- `pre-commit`:
  - `pnpm verify:api`
  - `pnpm lint-staged:web`
  - `pnpm typecheck:web`
- `commit-msg`: `commitlint` (`commitlint.config.ts`)
- `pre-push`: `pnpm verify` (API test + web build wajib sukses)

Format commit wajib:

```txt
type(scope): subject
```

Contoh valid:

```txt
feat(api): add hello endpoint
fix(web): show api error state
docs(readme): update quick start
```

Contoh invalid:

```txt
update code
feat: add endpoint
Fix(API): wrong casing
```

Dokumentasi setup awal detail ada di `docs/setup-local.md`.
Dokumentasi Docker detail ada di `docs/docker-guide.md`.
Panduan kontribusi ada di `CONTRIBUTING.md`.
