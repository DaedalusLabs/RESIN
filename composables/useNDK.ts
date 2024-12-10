import NDK, { NDKNip07Signer, type NDKSigner } from '@nostr-dev-kit/ndk';

let ndkInstance: NDK | null = null;
let signer: NDKSigner;

export function setSigner(newSigner: NDKSigner) {
    signer = newSigner
}

export function useNDK() {
    if (!signer) {
        console.log('No signer');
        return;
    }

    if (!ndkInstance) {
        ndkInstance = new NDK({
            autoConnectUserRelays: true,
            explicitRelayUrls: [
                'wss://relay.damus.io',
                'wss://relay.nostr.band',
                'wss://relay.primal.net',
                // "wss://nostr.dbtc.link",
                // "wss://nostr1.daedaluslabs.io",
                // "wss://nostr2.daedaluslabs.io",
                // "wss://nostr3.daedaluslabs.io",
                // Add more relays as needed
            ],
            signer
        });
    }

    
    return ndkInstance;
}