'use client';

import { useQuery } from '@tanstack/react-query';
import { z } from 'zod';

const helloResponseSchema = z.object({
  message: z.string()
});

type HelloResponse = z.infer<typeof helloResponseSchema>;
const apiBaseUrl =
  process.env.NEXT_PUBLIC_API_BASE_URL ?? 'http://localhost:8080/api';

async function fetchHelloMessage(): Promise<HelloResponse> {
  const response = await fetch(`${apiBaseUrl}/hello`);
  if (!response.ok) {
    throw new Error('Failed to fetch hello message from Rust API');
  }
  const payload = await response.json();
  return helloResponseSchema.parse(payload);
}

export default function HomePage() {
  const { data, isLoading, isError, error } = useQuery({
    queryKey: ['rust-hello'],
    queryFn: fetchHelloMessage
  });

  return (
    <main className="mx-auto flex min-h-screen w-full max-w-2xl flex-col justify-center gap-3 px-4">
      <h1 className="text-3xl font-bold tracking-tight">Monorepo Ready</h1>
      <p className="text-slate-600">
        Frontend Next.js mengambil data dari API Rust.
      </p>
      {isLoading && (
        <p className="rounded-md border border-slate-200 bg-slate-50 p-3 text-sm">
          Loading message from Rust API...
        </p>
      )}
      {isError && (
        <p className="rounded-md border border-red-200 bg-red-50 p-3 text-sm text-red-700">
          Error: {error.message}
        </p>
      )}
      {data && (
        <p className="rounded-md border border-emerald-200 bg-emerald-50 p-3 text-sm text-emerald-700">
          Message from Rust: {data.message}
        </p>
      )}
    </main>
  );
}
