import browser from "webextension-polyfill";
import {
  generateSecretKey,
  getPublicKey,
  getEventHash,
  Event,
  finalizeEvent,
  validateEvent,
} from "nostr-tools";
import { bytesToHex, hexToBytes } from "@noble/hashes/utils";

export async function getOrCreateKeyPair(): Promise<{
  privateKey: string;
  publicKey: string;
}> {
  const stored = await browser.storage.local.get(["privateKey", "publicKey"]);
  if (stored.privateKey && stored.publicKey) {
    console.log("Keys found in storage.");
    return { privateKey: stored.privateKey, publicKey: stored.publicKey };
  }

  console.log("No keys found in storage, generating new keys.");
  console.log("Generating private key...");
  const privateKeyBytes = generateSecretKey();
  console.log("Converting private key to hex...");
  const privateKey = bytesToHex(privateKeyBytes);
  console.log("Generating public key from private key...");
  const publicKey = getPublicKey(privateKeyBytes);
  console.log("Storing keys in storage...");
  await browser.storage.local.set({ privateKey, publicKey });
  console.log("Returning keys...");
  return { privateKey, publicKey };
}

browser.runtime.onMessage.addListener(async (message, sender) => {
  if (message.type === "getPublicKey") {
    const { publicKey } = await getOrCreateKeyPair();
    return publicKey;
  }

  if (message.type === "signEvent") {
    const { privateKey } = await getOrCreateKeyPair();
    let event: Event = message.event;
    event.pubkey = getPublicKey(hexToBytes(privateKey));
    event.id = getEventHash(event);
    event = finalizeEvent(event, hexToBytes(privateKey));
    validateEvent(event);
    return event;
  }

  // Add other message handlers as needed
});
