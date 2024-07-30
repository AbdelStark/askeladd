import { useEffect, useState } from 'react';

type NostrSDKModule = typeof import('@rust-nostr/nostr-sdk');

/** @TODO fix WASM import => using nostr-dev-kit at this moment */
export function useWasmNostrSDK() {
  const [nostrSDK, setNostrSDK] = useState<NostrSDKModule | null>(null);
  const [error, setError] = useState<Error | null>(null);
  const [isLoading, setIsLoading] = useState<boolean>(true);

  useEffect(() => {
    let isMounted = true;

    async function loadWasmModule() {
      try {
        setIsLoading(true);
        // const { Client, Event, Keys, NostrSigner, Tag, Output, EventId ,  } = await import('@rust-nostr/nostr-sdk');
        const sdkModule: NostrSDKModule = await import('@rust-nostr/nostr-sdk');

        const NostrSDK = await import('@rust-nostr/nostr-sdk');
        if (isMounted) {
          setNostrSDK(sdkModule);
          setIsLoading(false);
        }
      } catch (err) {
        if (isMounted) {
          setError(err as Error);
          setIsLoading(false);
        }
      }
    }

    // Ensure this is only run on the client
    if (typeof window !== 'undefined') {
      loadWasmModule();
    }

    return () => {
      isMounted = false;
    };
  }, []);

  return { nostrSDK, error, isLoading };
}
