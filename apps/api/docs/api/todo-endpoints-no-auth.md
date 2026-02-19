# Todo API (Tanpa Auth, Fase Awal)

Dokumen ini menjelaskan endpoint CRUD Todo untuk fase tanpa auth.

## Ringkasan
- Base path: `/api/todos`
- Response envelope:
  - sukses: `{ "data": ... }`
  - gagal: `{ "error": { "code": "...", "message": "..." } }`
- Asumsi fase ini:
  - belum ada login/session.
  - API memakai `DEFAULT_USER_ID` dari env sebagai user default sementara.

## Endpoint

## 1) Create Todo
- Method: `POST`
- URL: `/api/todos`
- Body:
```json
{
  "title": "Belajar SeaORM",
  "desc": "CRUD todo tanpa auth"
}
```
- Success `201`:
```json
{
  "data": {
    "id": "d6c8df2c-6a74-4017-ae68-0bdd2617dc90",
    "user_id": "00000000-0000-0000-0000-000000000001",
    "title": "Belajar SeaORM",
    "desc": "CRUD todo tanpa auth",
    "iscompleted": false,
    "created_at": "2026-02-18T12:00:00+00:00",
    "updated_at": "2026-02-18T12:00:00+00:00"
  }
}
```

## 2) List Todo
- Method: `GET`
- URL: `/api/todos`
- Success `200`:
```json
{
  "data": [
    {
      "id": "d6c8df2c-6a74-4017-ae68-0bdd2617dc90",
      "user_id": "00000000-0000-0000-0000-000000000001",
      "title": "Belajar SeaORM",
      "desc": "CRUD todo tanpa auth",
      "iscompleted": false,
      "created_at": "2026-02-18T12:00:00+00:00",
      "updated_at": "2026-02-18T12:00:00+00:00"
    }
  ]
}
```

## 3) Get Todo Detail
- Method: `GET`
- URL: `/api/todos/:id`
- Success `200`:
```json
{
  "data": {
    "id": "d6c8df2c-6a74-4017-ae68-0bdd2617dc90",
    "user_id": "00000000-0000-0000-0000-000000000001",
    "title": "Belajar SeaORM",
    "desc": "CRUD todo tanpa auth",
    "iscompleted": false,
    "created_at": "2026-02-18T12:00:00+00:00",
    "updated_at": "2026-02-18T12:00:00+00:00"
  }
}
```

## 4) Update Todo
- Method: `PATCH`
- URL: `/api/todos/:id`
- Body (partial update):
```json
{
  "title": "Belajar SeaORM - updated",
  "iscompleted": true
}
```
- Success `200`:
```json
{
  "data": {
    "id": "d6c8df2c-6a74-4017-ae68-0bdd2617dc90",
    "user_id": "00000000-0000-0000-0000-000000000001",
    "title": "Belajar SeaORM - updated",
    "desc": "CRUD todo tanpa auth",
    "iscompleted": true,
    "created_at": "2026-02-18T12:00:00+00:00",
    "updated_at": "2026-02-18T12:05:00+00:00"
  }
}
```

## 5) Delete Todo
- Method: `DELETE`
- URL: `/api/todos/:id`
- Success `204`: no body.

## Status Code
- `201` created
- `200` success read/update
- `204` success delete
- `400` validation error
- `404` todo not found
- `500` internal database/server error

## Error Code
- `BAD_REQUEST`
  - contoh: title kosong, title terlalu panjang, desc terlalu panjang.
- `NOT_FOUND`
  - todo id tidak ditemukan untuk default user.
- `INTERNAL_ERROR`
  - error database atau error internal tak terduga.

## Catatan Validasi
- `title` wajib saat create, non-empty setelah trim, max 200 karakter.
- `desc` opsional, max 2000 karakter.
- `iscompleted` harus boolean jika dikirim.
