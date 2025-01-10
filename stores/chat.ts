import { defineStore } from 'pinia';
import Dexie from 'dexie';
import NDKCacheAdapterDexie from '@nostr-dev-kit/ndk-cache-dexie';
import { NDKEvent } from '@nostr-dev-kit/ndk';
import type { NDKUserProfile, NDKUser, NDKFilter } from '@nostr-dev-kit/ndk';
import { useNostrStore } from './nostr';
import { useNDK } from '~/composables/useNDK';
import { useLocalStorage } from '@vueuse/core';
import { useSettingsStore } from './settings';
import { useRouter } from 'vue-router';

interface SerializedUser {
    pubkey: string;
    profile?: NDKUserProfile;
    imageUrl?: string;
    name?: string;
}

interface UnwrappedMessage {
    id: string;
    pubkey: string;
    content: string;
    created_at: number;
    tags: string[][];
    user?: NDKUser;
    isSent: boolean;
    recipientPubkey: string;
}

export interface ChatMessage {
    id: string;
    pubkey: string;
    content: string;
    created_at: number;
    tags: string[][];
    userProfile?: NDKUserProfile;
    isSent: boolean;
    recipientPubkey: string;
    encryptedContent?: string; // Original encrypted content
}

export interface Chat {
    pubkey: string;
    userProfile?: NDKUserProfile;
    messages: ChatMessage[];
    lastMessage?: ChatMessage;
    unreadCount: number;
    lastMessageTimestamp?: number;
}

class ChatDatabase extends Dexie {
    messages!: Dexie.Table<ChatMessage, string>;
    users!: Dexie.Table<SerializedUser, string>;
    chats!: Dexie.Table<Chat, string>; // Store chats by pubkey

    constructor() {
        super('ChatDatabase');
        this.version(2).stores({
            messages: 'id, pubkey, created_at, recipientPubkey',
            users: 'pubkey',
            chats: 'pubkey, lastMessageTimestamp' // Index chats by pubkey and last message timestamp
        });
    }
}

const db = new ChatDatabase();

export const useChatStore = defineStore('chat', {
    state: () => ({
        chats: [] as Chat[],
        initialized: false,
        cacheAdapter: null as NDKCacheAdapterDexie | null,
        lastMessageTimestamp: useLocalStorage('chat-last-message-timestamp', null as number | null),
        processingQueue: Promise.resolve(),
    }),

    getters: {
        whitelistedChats: (state) => {
            const config = useRuntimeConfig();
            const whitelist = config.public.PUBKEY_WHITELIST || [];
            
            return state.chats.filter(chat => {
                // Allow messages from whitelisted pubkeys
                if (whitelist.includes(chat.pubkey)) return true;
                // Allow messages sent by the user to whitelisted pubkeys
                if (chat.lastMessage?.isSent && whitelist.includes(chat.lastMessage.recipientPubkey)) return true;
                return false;
            });
        },
        whitelistedMessages: (state) => {
            const config = useRuntimeConfig();
            const whitelist = config.public.PUBKEY_WHITELIST || [];
            
            return state.chats.filter(chat => {
                // Allow messages from whitelisted pubkeys
                if (whitelist.includes(chat.pubkey)) return true;
                // Allow messages sent by the user to whitelisted pubkeys
                if (chat.lastMessage?.isSent && whitelist.includes(chat.lastMessage.recipientPubkey)) return true;
                return false;
            }).flatMap(chat => chat.messages);
        }
    },

    actions: {
        async processMessageInQueue(callback: () => Promise<void>) {
            // Chain the new operation to the existing queue
            this.processingQueue = this.processingQueue
                .then(callback)
                .catch(error => {
                    console.error('Error processing message in queue:', error);
                });
            
            // Wait for this operation to complete
            await this.processingQueue;
        },

        async init() {
            if (this.initialized) return;

            const nostrStore = useNostrStore();
            
            // Wait for nostr authentication
            if (!nostrStore.authenticated) {
                await ((nostrStore as unknown) as { checkAuthenticated: () => Promise<boolean> }).checkAuthenticated();
            }

            if (!nostrStore.authenticated) {
                throw new Error('Nostr authentication required');
            }

            // Initialize NDK cache adapter
            this.cacheAdapter = new NDKCacheAdapterDexie();
            const ndk = useNDK();
            if (ndk) {
                ndk.cacheAdapter = this.cacheAdapter;
            }

            // Load chats from database
            const storedChats = await db.chats.orderBy('lastMessageTimestamp').reverse().toArray();
            this.chats = storedChats;

            this.initialized = true;
        },

        async fetchChats() {
            const nostrStore = useNostrStore();
            const ndk = useNDK();

            if (!ndk || !nostrStore.pubkey || !nostrStore.authenticated) {
                console.log('Nostr not ready:', { ndk: !!ndk, pubkey: !!nostrStore.pubkey, authenticated: nostrStore.authenticated });
                return;
            }
            console.log('fetching chats');

            try {
                // Set up subscription for new messages
                const filter: { kinds: number[]; '#p': string[]; since?: number } = {
                    kinds: [1059], // Gift wrap kind
                    '#p': nostrStore.pubkey ? [nostrStore.pubkey] : [],
                    // ...(this.lastMessageTimestamp ? { since: this.lastMessageTimestamp - 1000 } : {}),
                    since: this.lastMessageTimestamp ? this.lastMessageTimestamp - 2000 : new Date(Date.now() - 7 * 24 * 60 * 60 * 1000).getTime()
                };
                const sub = ndk.subscribe(filter as unknown as NDKFilter, { closeOnEose: false });

                // Keep track of processed message IDs to prevent duplicates
                const processedMessageIds = new Set<string>();

                sub.on('event', async (event: NDKEvent) => {
                    // console.log('event', event);
                    // Process each message in sequence using the queue
                    await this.processMessageInQueue(async () => {
                        try {
                            // Skip if we've already processed this message
                            if (processedMessageIds.has(event.id)) {
                                return;
                            }

                            // Add to processed set BEFORE async operations
                            processedMessageIds.add(event.id);

                            // Store the original encrypted content
                            const encryptedContent = event.content;

                            // Process message synchronously to prevent race conditions
                            const unwrappedMessage = await ((nostrStore as unknown) as { unwrapMessage: (event: NDKEvent) => Promise<UnwrappedMessage> }).unwrapMessage(event);
                            
                            // Update last message timestamp
                            if (!this.lastMessageTimestamp || unwrappedMessage.created_at > this.lastMessageTimestamp) {
                                this.lastMessageTimestamp = unwrappedMessage.created_at;
                            }
                            
                            // Convert to ChatMessage format and ensure it's serializable
                            const message: ChatMessage = {
                                id: unwrappedMessage.id,
                                pubkey: unwrappedMessage.pubkey,
                                content: unwrappedMessage.content,
                                created_at: unwrappedMessage.created_at,
                                tags: unwrappedMessage.tags,
                                userProfile: unwrappedMessage.user?.profile ? JSON.parse(JSON.stringify(unwrappedMessage.user.profile)) : undefined,
                                isSent: unwrappedMessage.isSent,
                                recipientPubkey: unwrappedMessage.recipientPubkey,
                                encryptedContent
                            };

                            await this.addMessage(message);
                        } catch (error) {
                            processedMessageIds.delete(event.id);
                            console.error('Error processing message:', error);
                        }
                    });
                });

                // Load profiles in the background
                this.loadProfiles();

            } catch (error) {
                console.error('Error fetching chats:', error);
                throw error;
            }
        },

        async addMessage(message: ChatMessage) {
            // Skip messages without a valid pubkey
            if (!message.pubkey) {
                console.warn('Skipping message without pubkey:', message);
                return;
            }

            const settingsStore = useSettingsStore();
            const chatPubkey = message.isSent ? message.recipientPubkey : message.pubkey;
            
            // Find existing chat by pubkey
            let chat = await db.chats.get(chatPubkey);
            let isNewChat = false;

            if (!chat) {
                isNewChat = true;
                // New chat
                const ndk = useNDK();
                let userProfile: NDKUserProfile | undefined;
                
                if (ndk) {
                    try {
                        const user = await ndk.getUser({ pubkey: chatPubkey });
                        if (user) {
                            const profile = await user.fetchProfile();
                            if (profile) {
                                // Ensure profile is serializable
                                userProfile = JSON.parse(JSON.stringify(profile));
                                // Cache the profile, image URL, and name
                                await db.users.put({
                                    pubkey: chatPubkey,
                                    profile: userProfile,
                                    imageUrl: profile.image || undefined,
                                    name: profile.name || undefined
                                });
                            }
                        }
                    } catch (error) {
                        console.error('Error fetching user profile for new chat:', error);
                    }
                }

                chat = {
                    pubkey: chatPubkey,
                    messages: [],
                    lastMessage: message,
                    unreadCount: message.isSent ? 0 : 1,
                    userProfile,
                };
            }

            // Add message if not already present
            if (!chat.messages.find(m => m.id === message.id)) {
                chat.messages.push(message);
                chat.messages.sort((a, b) => a.created_at - b.created_at);
                
                // Update last message if this is more recent
                if (!chat.lastMessage || message.created_at > chat.lastMessage.created_at) {
                    chat.lastMessage = message;
                }
                
                // Update unread count and show notification
                if (!message.isSent) {
                    chat.unreadCount++;
                    if (settingsStore.notifications) {
                        const name = chat.userProfile?.name || chatPubkey.slice(0, 8);
                        try {
                            const notification = new Notification(name, {
                                body: message.content,
                                icon: '/images/logos/Resin_Hexagon_Orange_Fill.svg',
                                tag: 'resin-chat'
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

                // Save message to database
                await db.messages.put(message);

                // Update chat in database with last message timestamp
                await db.chats.put({
                    ...chat,
                    lastMessageTimestamp: message.created_at
                });

                // Update store state
                if (isNewChat) {
                    this.chats.unshift(chat);
                } else {
                    const index = this.chats.findIndex(c => c.pubkey === chatPubkey);
                    if (index !== -1) {
                        this.chats.splice(index, 1);
                        this.chats.unshift(chat);
                    }
                }
            }
        },

        markChatAsRead(pubkey: string) {
            const chat = this.chats.find(c => c.pubkey === pubkey);
            if (chat) {
                chat.unreadCount = 0;
            }
        },

        async loadProfiles() {
            const ndk = useNDK();
            if (!ndk) return;

            for (const chat of this.chats) {
                if (!chat.userProfile) {
                    try {
                        const user = await ndk.getUser({ pubkey: chat.pubkey });
                        if (user) {
                            const profile = await user.fetchProfile();
                            if (profile) {
                                // Ensure profile is serializable
                                const serializedProfile = JSON.parse(JSON.stringify(profile));
                                chat.userProfile = serializedProfile;

                                // Cache the profile, image URL, and name
                                await db.users.put({
                                    pubkey: chat.pubkey,
                                    profile: serializedProfile,
                                    imageUrl: profile.image || undefined,
                                    name: profile.name || undefined
                                });
                            }
                        }
                    } catch (error) {
                        console.error('Error fetching user profile:', error);
                    }
                }
            }
        },

        async clearCache() {
            try {
                // Clear Dexie database
                await db.messages.clear();
                await db.users.clear();

                // Clear NDK cache if available
                if (this.cacheAdapter) {
                    const ndkDb = new Dexie('ndk');
                    await ndkDb.delete();
                    this.cacheAdapter = null;
                }

                // Reset store state
                this.chats = [];
                this.initialized = false;
                this.lastMessageTimestamp = null;

                console.log('Chat cache cleared successfully');
            } catch (error) {
                console.error('Error clearing chat cache:', error);
                throw error;
            }
        },
    },
}); 