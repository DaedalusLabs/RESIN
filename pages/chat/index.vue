<template>
   <section class="h-full flex flex-col overflow-hidden">
      <!-- Header -->
      <div class="flex w-full items-center justify-between border-b border-gray-200 bg-white px-10 py-5">
         <h1 class="text-2xl font-extrabold leading-tight text-pirate-950">
            {{ $t('chat.title') }}
         </h1>
         <div class="flex justify-between items-center">
            <button
               @click="fetchFullHistory"
               class="text-sm px-4 py-2 bg-resin-500 text-white rounded-lg hover:bg-resin-600 disabled:opacity-50 flex items-center gap-2" 
               :disabled="isLoadingHistory"
            >
            <PhArrowClockwise :size="20" />
               {{ isLoadingHistory ? $t('chat.loadingHistory') : $t('chat.fetchHistory') }}
            </button>
            <div v-if="isLoadingHistory" class="flex flex-col gap-2 ml-4">
               <div class="flex items-center gap-2">
                  <div class="text-sm text-gray-600">{{ phase }}</div>
                  <div class="text-sm text-gray-600">{{ processedEvents }}/{{ totalEvents }}</div>
                  <div class="w-48 h-2 bg-gray-200 rounded-full overflow-hidden">
                     <div 
                        class="h-full bg-resin-500 transition-all duration-300"
                        :style="{ width: `${(processedEvents / totalEvents) * 100}%` }"
                     ></div>
                  </div>
               </div>
               <div v-if="decryptionPhase" class="flex items-center gap-2">
                  <div class="text-sm text-gray-600">Decrypting</div>
                  <div class="text-sm text-gray-600">{{ decryptedEvents }}/{{ totalDecryptEvents }}</div>
                  <div class="w-48 h-2 bg-gray-200 rounded-full overflow-hidden">
                     <div 
                        class="h-full bg-resin-500 transition-all duration-300"
                        :style="{ width: `${(decryptedEvents / totalDecryptEvents) * 100}%` }"
                     ></div>
                  </div>
               </div>
            </div>
         </div>
      </div>

      <!-- Loading State -->
      <div v-if="isLoading" class="flex flex-1 items-center justify-center">
         <div class="text-center">
            <div class="mb-4">
               <img src="/images/logos/Resin_Hexagon_Orange_Fill.svg" class="mx-auto h-12 w-12 animate-pulse" alt="Loading" />
            </div>
            <p class="text-gray-500">{{ $t('chat.loading') }}</p>
         </div>
      </div>

      <!-- Error State -->
      <div v-else-if="error" class="flex flex-1 items-center justify-center">
         <div class="text-center">
            <div class="mb-4 text-red-500">
               <PhWarning :size="48" class="mx-auto" />
            </div>
            <p class="text-gray-500">{{ error }}</p>
            <button
               class="mt-4 rounded-lg bg-resin-500 px-4 py-2 text-white hover:bg-resin-600"
               @click="retryInitialization"
            >
               {{ $t('chat.retry') }}
            </button>
         </div>
      </div>

      <!-- Main Chat Layout -->
      <div v-else class="flex flex-1 min-h-0">
         <!-- Left Sidebar - Conversations List -->
         <ChatList 
            :chats="filteredChats"
            :selected-pubkey="selectedChat?.pubkey"
            :show-hide-button="true"
            @select="selectChat"
            @hide="hideChat"
         />

         <!-- Right Side - Chat Messages -->
         <div class="flex flex-1 flex-col bg-gray-50 min-h-0">
            <template v-if="selectedChat">
               <!-- Chat Header -->
               <ChatMessageHeader
                  :user-profile="selectedChat.userProfile"
                  :pubkey="selectedChat.pubkey"
               >
                  <template #actions>
                     <a
                        :href="'https://njump.me/' + pubkeyToNpub(selectedChat.pubkey)"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="rounded-lg bg-gray-100 px-3 py-2 text-sm text-gray-700 hover:bg-gray-200"
                     >
                        View on njump
                     </a>
                  </template>
               </ChatMessageHeader>

               <!-- Messages -->
               <div ref="messagesContainer" class="flex-1 overflow-y-auto p-4">
                  <FlowbiteChatBubble
                     v-for="msg in selectedChat.messages"
                     :key="msg.id"
                     :message="msg.content"
                     :name="formatDisplayName(msg.userProfile?.name, msg.pubkey)"
                     :time="(new Date(msg.created_at * 1000)).toLocaleString()"
                     :profile-image="msg.userProfile?.image || '/images/logos/Resin_Hexagon_Orange_Fill.svg'"
                     :is-sent="msg.isSent"
                  >
                     <template #below-message>
                        <div v-if="isPropertyReference(msg)" class="mt-2 mb-4">
                           <Suspense>
                              <PropertyCard 
                                 v-if="msg.property"
                                 :property="msg.property" 
                                 compact
                              />
                              <template #fallback>
                                 <div class="rounded-lg border border-gray-200 bg-white p-4">
                                    <div class="animate-pulse flex space-x-4">
                                       <div class="flex-1 space-y-3">
                                          <div class="h-4 bg-gray-200 rounded w-3/4"></div>
                                          <div class="h-4 bg-gray-200 rounded w-1/2"></div>
                                       </div>
                                    </div>
                                 </div>
                              </template>
                           </Suspense>
                        </div>
                     </template>
                  </FlowbiteChatBubble>
               </div>

               <!-- Message Input -->
               <ChatMessageInput
                  :is-sending="isSending"
                  @send="handleSendMessage"
               />
            </template>
            <div v-else class="flex h-full items-center justify-center text-gray-500">
               {{ $t('chat.selectConversation') }}
            </div>
         </div>
      </div>
   </section>
</template>

<script setup lang="ts">
import { PhArrowClockwise, PhWarning } from "@phosphor-icons/vue";
import { useChatStore } from '~/stores/chat';
import { useNostrStore } from '~/stores/nostr';
import type { Chat, ChatMessage } from '~/stores/chat';
import { nip19 } from 'nostr-tools';
import type { NDKFilter, NDKEvent, NDKKind } from '@nostr-dev-kit/ndk';
import type { UnwrappedMessage } from '~/types/nostr';
import type { Property } from '~/types/property';

interface MessageWithProperty extends ChatMessage {
   property?: Property;
}

const { t } = useI18n();

useHead({
   title: t('chat.title'),
});

definePageMeta({
   layout: "white",
   middleware: ['auth'],
   hideBottomBar: false
});

const chatStore = useChatStore();
const nostrStore = useNostrStore();
const searchQuery = ref('');
const selectedChat = ref<Chat | null>(null);
const newMessage = ref('');
const isSending = ref(false);
const messagesContainer = ref<HTMLElement | null>(null);
const isLoading = ref(true);
const error = ref<string | null>(null);
const isLoadingHistory = ref(false);
const processedEvents = ref(0);
const totalEvents = ref(0);
const decryptionPhase = ref(false);
const decryptedEvents = ref(0);
const totalDecryptEvents = ref(0);
const phase = ref('Fetching');
const hiddenChats = ref<Set<string>>(new Set());

// Initialize
async function initializeChat() {
   try {
      isLoading.value = true;
      error.value = null;
      await chatStore.init();
    //   await chatStore.fetchChats();
   } catch (err) {
      error.value = err instanceof Error ? err.message : 'An error occurred';
      console.error('Failed to initialize chat:', err);
   } finally {
      isLoading.value = false;
   }
}

async function retryInitialization() {
   await initializeChat();
}

onMounted(async () => {
   const stored = localStorage.getItem('hidden-chats');
   if (stored) {
      hiddenChats.value = new Set(JSON.parse(stored));
   }
   await initializeChat();
});

// Computed
const filteredChats = computed(() => {
   let chats = chatStore.chats;
   
   // Filter out hidden chats
   chats = chats.filter(chat => !hiddenChats.value.has(chat.pubkey));
   
   // Apply search filter
   if (searchQuery.value) {
      const query = searchQuery.value.toLowerCase();
      chats = chats.filter(chat => {
         const name = chat.userProfile?.name?.toLowerCase() || '';
         const pubkey = chat.pubkey.toLowerCase();
         return name.includes(query) || pubkey.includes(query);
      });
   }
   
   return chats;
});

// Methods
function selectChat(chat: Chat) {
   selectedChat.value = chat;
   nextTick(() => {
      scrollToBottom();
   });
}

function scrollToBottom() {
   if (messagesContainer.value) {
      messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight;
   }
}

async function handleSendMessage(message: string) {
   try {
      isSending.value = true;
      // Cast through unknown to bypass type checking since we know the method exists
      await ((nostrStore as unknown) as { sendDirectMessage: (pubkey: string, content: string) => Promise<void> })
         .sendDirectMessage(selectedChat.value!.pubkey, message);
      scrollToBottom();
   } finally {
      isSending.value = false;
   }
}

function formatTime(timestamp?: number): string {
   if (!timestamp) return '';
   
   const date = new Date(timestamp * 1000);
   const now = new Date();
   
   if (date.toDateString() === now.toDateString()) {
      return date.toLocaleTimeString(undefined, { hour: '2-digit', minute: '2-digit' });
   }
   
   if (now.getFullYear() === date.getFullYear()) {
      return date.toLocaleDateString(undefined, { month: 'short', day: 'numeric' });
   }
   
   return date.toLocaleDateString(undefined, { year: 'numeric', month: 'short', day: 'numeric' });
}

// Helper function to format display names
function formatDisplayName(name?: string | null, pubkey?: string | null): string {
   if (name) return name;
   if (pubkey) return pubkey.slice(0, 8);
   return 'Unknown';
}

// Helper function to convert pubkey to npub
function pubkeyToNpub(pubkey?: string | null): string {
   if (!pubkey) return '';
   return nip19.npubEncode(pubkey);
}

// Watch for new messages
watch(() => chatStore.chats, () => {
   if (selectedChat.value) {
      const updatedChat = chatStore.chats.find(c => c.pubkey === selectedChat.value?.pubkey);
      if (updatedChat) {
         selectedChat.value = updatedChat;
         nextTick(() => {
            scrollToBottom();
         });
      }
   }
}, { deep: true });

async function fetchFullHistory() {
   const ndk = useNDK();
   if (!ndk) {
      console.error('NDK not initialized');
      return;
   }

   try {
      console.log('Starting to fetch full message history...');
      isLoadingHistory.value = true;
      processedEvents.value = 0;
      decryptionPhase.value = false;
      decryptedEvents.value = 0;
      phase.value = 'Fetching';

      const filter: NDKFilter = {
         kinds: [1059 as NDKKind],
         '#p': [nostrStore.pubkey!],
      };
      console.log('Using filter:', filter);

      // Fetch all events
      console.log('Fetching events from relays...');
      const events = await ndk.fetchEvents(filter);
      const eventArray = Array.from(events);
      totalEvents.value = eventArray.length;
      console.log(`Found ${totalEvents.value} events to process`);

      // Process events in batches to show progress
    //   for (const event of eventArray) {
    //      try {
    //         console.log(`Processing event ${processedEvents.value + 1}/${totalEvents.value}:`, event.id);
    //         processedEvents.value++;
    //      } catch (error) {
    //         console.error('Error processing event:', event.id, error);
    //      }
    //   }

      // Start decryption phase
      console.log('Starting decryption phase...');
      phase.value = 'Decrypting';
      decryptionPhase.value = true;
      totalDecryptEvents.value = eventArray.length;

      for (const event of eventArray) {
         try {
            // console.log(`Decrypting event ${decryptedEvents.value + 1}/${totalDecryptEvents.value}:`, event.id);
            // Cast through unknown to bypass type checking since we know the method exists
            const unwrappedMessage = await ((nostrStore as unknown) as { unwrapMessage: (event: NDKEvent) => Promise<UnwrappedMessage> }).unwrapMessage(event);
            
            // Convert to ChatMessage format
            const message = {
               id: unwrappedMessage.id,
               pubkey: unwrappedMessage.pubkey,
               content: unwrappedMessage.content,
               created_at: unwrappedMessage.created_at,
               tags: unwrappedMessage.tags,
               userProfile: unwrappedMessage.user?.profile ? JSON.parse(JSON.stringify(unwrappedMessage.user.profile)) : undefined,
               isSent: unwrappedMessage.isSent,
               recipientPubkey: unwrappedMessage.recipientPubkey,
               encryptedContent: event.content
            };

            await chatStore.addMessage(message);
            decryptedEvents.value++;
            // console.log(`Successfully decrypted message from ${unwrappedMessage.isSent ? 'you to' : ''} ${unwrappedMessage.pubkey}`);
         } catch (error) {
            console.error('Error decrypting event:', event.id, error);
         }
      }
      console.log('Finished processing all events');
   } catch (error) {
      console.error('Error fetching history:', error);
   } finally {
      isLoadingHistory.value = false;
      decryptionPhase.value = false;
      console.log('History fetch completed');
   }
}

const propertiesStore = usePropertiesStore();

async function loadPropertyForMessage(message: MessageWithProperty) {
   if (!message.property) {
      const propertyId = message.tags.find((tag: string[]) => tag[0] === 'e')?.[1];
      if (propertyId) {
         try {
            message.property = await propertiesStore.get(propertyId);
         } catch (error) {
            console.error('Error loading property:', error);
         }
      }
   }
}

// Watch for changes in selectedChat messages and load properties
watch(() => selectedChat.value?.messages, async (messages?: MessageWithProperty[]) => {
   if (messages) {
      for (const message of messages) {
         if (isPropertyReference(message)) {
            await loadPropertyForMessage(message);
         }
      }
   }
}, { immediate: true, deep: true });

function isPropertyReference(message: MessageWithProperty): message is MessageWithProperty & { property: Property } {
   const eTags = message.tags.filter((tag: string[]) => tag[0] === 'e');
   const kTags = message.tags.filter((tag: string[]) => tag[0] === 'k');
   
   return eTags.length > 0 && kTags.length > 0 && 
          (kTags[0][1] === '30402' || kTags[0][1] === '30403');
}

function hideChat(pubkey: string) {
   hiddenChats.value.add(pubkey);
   localStorage.setItem('hidden-chats', JSON.stringify(Array.from(hiddenChats.value)));
   
   // If the hidden chat was selected, clear selection
   if (selectedChat.value?.pubkey === pubkey) {
      selectedChat.value = null;
   }
}
</script>

<style scoped>
.line-clamp-1 {
   display: -webkit-box;
   -webkit-line-clamp: 1;
   -webkit-box-orient: vertical;
   overflow: hidden;
}
</style> 