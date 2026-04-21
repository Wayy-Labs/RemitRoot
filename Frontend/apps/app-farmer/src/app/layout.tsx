import './globals.css';
import type { Metadata } from 'next';
import type { ReactNode } from 'react';

export const metadata: Metadata = {
  title: 'RemitRoot Farmer',
  description: 'Farmer progressive web app for RemitRoot',
  manifest: '/manifest.json',
  themeColor: '#065f46'
};

export default function RootLayout({ children }: { children: ReactNode }) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
