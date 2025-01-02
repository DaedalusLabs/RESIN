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
            Messages
         </h1>
      </div>

      <!-- Chat container -->
      <div
         ref="chatContainer"
         class="no-scrollbar mx-auto flex w-10/12 flex-1 flex-col gap-10 overflow-y-scroll p-6 pb-28"
      >
         <FlowbiteChatBubble
            v-for="(msg, index) in messages"
            :key="index"
            :message="msg.text"
            :name="msg.sender"
            :status="msg.status"
            :time="msg.time"
            :profile-image="msg.profileImage"
            :is-sent="msg.isSent"
         ></FlowbiteChatBubble>
      </div>

      <!-- Input  -->
      <div
         class="shadow-top absolute bottom-14 flex w-full items-center bg-white px-3 py-5"
      >
         <div class="flex w-full items-center rounded-lg border bg-white">
            <input
               v-model="newMessage"
               type="text"
               placeholder="Type a message..."
               class="flex-grow rounded-l-lg border-none bg-transparent px-4 py-2 focus:outline-none"
               @keyup.enter="sendMessage"
            />
            <button
               class="rounded-r-lg border-none bg-transparent p-2 focus:outline-none"
               @click="sendMessage"
            >
               <PhPaperPlaneTilt :size="16" />
            </button>
         </div>
      </div>
   </section>
</template>

<script setup>
import { PhCaretLeft, PhPaperPlaneTilt } from "@phosphor-icons/vue";

definePageMeta({
   layout: "white",
   title: "Messages",
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
   {
      text: "Hello there, I have found some real estate properties that might interest you. Please let me know which types of properties you are looking for. Thank you",
      sender: "RESIN",
      status: "Delivered",
      time: "11:46",
      profileImage: "/images/avatar.png",
      isSent: false,
   },
   {
      text: "Yes, we do have several options available in the downtown area. I will send you the details and some pictures of the properties shortly.",
      sender: "RESIN",
      status: "Delivered",
      time: "11:46",
      profileImage: "/images/avatar.png",
      isSent: true,
   },
]);

const newMessage = ref("");
const chatContainer = ref(null);

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

onMounted(() => {
   scrollToBottom();
});

watch(messages, () => {
   scrollToBottom();
});

function sendMessage() {
   if (!newMessage.value.trim()) return; // Geen lege berichten toestaan

   const now = new Date();
   const time = now.toLocaleTimeString([], {
      hour: "2-digit",
      minute: "2-digit",
   });

   messages.value.push({
      text: newMessage.value,
      sender: "You",
      status: "Sent",
      time: time,
      profileImage: "/images/avatar.png",
      isSent: true,
   });

   newMessage.value = "";

   // Scroll naar de onderkant na het toevoegen van een bericht
   scrollToBottom();
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
