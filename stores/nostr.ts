import { defineStore } from 'pinia';
import type { NDKUser } from '@nostr-dev-kit/ndk';
import * as bip39 from '@scure/bip39';
import { HDKey } from '@scure/bip32'

import { NDKPrivateKeySigner } from '@nostr-dev-kit/ndk';
import { wordlist } from '@scure/bip39/wordlists/english';
import { bytesToHex } from '@noble/hashes/utils'

const DERIVATION_PATH = `m/44'/1237'`
const NOSTR_TYPE_KEY = 'nostr'
const NOSTR_MNEMONIC_KEY = 'mnemonic'

interface NostrState {
    user: NDKUser | null;
    pubkey: string | null;
    authenticated: boolean;
    mnemonic: string | null;
}

enum NostrLoginType {
    Extension = "ext",
    Mnemonic = "mnemonic"
}

export const useNostrStore = defineStore('nostr', {
    state: (): NostrState => ({
        user: null,
        pubkey: null,
        authenticated: false,
        mnemonic: import.meta.client && localStorage.getItem(NOSTR_TYPE_KEY) == NostrLoginType.Mnemonic ? localStorage.getItem(NOSTR_MNEMONIC_KEY) : null,
    }),
    actions: {
        async checkAuthenticated() {
            if (import.meta.client && localStorage.getItem(NOSTR_TYPE_KEY) !== null) {
                switch (localStorage.getItem(NOSTR_TYPE_KEY)) {
                    case NostrLoginType.Extension: 
                        await this.login();
                        break;
                    case NostrLoginType.Mnemonic:
                        await this.generateKeyPair(this.mnemonic);
                        break;
                }
            }
            return this.authenticated
        },
        async login() {
            try {
                // Check if extension exists
                if (!window?.nostr) {
                    throw new Error('No Nostr extension found');
                }

                // Request public key
                const pubkey = await window.nostr.getPublicKey();
                this.pubkey = pubkey;
                this.authenticated = true;
                localStorage.setItem(NOSTR_TYPE_KEY, NostrLoginType.Extension);

                return pubkey;
            } catch (error) {
                this.authenticated = false;
                throw error;
            }
        },
        async loginWithKey(nsec: `${string}1${string}`) {
            const signer = new NDKPrivateKeySigner(nsec);
        
            const privateKey = signer.privateKey;
            
            return { privateKey };
        },
        async generateKeyPair(mnemonic: string | null = null, accountIndex = 0) {
            if (!mnemonic) {
                mnemonic = bip39.generateMnemonic(wordlist, 128);
            }
            const root = HDKey.fromMasterSeed(await bip39.mnemonicToSeed(mnemonic));
            const seed = root.derive(`${DERIVATION_PATH}/${accountIndex}'/0/0`);
            const privateKey = seed.privateKey!;
            const publicKey = seed.publicKey!;

            this.mnemonic = mnemonic;
            this.pubkey = bytesToHex(publicKey);
            this.authenticated = true;

            localStorage.setItem(NOSTR_TYPE_KEY, NostrLoginType.Mnemonic);
            localStorage.setItem(NOSTR_MNEMONIC_KEY, this.mnemonic);

            return {
                privateKey: bytesToHex(privateKey),
                publicKey: bytesToHex(publicKey),
                mnemonic
            };
        },
        logout() {
            this.user = null;
            this.pubkey = null;
            this.authenticated = false;
        }
    }
});    