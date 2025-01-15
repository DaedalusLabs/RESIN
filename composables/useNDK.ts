import NDK, { type NDKSigner } from "@nostr-dev-kit/ndk";

let ndkInstance: NDK | null = null;
let signer: NDKSigner;

export function setSigner(newSigner: NDKSigner) {
   signer = newSigner;
}

export function useNDK() {
   if (!signer) {
      console.log("No signer");
      return;
   }

   if (!ndkInstance) {
      ndkInstance = new NDK({
         autoConnectUserRelays: true,
         explicitRelayUrls: [
            "wss://relay.damus.io",
            "wss://relay.nostr.band",
            "wss://relay.primal.net",
            "wss://nos.lol",
            "wss://nostr.dbtc.link",
            "wss://nostr1.daedaluslabs.io",
            "wss://nostr2.daedaluslabs.io",
         ],
         signer,
      });
   }

   return ndkInstance;
}
