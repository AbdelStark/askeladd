"use client";

import { ASKELADD_RELAY } from '@/constants/relay';
import NDK, { NDKPrivateKeySigner } from '@nostr-dev-kit/ndk';
import { createContext, useContext, useEffect, useState } from 'react';
import dotenv from "dotenv";
import { SimplePool } from 'nostr-tools';
dotenv.config();

export type NostrContextType = {
  ndk: NDK;
  pool: SimplePool
};

export const NostrContext = createContext<NostrContextType | null>(null);

export const NostrProvider: React.FC<React.PropsWithChildren> = ({ children }) => {
  const [privateKey, setPrivateKey] = useState<string | undefined>(process.env.NEXT_PUBLIC_DEFAULT_NOSTR_USER_SK)
  const pool = new SimplePool();

  const [ndk, setNdk] = useState<NDK>(
    new NDK({
      explicitRelayUrls: ASKELADD_RELAY,
    }),
  );

  useEffect(() => {
    const newNdk = new NDK({
      explicitRelayUrls: ASKELADD_RELAY,
      signer: privateKey ? new NDKPrivateKeySigner(privateKey) : undefined,
    });

    newNdk.connect().then(() => {
      setNdk(newNdk);
    });
  }, [privateKey, process.env.NEXT_PUBLIC_DEFAULT_NOSTR_USER_SK]);

  return <NostrContext.Provider value={{ ndk, pool }} >{children}</NostrContext.Provider>;
};

export const useNostrContext = () => {
  const nostr = useContext(NostrContext);

  if (!nostr) {
    throw new Error('NostrContext must be used within a NostrProvider');
  }

  return nostr;
};
