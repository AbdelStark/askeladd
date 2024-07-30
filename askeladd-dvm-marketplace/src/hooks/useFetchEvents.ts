import { useNostrContext } from '@/context/NostrContext';
import { NDKEvent, NDKKind } from '@nostr-dev-kit/ndk';

export const useFetchEvents = () => {
  const { ndk } = useNostrContext();
  const fetchEvents = async (kind?: NDKKind | number, limit?:number) => {
    try {
      if (!ndk?.signer) return { result: undefined, events: undefined };
      let eventsResult = await ndk.fetchEvents({
        kinds: [kind ?? 6600 as NDKKind],
        limit: limit ?? 50,
      });
      const events = Array.from(eventsResult?.values())
      console.log("events", events);

      return {
        result:undefined,
        events:events
      };
    } catch (e) {
      return {
        result:undefined,
        events:[]
      };
    }
  }
  return { fetchEvents }
};
