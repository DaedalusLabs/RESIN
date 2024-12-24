import { defineStore } from 'pinia';
import type { NDKFilter, NDKUser } from '@nostr-dev-kit/ndk';
import * as bip39 from '@scure/bip39';
import { HDKey } from '@scure/bip32'

import { NDKEvent, NDKNip07Signer, NDKPrivateKeySigner } from '@nostr-dev-kit/ndk';
import { wordlist } from '@scure/bip39/wordlists/english';
import { bytesToHex } from '@noble/hashes/utils'
import { setSigner, useNDK } from '~/composables/useNDK';

const DERIVATION_PATH = `m/44'/1237'`
const NOSTR_TYPE_KEY = 'nostr'

interface NostrState {
    user: NDKUser | null;
    pubkey: string | null;
    authenticated: boolean;
    mnemonic: string | null;
    messages: NostrMessage[];
    typeKey: NostrLoginType | null;
    lastMessagesRead: string | null;
}

interface NostrMessage {
    id: string;
    pubkey: string;
    content: string;
    created_at: number;
    tags: string[][];
    user: NDKUser | undefined;
    isSent: boolean;
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
        mnemonic:  null,
        messages: [],
        typeKey: null,
        lastMessagesRead: null,
    }),
    persist: {
        key: 'nostr-store',
        storage: piniaPluginPersistedstate.localStorage(),
        paths: ['mnemonic', 'typeKey', 'authenticated', 'lastMessagesRead'] 
    },
    getters: {
        unreadMessagesCount(): number {
            return this.messages.filter((m) => {
                return m.created_at > this.lastMessagesRead;
            }).length;
        },
        getTypeKey(): NostrLoginType | null {
            return this.typeKey;
        }
    },
    actions: {
        async checkAuthenticated() {
            if (import.meta.client && this.typeKey) {
                switch (this.typeKey) {
                    case NostrLoginType.Extension:
                        await this.login();
                        setSigner(new NDKNip07Signer());
                        break;
                    case NostrLoginType.Mnemonic:
                        const { privateKey } = await this.generateKeyPair(this.mnemonic);
                        console.log('set signer');
                        setSigner(new NDKPrivateKeySigner(privateKey));
                        break;
                    default:
                        console.log('No nostr signer');
                }
            }
            const ndk = useNDK();

            if (ndk) {
                this.pubkey = (await ndk?.signer?.user()).pubkey;
                await ndk.connect();

                this.user = await ndk.signer?.user();
            }

            return this.authenticated
        },
        async login() {
            try {
                // Check if extension exists
                if (!window?.nostr) {
                    throw new Error('No Nostr extension found');
                }

                const pubkey = await window.nostr.getPublicKey();
                this.pubkey = pubkey;
                this.authenticated = true;

                this.typeKey = NostrLoginType.Extension;

                return pubkey;
            } catch (error) {
                this.authenticated = false;
                throw error;
            }
        },
        // async loginWithKey(nsec: `${string}1${string}`) {
        //     const signer = new NDKPrivateKeySigner(nsec);

        //     const privateKey = signer.privateKey;

        //     return { privateKey };
        // },
        async generateKeyPair(mnemonic: string | null = null, accountIndex = 0) {
            if (!mnemonic) {
                mnemonic = bip39.generateMnemonic(wordlist, 128);
            }
            const root = HDKey.fromMasterSeed(await bip39.mnemonicToSeed(mnemonic));
            const seed = root.derive(`${DERIVATION_PATH}/${accountIndex}'/0/0`);
            const privateKey = seed.privateKey!;

            this.mnemonic = mnemonic;
            this.authenticated = true;
            this.typeKey = NostrLoginType.Mnemonic;

            return {
                privateKey: bytesToHex(privateKey),
                mnemonic
            };
        },
        logout() {
            this.user = null;
            this.pubkey = null;
            this.typeKey = null;
            this.authenticated = false;
        },
        async fetchDirectMessages() {
            const ndk = useNDK();

            if (!ndk) {
                console.log('No NDK instance');
                return;
            }

            const filter: NDKFilter = {
                kinds: [1059], // Gift wrap kind
                '#p': [this.pubkey!],
                // since: Math.floor(Date.now() / 1000) - (60 * 60 * 24 * 30) // Last 7 days
            };

            const messages = await ndk.subscribe(filter, { closeOnEose: false });

            messages.on('event', async (e) => {
                const u = await this.unwrapMessage(e);

                if (this.messages.find((m) => m.id == u.id) == undefined) {
                    this.messages.push(u);
                    this.messages = this.messages.sort((a, b) => a.created_at - b.created_at);
                }
            })
        },
        async sendDirectMessage(recipientPubkey: string, content: string) {
            const ndk = useNDK();

            // Create unsigned kind 14 message
            const unsignedMsg = {
                kind: 14,
                content,
                tags: [
                    ['p', recipientPubkey]
                ],
                created_at: Math.floor(Date.now() / 1000)
            };

            const recipient = await ndk.getUser({
                pubkey: recipientPubkey,
            });

            const wraps = await Promise.all([
                this.giftWrapMessage(unsignedMsg, recipient),
                this.giftWrapMessage(unsignedMsg, await ndk?.signer?.user()!)
            ]);
            
            await Promise.all(wraps.map(wrap => wrap.publish()));

            if (this.user)
                await this.user.fetchProfile();

        },

        async giftWrapMessage(unsignedMsg, targetUser: NDKUser) {
            const ndk = useNDK();

            if (!ndk?.signer) {
                console.log('no signer');
            }

            const signer = await ndk?.signer;

            // Create seal (kind 13)
            const seal = new NDKEvent(ndk);
            seal.kind = 13;
            seal.content = await signer?.encrypt(targetUser, JSON.stringify(unsignedMsg), 'nip44');
            await seal.sign();

            const randomSigner = NDKPrivateKeySigner.generate()

            // Create gift wrap (kind 1059)
            const giftWrap = new NDKEvent(ndk);
            giftWrap.kind = 1059;
            giftWrap.tags = [['p', targetUser.pubkey]];
            giftWrap.created_at = giftWrap.created_at - Math.round((Math.random() * 600));
            giftWrap.content = await randomSigner.encrypt(targetUser, JSON.stringify(seal), 'nip44');
            await giftWrap.sign(randomSigner);

            // Send to recipient's preferred relays
            return giftWrap;
        },
        async updateLastMessagesRead() {
            this.lastMessagesRead = Math.floor(new Date().getTime() / 1000); // Using getTime() which returns UTC milliseconds since epoch
        },
        async unwrapMessage(event: NDKEvent): Promise<NostrMessage> {
            const ndk = useNDK();

            let sealSender = ndk.getUser({
                pubkey: event.pubkey,
            });

            if (sealSender.pubkey == (await ndk.signer?.user())?.pubkey) {
                sealSender = await ndk.signer?.user()!;
            }

            // Unwrap gift wrap
            const sealedContent = await ndk.signer?.decrypt(sealSender, event.content, "nip44");
            const seal = JSON.parse(sealedContent || "{}");

            const messageSender = ndk.getUser({
                pubkey: seal.pubkey,
            });

            // Unwrap seal
            const messageContent = await ndk.signer?.decrypt(messageSender, seal.content, "nip44");
            const message = JSON.parse(messageContent || "{}");

            await messageSender.fetchProfile();

            return {
                id: event.id,
                pubkey: message.pubkey,
                content: message.content,
                created_at: message.created_at,
                tags: message.tags,
                user: messageSender,
                isSent: messageSender.pubkey == this.pubkey ? true : false,
            };
        }
    }
});    