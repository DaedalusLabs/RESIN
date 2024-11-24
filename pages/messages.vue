<template>
   <section class="flex flex-col justify-between">
      <!-- Header -->
      <div
         class="mb-5 flex w-full flex-col gap-5 border-b border-gray-200 bg-white px-10 pb-5 pt-10"
      >
         <NuxtLink :to="localePath('my-resin')">
            <PhCaretLeft :size="24" class="text-pirate-300" />
         </NuxtLink>
         <h1 class="text-2xl font-extrabold leading-tight text-pirate-950">
            Messages
         </h1>
      </div>

      <!-- Chat container -->
      <div class="mx-auto flex w-10/12 flex-1 flex-col gap-4 p-6">
         <!-- Incoming message -->
         <FlowbiteChatBubble
            message="Hello there,
I have found some real estate properties that might interest you. Please let me know which types of properties you are looking for.
Thank you"
            name="RESIN"
            status="Delivered"
            time="11:46"
            profile-image="/images/avatar.png"
         ></FlowbiteChatBubble>
      </div>

      <!-- Input box -->
      <div class="shadow-top flex w-full items-center bg-white px-3 py-5">
         <input
            type="text"
            placeholder="Type a message..."
            class="flex-grow rounded-lg border px-4 py-2"
         />
         <button
            class="ml-2 rounded-lg border-t bg-blue-500 px-4 py-2 text-white shadow-md"
         >
            Send
         </button>
      </div>
   </section>
</template>

<script setup>
import { PhCaretLeft } from "@phosphor-icons/vue";

definePageMeta({
   layout: "white",
   title: "Messages",
});
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
</style>
