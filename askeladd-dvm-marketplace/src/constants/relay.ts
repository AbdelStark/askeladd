export const ASKELADD_RELAY= [
    // DVM service prover deployed need to subscribed to this relays
    // "ws://127.0.0.1:8080", // run in local
    // "wss://relay.nostr.net",
    process.env.NEXT_PUBLIC_DEFAULT_RELAYER ?? 'wss://nostr-relay-nestjs-production.up.railway.app',// AFK relayer
]


export const APPLICATION_PUBKEY_DVM= process.env.NEXT_PUBLIC_DVM_PUBKEY
