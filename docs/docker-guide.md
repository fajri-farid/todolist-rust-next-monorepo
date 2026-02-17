# Docker Guide

Panduan menjalankan project ini dengan Docker Compose (`web` + `api` + `postgres`).

## 0) Quick Start Pemula (Disarankan)

Kalau kamu baru pertama kali pakai Docker di project ini, jalankan urutan ini:

1. Start stack:
   ```bash
   pnpm docker:up
   ```
2. Cek semua service sudah `Up`:
   ```bash
   pnpm docker:ps
   ```
3. Buka web:
   - `http://localhost:3000`
4. Selesai kerja, matikan:
   ```bash
   pnpm docker:down
   ```
5. Lanjut lagi lain waktu:
   ```bash
   pnpm docker:up
   ```

## 1) Prasyarat

- Docker Desktop aktif
- Docker Compose v2 (umumnya sudah include di Docker Desktop)

## 2) Service yang Dijalankan

- `web`: Next.js (`localhost:3000`)
- `api`: Rust Axum (`localhost:8080`)
- `db`: PostgreSQL (`localhost:5432`)

## 3) Jalankan Stack Docker

Dari root repo:

```bash
pnpm docker:up
```

Atau manual:

```bash
docker compose up --build -d
```

Catatan prompt/output ringkas:
- `-d` artinya container jalan di background.
- Saat sukses, kamu akan lihat container `todolist-web`, `todolist-api`, `todolist-db` berstatus running/healthy.
- Jika proses berhenti dengan `failed to solve` atau `ERROR`, berarti build belum sukses (lihat section Troubleshooting di bawah).

## 4) Cek Status dan Log

- Cek status service:
  ```bash
  pnpm docker:ps
  ```
- Arti status umum:
  - `Up`: container sedang jalan
  - `healthy`: healthcheck lulus (khusus service yang punya healthcheck)
  - `Exited`: container berhenti (lihat logs untuk error)

- Lihat log semua service (follow):
  ```bash
  pnpm docker:logs
  ```
- Lihat log service tertentu:
  ```bash
  docker compose logs -f web
  docker compose logs -f api
  docker compose logs -f db
  ```

## 5) Verifikasi Service

- Cek status:
  ```bash
  pnpm docker:ps
  ```
- Cek log:
  ```bash
  pnpm docker:logs
  ```
- Health API:
  ```bash
  curl http://localhost:8080/health
  ```
- Web:
  - `http://localhost:3000`

## 6) Restart, Stop, dan Delete

- Restart semua service:
  ```bash
  pnpm docker:restart
  ```
- Restart service tertentu:
  ```bash
  docker compose restart web
  docker compose restart api
  docker compose restart db
  ```

- Stop dan hapus container/network:
  ```bash
  pnpm docker:down
  ```
- Reset total (hapus container + network + volume DB):
  ```bash
  pnpm docker:reset
  ```

Ringkasan perintah:
- `down`: aman untuk stop sementara, data DB tetap ada.
- `reset`: hapus semuanya termasuk data DB.

## 7) Konfigurasi Environment

Compose mengambil base config dari:
- `apps/web/.env`
- `apps/api/.env`

Lalu beberapa env di-override oleh `docker-compose.yml` untuk konteks container:
- API bind host: `0.0.0.0`
- DB URL API: host `db` (nama service internal docker network)

## 8) Alert yang Harus Dihindari

- Jangan pakai `docker compose down -v` kalau tidak ingin kehilangan data PostgreSQL.
- Jangan ubah `DATABASE_URL` ke `localhost` di konteks container API.
  - Di container, host DB harus `db` (nama service Compose), bukan `localhost`.
- Jika `web` tidak bisa call API, pastikan:
  - `api` status `Up`
  - env `NEXT_PUBLIC_API_BASE_URL` benar (`http://localhost:8080` untuk akses dari browser host)
- Jika port bentrok (`3000`, `8080`, `5432`), hentikan proses lokal lain dulu atau ubah mapping port.
- Jika build aneh karena cache, jalankan:
  ```bash
  docker compose build --no-cache
  ```

## 9) Troubleshooting Cepat

- `web` tidak kebuka:
  - cek `pnpm docker:ps`
  - cek `pnpm docker:logs`
- API error CORS:
  - cek `WEB_ORIGIN` di `apps/api/.env`
  - pastikan nilainya `http://localhost:3000`
- API tidak bisa connect DB:
  - pastikan `db` status `healthy`
  - pastikan `DATABASE_URL` untuk container API pakai host `db`
- Port sudah dipakai:
  - tutup proses lokal di port `3000`, `8080`, atau `5432`
  - atau ubah mapping port di `docker-compose.yml`

## 10) Catatan Penting

- Saat dijalankan di browser host, frontend tetap akses API lewat `http://localhost:8080`.
- Service database bisa diakses dari host lewat `localhost:5432`.
