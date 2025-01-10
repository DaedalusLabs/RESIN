// composables/useNostr.ts
import { NDKPrivateKeySigner, NDKEvent, NDKUser } from '@nostr-dev-kit/ndk';
import { storeToRefs } from 'pinia';
import { useNostrStore } from "~/stores/nostr";
import { useNDK } from "./useNDK";

export function useNostr() {
   const nostrStore = useNostrStore();
   const { user, isLoading, error } = storeToRefs(nostrStore);
   const ndk = useNDK();

   const isAuthenticated = computed(() => nostrStore.authenticated);
   const pubkey = computed(() => user.value?.pubkey);

   async function loginWithExtension() {
      if (!ndk) throw new Error("NDK not initialized");
      return nostrStore.loginWithExtension(ndk);
   }

   async function loginWithMnemonic(mnemonic: string) {
      if (!ndk) throw new Error("NDK not initialized");
      return nostrStore.loginWithMnemonic(ndk, mnemonic);
   }

   async function loginWithNsecBunker(pubkey: string, relay: string, secret: string) {
      if (!ndk) throw new Error('NDK not initialized');

      try {
         // Create a temporary signer for the session
         const tempSigner = NDKPrivateKeySigner.generate();
         ndk.signer = tempSigner;

         // Create a bunker auth request event
         const authEvent = new NDKEvent(ndk);
         authEvent.kind = 27235; // Bunker auth request
         authEvent.tags = [
            ["p", pubkey], // The bunker's public key
            ["relay", relay], // The bunker's relay
            ["secret", secret], // The secret from the bunker URL
            ["u", window.location.origin], // The app's origin
            ["c", "resin-app"] // Client identifier
         ];
         await authEvent.sign();

         // Connect to NDK and wait for bunker response
         await ndk.connect();

         // Set up subscription for bunker response
         const sub = ndk.subscribe(
            {
               kinds: [27235],
               "#e": [authEvent.id!],
               limit: 1
            },
            { closeOnEose: false }
         );

         // Wait for bunker response or timeout
         const response = await new Promise<NDKEvent>((resolve, reject) => {
            const timeout = setTimeout(() => {
               reject(new Error("Bunker connection timeout"));
               sub.stop();
            }, 30000);

            sub.on("event", (event: NDKEvent) => {
               clearTimeout(timeout);
               resolve(event);
               sub.stop();
            });
         });

         // Update user with bunker response
         const userObj = ndk.getUser({ pubkey: response.pubkey });
         if (!userObj) throw new Error('Failed to get user');
         nostrStore.user = userObj;
      } catch (error) {
         console.error("Failed to login with NSec Bunker:", error);
         throw error;
      }
   }

   function hasExtension() {
      return typeof window !== "undefined" && "nostr" in window;
   }

   function hasNip44() {
      return typeof window !== "undefined" && "nip44" in window;
   }

   async function checkAuthenticated() {
      return nostrStore.authenticated;
   }

   return {
      isLoading,
      error,
      loginWithExtension,
      loginWithMnemonic,
      loginWithNsecBunker,
      hasExtension,
      hasNip44,
      checkAuthenticated,
      isAuthenticated,
      pubkey,
      user,
   };
}