# Husky Explanation

Dokumen ini menjelaskan flow quality gate Git di project berdasarkan konfigurasi Husky saat ini.

## 1) Cara Husky Aktif

- Husky diaktifkan lewat script root `prepare`:
  - `prepare`: `husky`
- Artinya, setelah `pnpm install`, hook Git akan dipasang/di-refresh.

## 2) Hook yang Aktif

### `pre-commit`

File: `.husky/pre-commit`  
Command:

```bash
pnpm verify:api
pnpm lint-staged:web
pnpm typecheck:web
```

Flow check:
1. `pnpm verify:api` -> menjalankan test API Rust.
2. `pnpm lint-staged:web` -> menjalankan lint/format hanya untuk file web yang di-stage.
3. `pnpm typecheck:web` -> validasi tipe data TypeScript frontend.

Jika salah satu gagal, proses commit dibatalkan.

Detail `lint-staged` web:
- Config: `apps/web/.lintstagedrc.json`
- ESLint config: `apps/web/eslint.config.mjs`
- Prettier config: `apps/web/.prettierrc.json`
- Pattern & task:
  - `apps/web/**/*.{js,jsx,ts,tsx,mjs,cjs}`
    - `prettier --write`
    - `eslint --fix --max-warnings=0 --no-warn-ignored`
  - `apps/web/**/*.{json,md,css}`
    - `prettier --write`

---

### `commit-msg`

File: `.husky/commit-msg`  
Command:

```bash
pnpm commitlint --edit "$1"
```

Pesan commit divalidasi oleh `commitlint` berdasarkan aturan di `commitlint.config.ts`.

Rule utama:
- Format wajib: `type(scope): subject`
- `type` wajib lowercase dan harus dari daftar type yang diizinkan.
- `scope` wajib ada dan lowercase.
- `subject` wajib ada dan tidak boleh diakhiri titik.

Jika tidak valid, commit dibatalkan.

---

### `pre-push`

File: `.husky/pre-push`  
Command:

```bash
pnpm verify
```

Eksekusi script root:

```bash
pnpm test && pnpm build
```

Dengan Turborepo:
- `pnpm test` -> `turbo run test`
- `pnpm build` -> `turbo run build`

Jika test/build gagal, push dibatalkan.

## 3) Ringkasan Gate

- Saat commit:
  - `pre-commit` (API test + web staged lint/format + web typecheck)
  - `commit-msg` (validasi format commit)
- Saat push:
  - `pre-push` (full verify: test + build)

Tujuan: error penting tertangkap sedini mungkin tanpa membuat flow commit terlalu lambat.
