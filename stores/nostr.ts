import { defineStore } from 'pinia';
import type { NDKFilter, NDKUser, NDKUserProfile } from '@nostr-dev-kit/ndk';
import * as bip39 from '@scure/bip39';
import { HDKey } from '@scure/bip32'
import { getRelayListForUser, NDKEvent, NDKNip07Signer, NDKPrivateKeySigner, NDKRelay } from '@nostr-dev-kit/ndk';
import { wordlist } from '@scure/bip39/wordlists/english';
import { bytesToHex } from '@noble/hashes/utils'
import { setSigner, useNDK } from '~/composables/useNDK';
import { getPublicKey } from 'nostr-tools';
import { useChatStore } from './chat';

const DERIVATION_PATH = `m/44'/1237'`

interface NostrState {
    user: NDKUser | null;
    pubkey: string | null;
    authenticated: boolean;
    mnemonic: string | null;
    typeKey: NostrLoginType | null;
    lastMessagesRead: number | null;
    pushNotificationsEnabled: boolean;
}

export enum NostrLoginType {
    Extension = "ext",
    Mnemonic = "mnemonic"
}

export interface NostrStore {
    authenticated: boolean;
    savePreferences: (key: string, value: string[]) => Promise<void>;
    loadPreferences: (key: string) => Promise<string[] | null>;
}

export const useNostrStore = defineStore('nostr', {
    state: (): NostrState => ({
        user: null,
        pubkey: null,
        authenticated: false,
        mnemonic: null,
        typeKey: null,
        lastMessagesRead: null,
        pushNotificationsEnabled: false,
    }),
    persist: {
        key: 'nostr-store',
        storage: localStorage,
        paths: ['mnemonic', 'typeKey', 'authenticated', 'lastMessagesRead', 'pushNotificationsEnabled'] 
    },
    getters: {
        unreadMessagesCount(): number {
            const chatStore = useChatStore();
            return chatStore.chats.reduce((count, chat) => count + chat.unreadCount, 0);
        },
        getTypeKey(): NostrLoginType | null {
            return this.typeKey;
        },
        async userProfile(): Promise<NDKUserProfile | null> {
            const ndk = useNDK();
            const profile = await (await ndk?.signer?.user())?.fetchProfile();
            return profile || null;
        }
    },
    actions: {
        async checkAuthenticated() {
            if (import.meta.client && this.typeKey) {
                console.log('check authenticated');
                switch (this.typeKey) {
                    case NostrLoginType.Extension:
                        await this.login();
                        
                        setSigner(new NDKNip07Signer());
                        this.pubkey = await window.nostr.getPublicKey();
                        break;
                    case NostrLoginType.Mnemonic:
                        const { privateKey, _, pubkey } = await this.generateKeyPair(this.mnemonic);
                        
                        setSigner(new NDKPrivateKeySigner(privateKey));
                        this.pubkey = pubkey;
                        break;
                    default:
                        console.log('No nostr signer');
                        return false;
                }

                const ndk = useNDK();

            if (ndk) {
                // this.pubkey = (await ndk?.signer?.user()).pubkey;
                await ndk.connect();

                this.user = await ndk.signer?.user();
            }

            return this.authenticated
            }
            
        },
        async login() {
            try {
                // Check if extension exists
                if (!window?.nostr) {
                    throw new Error('No Nostr extension found');
                    return;
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
            this.pubkey = getPublicKey(privateKey);

            return {
                privateKey: bytesToHex(privateKey),
                mnemonic,
                pubkey: this.pubkey
            };
        },
        async logout() {
            const ndk = useNDK();
            const chatStore = useChatStore();

            // Clear chat cache
            await chatStore.clearCache();

            if (ndk) {
                ndk.signer = undefined;
            }

            this.user = null;
            this.pubkey = null;
            this.typeKey = null;
            this.mnemonic = null;
            this.lastMessagesRead = null;
            this.authenticated = false;
            this.pushNotificationsEnabled = false;
        },
        async fetchDirectMessages() {
            const chatStore = useChatStore();
            await chatStore.init();
            await chatStore.fetchChats();
        },
        async sendDirectMessage(recipientPubkey: string, content: string, eventId?: string, eventKind?: number) {
            const ndk = useNDK();
            if (!ndk) throw new Error('NDK not initialized');

            // Create unsigned kind 14 message
            const unsignedMsg = {
                kind: 14,
                content,
                tags: [
                    ['p', recipientPubkey],
                    ...(eventId ? [
                        ['e', eventId],
                        ...(eventKind ? [['k', eventKind.toString()]] : [])
                    ] : [])
                ],
                created_at: Math.floor(Date.now() / 1000)
            };

            const recipient = await ndk.getUser({
                pubkey: recipientPubkey,
            })

            // Get recipient's preferred relays
            // const relayList = recipient.relayUrls
            const writeRelays = await getRelayListForUser(recipientPubkey, ndk);
        
            // Create wraps for both recipient and sender
            const senderUser = await ndk.signer?.user();
            if (!senderUser) throw new Error('No signer available');

            const wraps = await Promise.all([
                this.giftWrapMessage(unsignedMsg, recipient),
                this.giftWrapMessage(unsignedMsg, senderUser)
            ]);
            
            // Publish to recipient's write relays and our own relays
            await Promise.all(wraps.map(async (wrap) => {
                if (writeRelays && writeRelays.relays.size > 0) {
                    // Publish to recipient's write relays
                    await wrap.publish(writeRelays);
                } else {
                    // Fallback to default relays if no write relays found
                    await wrap.publish();
                }
            }));

            if (this.user)
                await this.user.fetchProfile();
        },
        async signMessage(message: string) {
            const ndk = useNDK();

            if (!ndk?.signer) {
                console.log('no signer');
            }

            const ndkEvent = new NDKEvent(ndk);
            ndkEvent.kind = 20000;
            ndkEvent.content = message;

            await ndkEvent?.sign();
            return ndkEvent.rawEvent();
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
        updateLastMessagesRead() {
            this.lastMessagesRead = Math.floor(Date.now() / 1000);
        },
        async unwrapMessage(event: NDKEvent): Promise<UnwrappedMessage> {
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

            if (!message.profile) {
                message.profile = await messageSender.fetchProfile();
            }

            const pTag = message.tags.find((t) => t[0] == 'p');

            return {
                id: event.id,
                pubkey: message.pubkey || messageSender.pubkey,
                content: message.content,
                created_at: message.created_at,
                tags: message.tags,
                user: messageSender,
                isSent: messageSender.pubkey == this.pubkey ? true : false,
                recipientPubkey: pTag ? pTag[1] : message.pubkey,
            };
        },
        async savePreferences(key: string, value: string[]): Promise<void> {
            const ndk = useNDK();
            if (!ndk) return;

            const config = useRuntimeConfig();
            const prefix = config.public.NOSTR_SETTINGS_PREFIX || 'resin';
            const prefixedKey = `${prefix}:${key}`;

            const event = new NDKEvent(ndk);
            event.kind = 30078;
            
            // Encrypt the content for ourselves
            const user = await ndk.signer?.user();
            if (!user) return;
            
            event.content = await ndk.signer?.encrypt(user, JSON.stringify(value), 'nip44') || '';
            event.tags = [['d', prefixedKey]];
            
            await event.publish();
        },
        async loadPreferences(key: string): Promise<string[] | null> {
            const ndk = useNDK();
            if (!ndk) {
                console.log('no ndk');
                return null;
            }

            await ndk.connect();

            // // Wait for at least one relay to connect (with 5 second timeout)
            // try {
            //     await new Promise<void>((resolve, reject) => {
            //         const timeout = setTimeout(() => {
            //             reject(new Error('Timeout waiting for relay connection'));
            //         }, 5000);

            //         const checkConnection = () => {
            //             const connectedRelays = Array.from(ndk.pool.relays.values()).filter(relay => relay.connected);
            //             console.log('connectedRelays', connectedRelays);
            //             if (connectedRelays.length > 3) {
            //                 clearTimeout(timeout);
            //                 resolve();
            //             } else {
            //                 setTimeout(checkConnection, 100);
            //             }
            //         };
            //         checkConnection();
            //     });
            // } catch (e) {
            //     console.error('Failed to connect to any relay:', e);
            //     return null;
            // }

            const config = useRuntimeConfig();
            const prefix = config.public.NOSTR_SETTINGS_PREFIX || 'resin';
            const prefixedKey = `${prefix}:${key}`;

            const filter: NDKFilter = {
                kinds: [30078],
                authors: [this.pubkey!],
                '#d': [prefixedKey],
                limit: 1
            };
            
            // Create a promise that will resolve when we get our first event or timeout
            const event = await new Promise<NDKEvent | null>((resolve, reject) => {
                const timeout = setTimeout(() => {
                    resolve(null);
                }, 5000); // 5 second timeout
        
                ndk.fetchEvents(filter).then(events => {
                    clearTimeout(timeout);
                    const firstEvent = Array.from(events)[0];
                    resolve(firstEvent || null);
                });
            });
            
            if (!event) return null;

            try {
                const user = await ndk.signer?.user();
                if (!user) return null;

                const decryptedContent = await ndk.signer?.decrypt(user, event.content, 'nip44');
                if (!decryptedContent) return null;

                return JSON.parse(decryptedContent);
            } catch (e) {
                console.error('Failed to parse preferences:', e);
                return null;
            }
        },
        async togglePushNotifications() {
            if (!this.pushNotificationsEnabled) {
                // Request permission if not already granted
                const permission = await Notification.requestPermission();
                if (permission === 'granted') {
                    this.pushNotificationsEnabled = true;
                }
            } else {
                this.pushNotificationsEnabled = false;
            }
        },
        async showNotification(title: string, body: string) {
            console.log('showNotification', this.pushNotificationsEnabled);
            if (!this.pushNotificationsEnabled) return;

            try {
                const notification = new Notification(title, {
                    body,
                    icon: '/images/logos/Resin_Hexagon_Orange_Fill.svg',
                    tag: 'resin-chat',
                    renotify: true
                });

                notification.onclick = () => {
                    window.focus();
                    notification.close();
                    // Navigate to chat
                    const router = useRouter();
                    router.push('/chat');
                };
            } catch (error) {
                console.error('Error showing notification:', error);
            }
        }
    }
});    