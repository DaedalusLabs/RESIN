import { defineStore } from 'pinia';
import NDK, { NDKPrivateKeySigner, NDKEvent, type NDKUser, NDKNip07Signer } from '@nostr-dev-kit/ndk';
import { privateKeyFromSeedWords } from 'nostr-tools/nip06';

const DAEDALUS_RELAYS = [
   "wss://nostr1.daedaluslabs.io",
   "wss://nostr2.daedaluslabs.io",
   "wss://nostr3.daedaluslabs.io",
];

export enum NostrLoginType {
   Extension = 'extension',
   Mnemonic = 'mnemonic',
   NsecBunker = 'nsecBunker'
}

interface BunkerClientKeys {
   privateKey: string;
   publicKey: string;
}

export const useNostrStore = defineStore('nostr', {
   state: () => ({
      user: null as NDKUser | null,
      ndk: null as NDK | null,
      isLoading: false,
      error: null as string | null,
      typeKey: null as NostrLoginType | null,
      mnemonic: null as string | null,
      bunkerClientKeys: null as BunkerClientKeys | null,
      nostrConnectUrl: null as string | null,
   }),

   getters: {
      authenticated: (state) => !!state.user,
      getTypeKey: (state) => state.typeKey,
   },

   persist: {
    key: 'nostr-store',
    storage: localStorage,
    paths: ['mnemonic', 'typeKey', 'bunkerClientKeys'] 
},

   actions: {
      async checkAuthenticated() {
         return !!this.user;
      },
      async loginWithExtension(ndk: NDK) {
         this.isLoading = true;
         this.error = null;

         try {
            const signer = new NDKNip07Signer();
            ndk.signer = signer;
            await ndk.connect();

            const user = await signer.user();
            if (!user) throw new Error('Failed to get user from extension');
            
            this.user = user;
            this.typeKey = NostrLoginType.Extension;
         } catch (error) {
            this.error = error instanceof Error ? error.message : 'Failed to login with extension';
            throw error;
         } finally {
            this.isLoading = false;
         }
      },

      async loginWithMnemonic(ndk: NDK, mnemonic: string) {
         this.isLoading = true;
         this.error = null;

         try {
            const privateKey = privateKeyFromSeedWords(mnemonic);
            const signer = new NDKPrivateKeySigner(privateKey);
            ndk.signer = signer;
            await ndk.connect();

            const user = await signer.user();
            if (!user) throw new Error('Failed to get user from mnemonic');
            
            this.user = user;
            this.typeKey = NostrLoginType.Mnemonic;
            this.mnemonic = mnemonic;
         } catch (error) {
            this.error = error instanceof Error ? error.message : 'Failed to login with mnemonic';
            throw error;
         } finally {
            this.isLoading = false;
         }
      },

      async loginWithNsecBunker(secret: string, bunkerPubkey?: string) {
         this.isLoading = true;
         this.error = null;

         try {
            console.debug('NSec Bunker: Starting login process');
            
            // Generate or retrieve client keypair
            if (!this.bunkerClientKeys) {
               console.debug('NSec Bunker: Generating new client keypair');
               const clientSigner = NDKPrivateKeySigner.generate();
               const clientUser = await clientSigner.user();
               if (!clientUser?.pubkey || !clientSigner.privateKey) {
                  throw new Error('Failed to generate client keypair');
               }
               this.bunkerClientKeys = {
                  privateKey: clientSigner.privateKey,
                  publicKey: clientUser.pubkey
               };
            } else {
               console.debug('NSec Bunker: Using existing client keypair', this.bunkerClientKeys);
            }

            const relay = DAEDALUS_RELAYS[0];

            // Generate nostrconnect URL if we have a bunker pubkey
            if (true) {
               const connectUrl = new URL(`nostrconnect://${this.bunkerClientKeys.publicKey}`);
               connectUrl.searchParams.set('relay', relay);
            //    connectUrl.searchParams.set('public_key', this.bunkerClientKeys.publicKey);
            //    connectUrl.searchParams.set('client_pubkey', this.bunkerClientKeys.publicKey);
               connectUrl.searchParams.set('perms', 'nip44_encrypt,nip44_decrypt');
               connectUrl.searchParams.set('secret', secret);
               connectUrl.searchParams.set('name', 'resin');
               this.nostrConnectUrl = connectUrl.toString();
               console.debug('NSec Bunker: Generated connect URL:', this.nostrConnectUrl);
            } else {
               // Generate bunker URL for client-initiated connection
               const bunkerUrl = new URL('bunker://');
               bunkerUrl.searchParams.set('relay', relay);
               bunkerUrl.searchParams.set('client_pubkey', this.bunkerClientKeys.publicKey);
               bunkerUrl.searchParams.set('secret', secret);
               this.nostrConnectUrl = bunkerUrl.toString();
               console.debug('NSec Bunker: Generated bunker URL:', this.nostrConnectUrl);
            }

            if (!this.ndk) {
               console.debug('NSec Bunker: Initializing NDK with relay:', relay);
               this.ndk = new NDK({
                  explicitRelayUrls: [relay],
               });
            }

            if (!this.ndk || !this.bunkerClientKeys) {
               throw new Error('Failed to initialize NDK or client keypair');
            }

            // Create a temporary signer for the session using our client keypair
            console.debug('NSec Bunker: Setting up signer with client keypair');
            const signer = new NDKPrivateKeySigner(this.bunkerClientKeys.privateKey);
            this.ndk.signer = signer;

            // Create a bunker auth request event (NIP-46)
            console.debug('NSec Bunker: Creating connect event');
            const authEvent = new NDKEvent(this.ndk);
            authEvent.kind = 24133; // NIP-46 connect event
            authEvent.tags = [
               ["client", "resin-app"], // Client identifier
               ["url", window.location.origin], // The app's origin
               ["secret", secret], // The secret for validation
            ];

            // Add bunker pubkey tag if provided
            if (bunkerPubkey) {
               authEvent.tags.push(["p", bunkerPubkey]);
            }

            await authEvent.sign();

            // Connect to NDK and wait for bunker response
            console.debug('NSec Bunker: Connecting to relay');
            await this.ndk.connect();

            // Set up subscription for bunker response
            console.debug('NSec Bunker: Setting up subscription for response');
            const sub = this.ndk.subscribe(
               {
                  kinds: [24133], // NIP-46 connect response
                  "#e": [authEvent.id!],
                  limit: 1
               },
               { closeOnEose: false }
            );

            // Wait for bunker response or timeout
            console.debug('NSec Bunker: Waiting for response');
            const response = await new Promise<NDKEvent>((resolve, reject) => {
               const timeout = setTimeout(() => {
                  console.debug('NSec Bunker: Connection timeout');
                  reject(new Error("Bunker connection timeout"));
                  sub.stop();
               }, 30000);

               sub.on("event", (event: NDKEvent) => {
                  console.debug('NSec Bunker: Received response event');
                  // Validate secret in response
                  const secretTag = event.tags.find(tag => tag[0] === 'secret');
                  if (!secretTag || secretTag[1] !== secret) {
                     reject(new Error('Invalid secret in response'));
                     return;
                  }
                  clearTimeout(timeout);
                  resolve(event);
                  sub.stop();
               });
            });

            // Update user with bunker response
            console.debug('NSec Bunker: Updating user with response');
            const userObj = this.ndk.getUser({ pubkey: response.pubkey });
            if (!userObj) throw new Error('Failed to get user');
            this.user = userObj;
            this.typeKey = NostrLoginType.NsecBunker;
            console.debug('NSec Bunker: Login successful');
         } catch (error) {
            console.error('NSec Bunker: Login failed', error);
            this.error = error instanceof Error ? error.message : 'Failed to login with NSec Bunker';
            throw error;
         } finally {
            this.isLoading = false;
         }
      },
   },
});    