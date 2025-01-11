<template>
   <div class="border-t border-gray-200 bg-white p-4">
      <div class="flex w-full items-center rounded-lg border bg-white">
         <textarea
            v-model="message"
            rows="1"
            :placeholder="$t('chat.typeMessage')"
            class="max-h-[250px] flex-grow resize-none overflow-y-auto rounded-l-lg border-none bg-transparent px-4 py-2 focus:outline-none"
            :disabled="isSending"
            @keydown.enter.exact.prevent="handleEnter"
            @keydown.shift.enter.prevent="addNewline"
            @input="(e) => autoGrow(e.target as HTMLTextAreaElement)"
         />
         <button
            class="rounded-r-lg border-none bg-transparent p-4 focus:outline-none disabled:opacity-50"
            :disabled="isSending"
            :title="$t('chat.send')"
            @click="handleSend"
         >
            <PhPaperPlaneTilt :size="20" />
         </button>
      </div>
   </div>
</template>

<script setup lang="ts">
import { PhPaperPlaneTilt } from "@phosphor-icons/vue";

const props = defineProps({
   isSending: {
      type: Boolean,
      default: false,
   },
});

const emit = defineEmits<{
   send: [message: string];
}>();

const message = ref("");

function autoGrow(textarea: HTMLTextAreaElement) {
   textarea.style.height = "auto";
   const newHeight = Math.min(textarea.scrollHeight, 250);
   textarea.style.height = newHeight + "px";
}

function addNewline() {
   message.value += "\n";
   const textarea = document.querySelector("textarea");
   if (textarea) {
      nextTick(() => autoGrow(textarea));
   }
}

function handleEnter(e: KeyboardEvent) {
   // Only send if not holding shift
   if (!e.shiftKey) {
      handleSend();
   } else {
      addNewline();
   }
}

function handleSend() {
   if (!message.value.trim() || props.isSending) return;

   // Remove trailing newlines
   const cleanMessage = message.value.replace(/\n+$/, "");
   emit("send", cleanMessage);
   message.value = "";

   // Reset textarea height
   const textarea = document.querySelector("textarea");
   if (textarea) {
      textarea.style.height = "auto";
   }
}
</script>
