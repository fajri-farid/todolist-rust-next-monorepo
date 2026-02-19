# Web Structure Guide (Patokan Tetap)

Dokumen ini adalah pedoman resmi struktur `apps/web` untuk aplikasi todolist sederhana (user POV only).
Fokus: simple, rapi, scalable secukupnya, tanpa over-engineering.

## 1) Struktur Folder Paten

```txt
apps/web/
  docs/
    structure-web.md
  public/
  src/
    app/
      layout.tsx
      page.tsx
      login/
        page.tsx
      register/
        page.tsx
      todos/
        page.tsx
      globals.css
      not-found.tsx
      error.tsx
    components/
      pages/
        home/
        login/
        register/
        todos/
      shared/
        ui/
        layout/
    lib/
      api-client.ts
      api/
        auth.api.ts
        todo.api.ts
      env.ts
      utils.ts
    providers/
      query-provider.tsx
      app-provider.tsx
    styles/
    hooks/            # optional
    types/
    validations/
    context/          # optional
    constants/
  next.config.ts
  tsconfig.json
  eslint.config.mjs
```

Catatan:

- Route aktif fase sekarang:
  - `/` (home)
  - `/login`
  - `/register`
  - `/todos` (halaman setelah login)
- `hooks` dan `context` hanya dibuat jika benar-benar dibutuhkan.
- `components/pages/*` untuk komponen spesifik halaman.
- `components/shared/*` untuk komponen reusable lintas halaman.

## 2) Prinsip Arsitektur

- Next.js App Router sebagai fondasi utama.
- Server Component default; Client Component hanya saat perlu interaktivitas.
- File `app/*/page.tsx` tetap tipis (composition + wiring), bukan tempat logic berat.
- Jangan tambah layer baru sebelum ada kebutuhan nyata.

## 3) Aturan Wajib (KISS + DRY)

- Semua call API lewat `lib/api-client.ts` (single gateway).
- Semua env parsing via `lib/env.ts`.
- Validasi input/form taruh di `validations/*`.
- Tipe global/shared taruh di `types/*`.
- Konstanta global/shared taruh di `constants/*`.
- Komponen shared tidak boleh mengandung logic domain spesifik halaman tertentu.
- Komponen/page tidak boleh `fetch` langsung ke backend; wajib lewat `lib/api/*.api.ts`.

## 4) Dependency Direction (Wajib)

- Arah dependency yang diizinkan:
  - `app/*` -> `components/*`, `providers/*`, `lib/*`, `types/*`, `validations/*`, `constants/*`
  - `components/pages/*` -> `components/shared/*`, `hooks/*`, `lib/*`, `types/*`
  - `lib/api/*.api.ts` -> `lib/api-client.ts`, `types/*`, `validations/*`
  - `components/shared/*` -> `lib/*`, `types/*`, `constants/*`
- Arah dependency yang dilarang:
  - `lib/*` import dari `components/*` atau `app/*`
  - `components/shared/*` import dari `components/pages/*`

## 5) Konvensi Naming

- Folder/file umum: `kebab-case`.
- React component: `PascalCase.tsx`.
- Hooks: `use-*.ts`.
- Constant file: `*.constants.ts` (opsional) atau grouped per domain.
- Zod schema file: `*.schema.ts`.
- Test file: `*.test.ts(x)`.

## 6) Components Rule

- `components/pages/<page-name>`:
  - hanya untuk kebutuhan page tersebut.
  - boleh direfactor ke shared jika dipakai >= 2 halaman.
- `components/shared/ui`:
  - atomik/reusable (button, input, modal, badge, dll).
- `components/shared/layout`:
  - komponen layout umum (navbar, footer, container, section shell).

## 7) State Strategy

- Server state/API cache: React Query (`providers/query-provider.tsx`).
- UI local state: state lokal komponen dulu.
- `context/*` hanya jika state perlu di-share lintas banyak page/component dan prop drilling sudah tidak layak.

## 8) API Layer Strategy (Best Practice)

- `lib/api-client.ts`:
  - satu gateway request global (base URL, header, error normalize).
- `lib/api/auth.api.ts`:
  - semua function endpoint auth (`login`, `register`, `logout`).
- `lib/api/todo.api.ts`:
  - semua function endpoint todo (`getTodos`, `createTodo`, `toggleTodo`, dll).
- Alasan:
  - konsistensi request dan error handling,
  - perubahan endpoint cukup di satu area,
  - mencegah duplikasi logic networking di komponen.

## 9) Styling Strategy

- `app/globals.css` untuk base/global reset.
- Style spesifik komponen tetap dekat dengan komponennya.
- Hindari style duplikat; ekstrak ke `shared/ui` jika reusable.

## 10) Do / Don't

- Do:
  - pisahkan komponen page-specific di `components/pages/*`.
  - simpan komponen reusable di `components/shared/*`.
  - akses backend lewat `lib/api-client.ts` + `lib/api/*.api.ts`.
  - jaga file `app/*/page.tsx` tetap tipis (composition only).
- Don't:
  - jangan `fetch` backend langsung di komponen/page.
  - jangan taruh helper acak di root `src/` tanpa kategori folder.
  - jangan import dari `components/pages/*` ke `components/shared/*`.
  - jangan over-abstraction untuk kebutuhan yang belum ada.

## 11) Rule Perubahan Struktur

- Perubahan struktur inti wajib update dokumen ini pada PR yang sama.
- Dokumen ini menjadi referensi utama implementasi frontend sampai direvisi resmi.

> 18 Februari 2026
