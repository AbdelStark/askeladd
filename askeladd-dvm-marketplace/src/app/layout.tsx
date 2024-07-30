import {  NostrProvider } from "@/context/NostrContext";
import "./globals.css";
import { Inter } from "next/font/google";

const inter = Inter({ subsets: ["latin"] });

export const metadata = {
  title: "Askeladd DVM Marketplace",
  description: "A decentralized marketplace for Zero-Knowledge Proofs",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <NostrProvider>
        <body className={inter.className}>{children}</body>

      </NostrProvider>
    </html>
  );
}
