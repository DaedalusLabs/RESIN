import { defineStore } from 'pinia';
import Dexie from 'dexie';
import NDKCacheAdapterDexie from '@nostr-dev-kit/ndk-cache-dexie';
import { NDKEvent } from '@nostr-dev-kit/ndk';
import type { NDKUserProfile, NDKUser, NDKFilter } from '@nostr-dev-kit/ndk';
import { useNostrStore } from './nostr';
import { useNDK } from '~/composables/useNDK';
import { useLocalStorage } from '@vueuse/core';

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
}

class ChatDatabase extends Dexie {
    messages!: Dexie.Table<ChatMessage, string>;
    users!: Dexie.Table<SerializedUser, string>;

    constructor() {
        super('ChatDatabase');
        this.version(1).stores({
            messages: 'id, pubkey, created_at',
            users: 'pubkey'
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
    }),

    actions: {
        async init() {
            if (this.initialized) return;

            const nostrStore = useNostrStore();
            
            // Wait for nostr authentication
            if (!nostrStore.authenticated) {
                // Cast through unknown to bypass type checking since we know the method exists
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

            this.initialized = true;
        },

        async fetchChats() {
            const nostrStore = useNostrStore();
            const ndk = useNDK();

            if (!ndk || !nostrStore.pubkey || !nostrStore.authenticated) {
                console.log('Nostr not ready:', { ndk: !!ndk, pubkey: !!nostrStore.pubkey, authenticated: nostrStore.authenticated });
                return;
            }

            try {
                // Fetch and display messages from cache first
                const cachedMessages = await db.messages.toArray();
                await this.processMessages(cachedMessages, true); // Skip profile fetching initially

                // Set up subscription for new messages
                const filter = {
                    kinds: [1059], // Gift wrap kind
                    '#p': nostrStore.pubkey ? [nostrStore.pubkey] : [],
                    ...(this.lastMessageTimestamp ? { since: this.lastMessageTimestamp } : {})
                };

                console.log('Filter:', filter);

                const sub = ndk.subscribe(filter as unknown as NDKFilter, { closeOnEose: false });

                sub.on('event', async (event: NDKEvent) => {
                    try {
                        // Store the original encrypted content
                        const encryptedContent = event.content;

                        // Cast through unknown to bypass type checking since we know the method exists
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
                            encryptedContent // Store the original encrypted content
                        };
                        
                        // Cache the message
                        await db.messages.put(message);
                        
                        // Update the chat
                        await this.addMessage(message);
                    } catch (error) {
                        console.error('Error processing message:', error);
                    }
                });

                // Load profiles in the background
                this.loadProfiles();

            } catch (error) {
                console.error('Error fetching chats:', error);
                throw error; // Re-throw to handle in the UI
            }
        },

        async processMessages(messages: ChatMessage[], skipProfiles = false) {
            const nostrStore = useNostrStore();
            const ndk = useNDK();

            if (!ndk || !nostrStore.authenticated) {
                console.log('Nostr not ready for processing messages');
                return;
            }

            // Group messages by chat (pubkey)
            const chatMap = new Map<string, Chat>();

            // First pass: Create chats and load profiles
            for (const message of messages) {
                // Skip messages without a valid pubkey
                if (!message.pubkey) {
                    console.warn('Skipping message without pubkey:', message);
                    continue;
                }

                // For outgoing messages, use recipientPubkey as the chat key
                const chatPubkey = message.isSent ? message.recipientPubkey : message.pubkey;
                
                let chat = chatMap.get(chatPubkey);
                if (!chat) {
                    chat = {
                        pubkey: chatPubkey,
                        messages: [],
                        unreadCount: 0,
                    };
                    chatMap.set(chatPubkey, chat);

                    // Try to get user profile from cache only
                    if (!skipProfiles) {
                        try {
                            const cachedUser = await db.users.get(chatPubkey);
                            if (cachedUser?.profile) {
                                const userProfile = JSON.parse(JSON.stringify(cachedUser.profile));
                                // If the profile doesn't have an image or name but we have cached values, use them
                                if (!userProfile.image && cachedUser.imageUrl) {
                                    userProfile.image = cachedUser.imageUrl;
                                }
                                if (!userProfile.name && cachedUser.name) {
                                    userProfile.name = cachedUser.name;
                                }
                                chat.userProfile = userProfile;
                            }
                        } catch (error) {
                            console.error('Error fetching cached user profile:', error);
                        }
                    }
                }
            }

            // Second pass: Process messages and decrypt if needed
            for (const message of messages) {
                if (!message.pubkey) continue;

                const chatPubkey = message.isSent ? message.recipientPubkey : message.pubkey;
                const chat = chatMap.get(chatPubkey);
                if (!chat) continue;

                // If we have decrypted content, use it directly
                if (message.content) {
                    chat.messages.push(message);
                }
                // Otherwise, try to decrypt if we have encrypted content
                else if (message.encryptedContent && ndk) {
                    try {
                        const event = new NDKEvent(ndk);
                        event.content = message.encryptedContent;
                        event.pubkey = message.pubkey;
                        event.id = message.id;
                        event.created_at = message.created_at;
                        event.tags = message.tags;

                        // Cast through unknown to bypass type checking since we know the method exists
                        const unwrappedMessage = await ((nostrStore as unknown) as { unwrapMessage: (event: NDKEvent) => Promise<UnwrappedMessage> }).unwrapMessage(event);
                        
                        // Update the message with decrypted content
                        message.content = unwrappedMessage.content;
                        await db.messages.put(message);
                        
                        chat.messages.push(message);
                    } catch (error) {
                        console.error('Error decrypting cached message:', error);
                    }
                }
                
                // Update last message if this is more recent
                if (!chat.lastMessage || message.created_at > chat.lastMessage.created_at) {
                    chat.lastMessage = message;
                }

                // Also update the message's user profile if needed
                if (!message.userProfile && !skipProfiles) {
                    try {
                        const messagePubkey = message.isSent ? message.recipientPubkey : message.pubkey;
                        const cachedUser = await db.users.get(messagePubkey);
                        if (cachedUser?.profile) {
                            const userProfile = JSON.parse(JSON.stringify(cachedUser.profile));
                            // If the profile doesn't have an image or name but we have cached values, use them
                            if (!userProfile.image && cachedUser.imageUrl) {
                                userProfile.image = cachedUser.imageUrl;
                            }
                            if (!userProfile.name && cachedUser.name) {
                                userProfile.name = cachedUser.name;
                            }
                            message.userProfile = userProfile;
                        }
                    } catch (error) {
                        console.error('Error fetching cached user profile for message:', error);
                    }
                }
            }

            // Sort messages in each chat by timestamp
            for (const chat of chatMap.values()) {
                chat.messages.sort((a, b) => a.created_at - b.created_at);
            }

            // Update store
            this.chats = Array.from(chatMap.values()).sort((a, b) => {
                return (b.lastMessage?.created_at || 0) - (a.lastMessage?.created_at || 0);
            });
        },

        async addMessage(message: ChatMessage) {
            // Skip messages without a valid pubkey
            if (!message.pubkey) {
                console.warn('Skipping message without pubkey:', message);
                return;
            }

            // For outgoing messages, use recipientPubkey as the chat key
            const chatPubkey = message.isSent ? message.recipientPubkey : message.pubkey;
            const chatIndex = this.chats.findIndex(c => c.pubkey === chatPubkey);
            
            if (chatIndex === -1) {
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

                this.chats.unshift({
                    pubkey: chatPubkey,
                    messages: [message],
                    lastMessage: message,
                    unreadCount: message.isSent ? 0 : 1,
                    userProfile,
                });
            } else {
                // Existing chat
                const chat = this.chats[chatIndex];
                
                // Add message if not already present
                if (!chat.messages.find(m => m.id === message.id)) {
                    chat.messages.push(message);
                    chat.messages.sort((a, b) => a.created_at - b.created_at);
                    
                    // Update last message if this is more recent
                    if (!chat.lastMessage || message.created_at > chat.lastMessage.created_at) {
                        chat.lastMessage = message;
                    }
                    
                    // Update unread count
                    if (!message.isSent) {
                        chat.unreadCount++;
                    }

                    // Move chat to top if it's a new message
                    if (message.created_at > (chat.lastMessage?.created_at || 0)) {
                        this.chats.splice(chatIndex, 1);
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
    },
}); 