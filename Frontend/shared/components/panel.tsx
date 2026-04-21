import type { PropsWithChildren } from 'react';

export function Panel({ children }: PropsWithChildren) {
  return (
    <section className="rounded-lg border border-gray-200 bg-white p-4 shadow-sm">
      {children}
    </section>
  );
}
