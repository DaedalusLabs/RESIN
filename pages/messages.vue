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
            v-for="msg in nostrStore.messages"
            :key="msg.id"
            :message="msg.content"
            :name="msg.user?.profile?.name ? msg.user?.profile?.name : ''"
            :status="msg.status"
            :time="(new Date(msg.created_at * 1000)).toLocaleString()"
            :profile-image="msg.user?.profile?.image ? msg.user?.profile?.image : '/images/logos/Resin_Hexagon_Orange_Fill.svg'"
            :is-sent="msg.isSent"
            :profile="msg.user"
         ></FlowbiteChatBubble>
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

<script setup>
import { PhCaretLeft, PhPaperPlaneTilt } from "@phosphor-icons/vue";
const nostrStore = useNostrStore();
const runtimeConfig = useRuntimeConfig();

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
      const { localePath } = useNuxtApp();
      window.location.href = localePath("home");
   }
};

const messages = ref([
]);

const newMessage = ref("");
const chatContainer = ref(null);
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

onMounted(async() => {
   scrollToBottom();
   await nostrStore.checkAuthenticated();
   await nostrStore.fetchDirectMessages();
   console.log(runtimeConfig.public.MESSAGES_NPUB);

   nostrStore.updateLastMessagesRead();
});

watch(messages, () => {
   scrollToBottom();
   
});

async function sendMessage() {
   if (!newMessage.value.trim() || isSending.value) return;

   try {
      isSending.value = true;
      await nostrStore.sendDirectMessage(runtimeConfig.public.MESSAGES_NPUB, newMessage.value);
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

function autoGrow(e) {
   const textarea = e.target;
   textarea.style.height = 'auto';
   const newHeight = Math.min(textarea.scrollHeight, 250); // 250px is approximately 10 rows
   textarea.style.height = newHeight + 'px';
}

function addNewline(e) {
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
