<template>
   <FlowbiteModal :is-open="isOpen" @update:is-open="$emit('update:isOpen', $event)">
      <template #title>
         <div v-if="isRequestSent" class="mb-5 flex items-center gap-2">
            <PhCheck size="24" class="text-resin-500" />
            <h3>Tour requested</h3>
         </div>
         <h3 v-else class="mb-5 mr-3 text-lg">
            Send tour request for {{ propertyAddress }}
         </h3>
      </template>
      <div v-if="isRequestSent">
         <p>
            One of our agents will contact you shortly to make an
            appointment. Make sure to check your messages inbox in your
            profile.
         </p>
      </div>
      <div
         v-else
         class="flex items-center justify-between rounded-lg border border-gray-300 p-3"
      >
         <p class="mr-3 text-sm text-pirate-950">
            I want to request a tour to see {{ propertyAddress }}.
            <span class="text-gray-500">REF {{ referenceNumber }}</span>
         </p>
         <button
            class="rounded-lg bg-resin-500 px-6 py-3 text-sm font-semibold text-white"
            @click="$emit('sendRequest')"
         >
            Send
         </button>
      </div>
   </FlowbiteModal>
</template>

<script setup>
import { PhCheck } from "@phosphor-icons/vue";

defineProps({
   isOpen: {
      type: Boolean,
      required: true
   },
   isRequestSent: {
      type: Boolean,
      required: true
   },
   propertyAddress: {
      type: String,
      required: true
   },
   referenceNumber: {
      type: Number,
      required: true
   }
});

defineEmits(['update:isOpen', 'sendRequest']);
</script> 