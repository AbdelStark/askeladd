import { useNostrContext } from '@/context/NostrContext';
import { NDKEvent, NDKKind } from '@nostr-dev-kit/ndk';

export const useSendNote = () => {
  const { ndk } = useNostrContext();
  const sendNote = async (data: { content: string; tags?: string[][], kind?: NDKKind | number }) => {
    try {
      if (!ndk?.signer) return { result: undefined, event: undefined };
      const event = new NDKEvent(ndk);
      event.kind = data?.kind;
      event.content = data.content;
      event.tags = data.tags ?? [];

      let result = await event.publish();
      return { result: result, event };
    } catch (e) {
      return {
        result: undefined,
        event: undefined
      }
    }
  }
  const publishNote = async (event:NDKEvent) => {
    try {
      if (!ndk?.signer) return { result: undefined, event: undefined };
      let result = await event.publish();
      return { result: result, event };
    } catch (e) {
      return {
        result: undefined,
        event: undefined
      }
    }
  }
  return { sendNote, publishNote }
};
