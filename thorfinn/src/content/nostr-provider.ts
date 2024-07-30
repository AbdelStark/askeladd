import { Event } from "nostr-tools";

const nostr = {
  _requests: {} as {
    [id: string]: {
      resolve: (value: any) => void;
      reject: (reason?: any) => void;
    };
  },
  _pubkey: null as string | null,

  async getPublicKey(): Promise<string> {
    if (this._pubkey) return this._pubkey;
    this._pubkey = await this._call("getPublicKey", {});
    return this._pubkey as string;
  },

  async signEvent(event: Event): Promise<Event> {
    return this._call("signEvent", { event });
  },

  async getRelays(): Promise<{
    [url: string]: { read: boolean; write: boolean };
  }> {
    return this._call("getRelays", {});
  },

  nip04: {
    async encrypt(peer: string, plaintext: string): Promise<string> {
      return nostr._call("nip04.encrypt", { peer, plaintext });
    },

    async decrypt(peer: string, ciphertext: string): Promise<string> {
      return nostr._call("nip04.decrypt", { peer, ciphertext });
    },
  },

  _call(type: string, params: any): Promise<any> {
    const id = Math.random().toString().slice(-4);
    return new Promise((resolve, reject) => {
      this._requests[id] = { resolve, reject };
      window.postMessage(
        {
          id,
          ext: "thorfinn",
          type,
          params,
        },
        "*",
      );
    });
  },
};

window.addEventListener("message", (message) => {
  if (
    !message.data ||
    message.data.response === null ||
    message.data.response === undefined ||
    message.data.ext !== "thorfinn" ||
    !nostr._requests[message.data.id]
  )
    return;

  if (message.data.response.error) {
    const error = new Error("thorfinn: " + message.data.response.error.message);
    nostr._requests[message.data.id].reject(error);
  } else {
    nostr._requests[message.data.id].resolve(message.data.response);
  }

  delete nostr._requests[message.data.id];
});

(window as any).nostr = nostr;
