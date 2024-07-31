import { ASKELADD_RELAY } from '@/constants/relay';
import { useNostrContext } from '@/context/NostrContext';
import { KIND_JOB_REQUEST, KIND_JOB_RESULT } from '@/types';
import { NDKKind } from '@nostr-dev-kit/ndk';
import { SimplePool, NostrEvent } from 'nostr-tools';
import { useState } from 'react';

interface IEventFilter {
  kind?: NDKKind | number, limit?: number, since?: number, until?: number, kinds?: NDKKind[], search?: string, ids?: string[], authors?: string[]
}
interface ISubscriptionData {
  pubkey?: string;
  created_at?: number;
  filter?: IEventFilter;
  onEventCallback: (event: NostrEvent) => void;
}
const DEFAULT_LIMIT = 300
export const useFetchEvents = () => {
  const { ndk } = useNostrContext();
  const [pool, setPool] = useState(new SimplePool())

  const fetchEvents = async (data: IEventFilter) => {
    try {
      if (!ndk?.signer) return { result: undefined, events: undefined };
      const { kind, limit, since, until, kinds, search } = data;
      let eventsResult = await ndk.fetchEvents({
        kinds: kind ? [kind] : kinds ?? [KIND_JOB_RESULT as NDKKind, KIND_JOB_REQUEST],
        since: since,
        until: until,
        limit: limit ?? DEFAULT_LIMIT,
        search: search
      });
      const events = Array.from(eventsResult?.values())
      return {
        result: undefined,
        events: events
      };
    } catch (e) {
      return {
        result: undefined,
        events: []
      };
    }
  }
  const fetchEventsTools = async (data: IEventFilter) => {
    try {
      if (!ndk?.signer) return { result: undefined, events: undefined };
      const { kind, limit, since, until, kinds, search } = data;
      const pool = new SimplePool()
      let relays = ASKELADD_RELAY;
      const kind_search = kind ? [kind] : kinds ?? [KIND_JOB_REQUEST, KIND_JOB_RESULT];
      const events = await pool.querySync(relays, { kinds: kind_search, until, since, limit: limit ?? DEFAULT_LIMIT, search })
      return {
        result: undefined,
        events: events
      };
    } catch (e) {
      return {
        result: undefined,
        events: []
      };
    }
  }

  /** @TODO fix subscription Nostr not working as expected */
  const setupSubscriptionNostr = async ({ pubkey, filter, onEventCallback }: ISubscriptionData) => {
    let h = await pool.subscribeMany(
      ASKELADD_RELAY,
      [
        {
          authors: filter?.authors ?? [],
          ids: filter?.ids ?? [],
          since: filter?.since,
          search: filter?.search,
          kinds: filter?.kinds,
        },
      ],
      {
        onevent(event) {
          console.log("Event subscription received: ", event?.id)
          onEventCallback(event)
          h.close();
        },

        onclose: () => {

        },
        oneose() {
          h.close()
        }
      }
    )
    setPool(pool);
    return h;
  }
  return { fetchEvents, fetchEventsTools, setupSubscriptionNostr, pool }
};
