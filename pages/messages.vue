<template>
   <section class="h-full flex flex-col overflow-hidden">
      <!-- Header -->
      <div class="flex w-full items-center justify-between border-b border-gray-200 bg-white px-10 py-5">
         <div class="flex items-center gap-4">
            <button @click="goBack" class="text-gray-500 hover:text-gray-700">
               <PhArrowLeft :size="24" />
            </button>
            <h1 class="text-2xl font-extrabold leading-tight text-pirate-950">
               {{ $t('messages.title') }}
            </h1>
         </div>
      </div>

      <!-- Messages -->
      <div ref="chatContainer" class="flex-1 overflow-y-auto p-4 bg-gray-50">
         <FlowbiteChatBubble
            v-for="msg in messages"
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
      <ChatMessageInput
         :is-sending="isSending"
         @send="sendMessage"
      />
   </section>
</template>

<script setup lang="ts">
import { PhArrowLeft } from "@phosphor-icons/vue";
import { useChatStore } from '~/stores/chat';
import { useNostrStore } from '~/stores/nostr';
import type { ChatMessage } from '~/stores/chat';
import type { Property } from '~/types/property';

interface MessageWithProperty extends ChatMessage {
   property?: Property;
}

const { t } = useI18n();

useHead({
   title: t('messages.title'),
});

definePageMeta({
   layout: "white",
   middleware: ['auth'],
   hideBottomBar: false
});

const chatStore = useChatStore();
const nostrStore = useNostrStore();
const runtimeConfig = useRuntimeConfig();
const chatContainer = ref<HTMLElement | null>(null);
const isSending = ref(false);

const messages = computed(() => {
   const whitelist = runtimeConfig.public.PUBKEY_WHITELIST || [];
   const allMessages = chatStore.chats.flatMap(chat => chat.messages);
   
   return allMessages.filter(msg => {
      // Allow messages from whitelisted pubkeys
      if (whitelist.includes(msg.pubkey)) return true;
      // Allow messages sent by the user to whitelisted pubkeys
      if (msg.isSent && whitelist.includes(msg.recipientPubkey)) return true;
      return false;
   }).sort((a, b) => a.created_at - b.created_at);
});

function scrollToBottom() {
   nextTick(() => {
      if (chatContainer.value) {
         chatContainer.value.scrollTop = chatContainer.value.scrollHeight;
      }
   });
}

watch(messages, () => {
   scrollToBottom();
});

onMounted(async () => {
   scrollToBottom();
   // Cast through unknown to bypass type checking since we know the methods exist
   await ((nostrStore as unknown) as { checkAuthenticated: () => Promise<void> }).checkAuthenticated();
   await chatStore.init();
   
   // Update last read timestamp for whitelisted messages
   const whitelist = runtimeConfig.public.PUBKEY_WHITELIST || [];
   const latestMessageTime = messages.value.reduce((latest, msg) => {
      const isFromWhitelist = whitelist.includes(msg.pubkey);
      const isSentToWhitelist = msg.isSent && whitelist.includes(msg.recipientPubkey);
      if ((isFromWhitelist || isSentToWhitelist) && msg.created_at > latest) {
         return msg.created_at;
      }
      return latest;
   }, 0);

   if (latestMessageTime > 0) {
      nostrStore.$patch({ lastMessagesRead: latestMessageTime });
   }
});

async function sendMessage(message: string) {
   if (!message.trim() || isSending.value) return;

   try {
      isSending.value = true;
      // Cast through unknown to bypass type checking since we know the method exists
      await ((nostrStore as unknown) as { sendDirectMessage: (pubkey: string, content: string) => Promise<void> })
         .sendDirectMessage(runtimeConfig.public.MESSAGES_PUBKEY, message);
      scrollToBottom();
   } finally {
      isSending.value = false;
   }
}

function goBack() {
   if (window.history.length > 1) {
      window.history.back();
   } else {
      // Fallback: navigate to home if there is no history
      const router = useRouter();
      router.push('/');
   }
}

// Helper function to format display names
function formatDisplayName(name?: string | null, pubkey?: string | null): string {
   if (name) return name;
   if (pubkey) return pubkey.slice(0, 8);
   return 'Unknown';
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

// Watch for changes in messages and load properties
watch(() => messages.value, async (newMessages?: MessageWithProperty[]) => {
   if (newMessages) {
      for (const message of newMessages) {
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
</script>
