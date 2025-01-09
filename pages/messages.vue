<template>
   <section class="flex max-h-full flex-col justify-between">
      <!-- Header -->
      <div
         class="mb-5 flex w-full flex-col gap-5 border-b border-gray-200 bg-white px-10 pb-5 pt-10"
      >
         <NuxtLinkLocale @click="goBack">
            <PhCaretLeft :size="24" class="text-pirate-300" />
         </NuxtLinkLocale>
         <h1 class="text-2xl font-extrabold leading-tight text-pirate-950">
            {{ $t('messages.title') }}
         </h1>
      </div>

      <!-- Chat container -->
      <div
         ref="chatContainer"
         class="no-scrollbar mx-auto flex w-10/12 flex-1 flex-col gap-10 overflow-y-scroll pb-28"
      >
         <FlowbiteChatBubble
            v-for="msg in messages"
            :key="msg.id"
            :message="msg.content"
            :name="msg.userProfile?.name || ''"
            :time="(new Date(msg.created_at * 1000)).toLocaleString()"
            :profile-image="msg.userProfile?.image || '/images/logos/Resin_Hexagon_Orange_Fill.svg'"
            :is-sent="msg.isSent"
         />
      </div>

      <!-- Input  -->
      <div
         class="shadow-top absolute bottom-14 flex w-full items-center bg-white px-3 py-5"
      >
         <div class="flex w-full items-center rounded-lg border bg-white">
            <textarea
               v-model="newMessage"
               rows="1"
               :placeholder="$t('messages.typeMessage')"
               class="flex-grow rounded-l-lg border-none bg-transparent px-4 py-2 focus:outline-none resize-none overflow-y-auto max-h-[250px]"
               @keyup.enter.exact="sendMessage"
               @keydown.shift.enter.prevent="addNewline"
               @input="autoGrow"
               :disabled="isSending"
            />
            <button
               class="rounded-r-lg border-none bg-transparent p-2 focus:outline-none disabled:opacity-50"
               @click="sendMessage"
               :disabled="isSending"
               :title="$t('messages.send')"
            >
               <PhPaperPlaneTilt :size="16" />
            </button>
         </div>
      </div>
   </section>
</template>

<script setup lang="ts">
import { PhCaretLeft, PhPaperPlaneTilt } from "@phosphor-icons/vue";
import { useChatStore } from '~/stores/chat';
import { useNostrStore } from '~/stores/nostr';

const nostrStore = useNostrStore();
const chatStore = useChatStore();
const runtimeConfig = useRuntimeConfig();

// Use whitelistedChats instead of all chats
definePageMeta({
   layout: "white",
   title: "Messages",
   middleware: ['auth']
});

const goBack = () => {
   if (window.history.length > 1) {
      window.history.back();
   } else {
      // Fallback: navigate to home if there is no history
      const router = useRouter();
      router.push('/');
   }
};

const messages = computed(() => {
   const chat = chatStore.chats.find(c => c.pubkey === runtimeConfig.public.MESSAGES_NPUB);
   return chat?.messages || [];

   const whitelist = runtimeConfig.public.PUBKEY_WHITELIST || [];
   // whitelist.push(nostrStore.pubkey);
   // console.log(nostrStore.pubkey, whitelist);
   const allMessages = chatStore.chats.flatMap(chat => chat.messages);
   
   return allMessages.filter(msg => {
      // Allow messages from whitelisted pubkeys
      if (whitelist.includes(msg.pubkey)) return true;
      // Allow messages sent by the user to whitelisted pubkeys
      if (msg.isSent && whitelist.includes(msg.recipientPubkey)) return true;
      console.log('msg', msg);
      return false;
   }).sort((a, b) => a.created_at - b.created_at);
});

const newMessage = ref("");
const chatContainer = ref<HTMLElement | null>(null);
const isSending = ref(false);

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
   await ((nostrStore as unknown) as { checkAuthenticated: () => Promise<boolean> }).checkAuthenticated();
   await chatStore.init();
   await chatStore.fetchChats();
   ((nostrStore as unknown) as { updateLastMessagesRead: () => void }).updateLastMessagesRead();
});

async function sendMessage() {
   if (!newMessage.value.trim() || isSending.value) return;

   try {
      isSending.value = true;
      // Cast through unknown to bypass type checking since we know the method exists
      await ((nostrStore as unknown) as { sendDirectMessage: (pubkey: string, content: string) => Promise<void> })
         .sendDirectMessage(runtimeConfig.public.MESSAGES_NPUB, newMessage.value);
      newMessage.value = "";
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
   const newHeight = Math.min(textarea.scrollHeight, 250); // 250px is approximately 10 rows
   textarea.style.height = newHeight + 'px';
}

function addNewline(e: Event) {
   newMessage.value += '\n';
   nextTick(() => autoGrow(e));
}
</script>

<style scoped>
section {
   height: calc(94vh);
}

.shadow-top {
   box-shadow:
      0px -4px 6px -1px rgba(0, 0, 0, 0.1),
      0px -2px 4px -2px rgba(0, 0, 0, 0.1);
}

.no-scrollbar::-webkit-scrollbar {
   display: none;
}

.no-scrollbar {
   -ms-overflow-style: none;
   scrollbar-width: none;
}
</style>
