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
         <div class="flex w-96 flex-col border-r border-gray-200 bg-white">
            <!-- Search -->
            <div class="border-b border-gray-200 p-4">
               <div class="relative">
                  <input
                     v-model="searchQuery"
                     type="text"
                     :placeholder="$t('chat.search')"
                     class="w-full rounded-lg border border-gray-300 bg-gray-50 p-2.5 pl-10 text-sm text-gray-900 focus:border-gray-500 focus:ring-0"
                  />
                  <PhMagnifyingGlass class="absolute left-3 top-2.5 text-gray-500" :size="20" />
               </div>
            </div>
            <!-- Conversations List -->
            <div class="flex-1 overflow-y-auto">
               <div
                  v-for="chat in filteredChats"
                  :key="chat.pubkey"
                  class="cursor-pointer border-b border-gray-100 p-4 hover:bg-gray-50"
                  :class="{ 'bg-gray-100': selectedChat?.pubkey === chat.pubkey }"
                  @click="selectChat(chat)"
               >
                  <div class="flex items-center gap-3">
                     <img
                        :src="chat.userProfile?.image || '/images/logos/Resin_Hexagon_Orange_Fill.svg'"
                        class="h-12 w-12 rounded-full"
                        alt="Profile"
                     />
                     <div class="flex-1">
                        <div class="flex items-center justify-between">
                           <h3 class="font-semibold">
                              {{ formatDisplayName(chat.userProfile?.name, chat.pubkey) }}
                           </h3>
                           <span class="text-xs text-gray-500">
                              {{ formatTime(chat.lastMessage?.created_at) }}
                           </span>
                        </div>
                        <p class="mt-1 text-xs text-gray-500">
                           {{ pubkeyToNpub(chat.pubkey) }}
                        </p>
                        <p class="mt-1 text-sm text-gray-600 line-clamp-1">
                           {{ chat.lastMessage?.content || '' }}
                        </p>
                     </div>
                  </div>
               </div>
            </div>
         </div>

         <!-- Right Side - Chat Messages -->
         <div class="flex flex-1 flex-col bg-gray-50 min-h-0">
            <template v-if="selectedChat">
               <!-- Chat Header -->
               <div class="flex items-center justify-between border-b border-gray-200 bg-white p-4">
                  <div class="flex items-center gap-3">
                     <img
                        :src="selectedChat.userProfile?.image || '/images/logos/Resin_Hexagon_Orange_Fill.svg'"
                        class="h-10 w-10 rounded-full"
                        alt="Profile"
                     />
                     <div>
                        <h2 class="font-semibold">
                           {{ formatDisplayName(selectedChat.userProfile?.name, selectedChat.pubkey) }}
                        </h2>
                        <p class="text-xs text-gray-500">
                           {{ pubkeyToNpub(selectedChat.pubkey) }}
                        </p>
                     </div>
                  </div>
                  <a
                     :href="'https://njump.me/' + pubkeyToNpub(selectedChat.pubkey)"
                     target="_blank"
                     rel="noopener noreferrer"
                     class="rounded-lg bg-gray-100 px-3 py-2 text-sm text-gray-700 hover:bg-gray-200"
                  >
                     View on njump
                  </a>
               </div>

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
                        <div v-if="isPropertyReference(msg)" class="mt-2">
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
               <div class="border-t border-gray-200 bg-white p-4">
                  <div class="flex w-full items-center rounded-lg border bg-white">
                     <textarea
                        v-model="newMessage"
                        rows="1"
                        :placeholder="$t('chat.typeMessage')"
                        class="flex-grow rounded-l-lg border-none bg-transparent px-4 py-2 focus:outline-none resize-none overflow-y-auto max-h-[250px]"
                        @keyup.enter.exact="sendMessage"
                        @keydown.shift.enter.prevent="addNewline"
                        @input="autoGrow"
                        :disabled="isSending"
                     />
                     <button
                        class="rounded-r-lg border-none bg-transparent p-4 focus:outline-none disabled:opacity-50"
                        @click="sendMessage"
                        :disabled="isSending"
                        :title="$t('chat.send')"
                     >
                        <PhPaperPlaneTilt :size="20" />
                     </button>
                  </div>
               </div>
            </template>
            <div v-else class="flex h-full items-center justify-center text-gray-500">
               {{ $t('chat.selectConversation') }}
            </div>
         </div>
      </div>
   </section>
</template>

<script setup lang="ts">
import { PhArrowClockwise, PhMagnifyingGlass, PhPaperPlaneTilt, PhWarning } from "@phosphor-icons/vue";
import { useChatStore } from '~/stores/chat';
import { useNostrStore } from '~/stores/nostr';
import type { Chat } from '~/stores/chat';
import { nip19 } from 'nostr-tools';
import type { NDKFilter, NDKEvent, NDKKind } from '@nostr-dev-kit/ndk';
import type { UnwrappedMessage } from '~/types/nostr';

interface ChatMessage {
   id: string;
   pubkey: string;
   content: string;
   created_at: number;
   tags: string[][];
   userProfile?: {
      [x: string]: string | number | undefined;
      created_at?: number;
      name?: string;
   };
   isSent: boolean;
   recipientPubkey: string;
   property?: any;
}

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

// Initialize
async function initializeChat() {
   try {
      isLoading.value = true;
      error.value = null;
      await chatStore.init();
      await chatStore.fetchChats();
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
   await initializeChat();
});

// Computed
const filteredChats = computed(() => {
   if (!searchQuery.value) return chatStore.chats;
   
   const query = searchQuery.value.toLowerCase();
   return chatStore.chats.filter(chat => {
      const name = chat.userProfile?.name?.toLowerCase() || '';
      const pubkey = chat.pubkey.toLowerCase();
      return name.includes(query) || pubkey.includes(query);
   });
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

async function sendMessage() {
   if (!newMessage.value.trim() || isSending.value || !selectedChat.value) return;

   try {
      isSending.value = true;

      // Cast through unknown to bypass type checking since we know the method exists
      await ((nostrStore as unknown) as { sendDirectMessage: (pubkey: string, content: string) => Promise<void> })
         .sendDirectMessage(selectedChat.value.pubkey, newMessage.value);
      newMessage.value = '';
      
      // Reset textarea height
      const textarea = document.querySelector('textarea');
      if (textarea) {
         textarea.style.height = 'auto';
      }
      
      scrollToBottom();
   } finally {
      isSending.value = false;
   }
}

function autoGrow(e: Event) {
   const textarea = e.target as HTMLTextAreaElement;
   textarea.style.height = 'auto';
   const newHeight = Math.min(textarea.scrollHeight, 250);
   textarea.style.height = newHeight + 'px';
}

function addNewline(e: Event) {
   newMessage.value += '\n';
   nextTick(() => autoGrow(e));
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

async function loadPropertyForMessage(message: ChatMessage) {
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
watch(() => selectedChat.value?.messages, async (messages?: ChatMessage[]) => {
   if (messages) {
      for (const message of messages) {
         if (isPropertyReference(message)) {
            await loadPropertyForMessage(message);
         }
      }
   }
}, { immediate: true, deep: true });

function isPropertyReference(message: { tags: string[][] }) {
   const eTags = message.tags.filter((tag: string[]) => tag[0] === 'e');
   const kTags = message.tags.filter((tag: string[]) => tag[0] === 'k');
   
   return eTags.length > 0 && kTags.length > 0 && 
          (kTags[0][1] === '30402' || kTags[0][1] === '30403');
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