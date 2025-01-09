import type { NDKUser } from '@nostr-dev-kit/ndk';

export interface UnwrappedMessage {
    id: string;
    pubkey: string;
    content: string;
    created_at: number;
    tags: string[][];
    user?: NDKUser;
    isSent: boolean;
    recipientPubkey: string;
} 