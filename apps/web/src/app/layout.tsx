import type { Metadata } from 'next';
import { QueryProvider } from '@/providers/query-provider';
import type { ReactNode } from 'react';
import './globals.css';

export const metadata: Metadata = {
  title: 'Todolist Monorepo',
  description: 'Next.js frontend for Rust API monorepo'
};

type Props = {
  children: ReactNode;
};

export default function RootLayout({ children }: Props) {
  return (
    <html lang="en">
      <body>
        <QueryProvider>{children}</QueryProvider>
      </body>
    </html>
  );
}
