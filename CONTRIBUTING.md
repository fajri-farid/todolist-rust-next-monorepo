# Contributing Guide

## 1) Branching

- Buat branch dari `main` dengan pola:
  - `feat/<scope>-<short-desc>`
  - `fix/<scope>-<short-desc>`
  - `chore/<scope>-<short-desc>`

Contoh:
- `feat/api-hello-endpoint`
- `fix/web-loading-state`

## 2) Development Flow

1. Install dependency:
   ```bash
   pnpm install
   ```
2. Jalankan environment dev:
   ```bash
   pnpm dev
   ```
3. Pastikan quality gate lokal lolos:
   ```bash
   pnpm verify
   ```
4. Untuk perubahan frontend, jalankan check tambahan:
   ```bash
   pnpm check:web
   pnpm format:check:web
   ```

## 3) Commit Convention

Project ini memakai Conventional Commits via `commitlint`.

Format wajib:

```txt
type(scope): subject
```

Contoh valid:
- `feat(api): add hello endpoint`
- `fix(web): handle rust api error`
- `docs(readme): clarify production run`

Contoh invalid:
- `update api`
- `feat: add endpoint`
- `Fix(API): wrong case`

## 4) Pull Request Checklist

Sebelum membuka PR, pastikan:
- Perubahan relevan dengan scope PR.
- `pnpm verify` sukses.
- Tidak ada perubahan file generated yang tidak perlu.
- Dokumentasi diperbarui jika ada perubahan command/alur.
- PR description menjelaskan:
  - tujuan perubahan
  - file utama yang diubah
  - cara verifikasi

## 5) Notes

- Husky hooks aktif otomatis lewat script `prepare` saat `pnpm install`.
- `pre-commit` menjalankan:
  - test API
  - lint staged file web (prettier + eslint autofix)
  - typecheck web
- `pre-push` menjalankan `pnpm verify`.
