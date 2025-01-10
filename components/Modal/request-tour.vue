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
         </p>
         <button
            class="rounded-lg bg-resin-500 px-6 py-3 text-sm font-semibold text-white"
            @click="handleSendRequest"
         >
            Send
         </button>
      </div>
   </FlowbiteModal>
</template>

<script setup>
import { PhCheck } from "@phosphor-icons/vue";
import { computed } from 'vue';

const nostrStore = useNostrStore();
const runtimeConfig = useRuntimeConfig();
const props = defineProps({
   isOpen: {
      type: Boolean,
      required: true
   },
   isRequestSent: {
      type: Boolean,
      required: true
   },
   property: {
      type: Object,
      required: true
   },
   referenceNumber: {
      type: Number,
      required: false
   }
});

const propertyAddress = computed(() => {
   return `${props.property.title} ${props.property.location.street}, ${props.property.location.city}, ${props.property.location.country}`;
});

const emit = defineEmits(['update:isOpen', 'sendRequest']);

const handleSendRequest = async () => {
      const contactMessage = `I want to request a tour to see ${propertyAddress.value}.`;
      
      await nostrStore.sendDirectMessage(runtimeConfig.public.MESSAGES_PUBKEY, contactMessage, props.property.id, props.property.kind);

      emit('sendRequest', {});
};
</script> 