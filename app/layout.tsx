import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Epic Games Launcher",
  description: "Modern Epic Games Launcher concept built with Next.js & Tailwind",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="zh-CN">
      <body className="bg-launcher-bg text-launcher-text overflow-hidden">
        {children}
      </body>
    </html>
  );
}
