<template>
   <FlowbiteModal :is-open="isOpen" @update:is-open="$emit('update:isOpen', $event)">
      <template #title>
         <h3 class="text-lg font-bold text-gray-900">Contact agent</h3>
      </template>
      <div class="mt-4">
         <p class="text-gray-900">Property</p>
         <p class="font-bold text-gray-900">{{ propertyAddress }}</p>
         <br />
         <p>
            Thank you for your interest in this property. Please leave your
            preferred contact information and our agent will contact you
            shortly.
         </p>
         <div class="mt-6">
            <FlowbiteTextInput
               label="Phone"
               placeholder="Phone"
               class="mb-4"
            />
         </div>
         <div class="mt-6 flex items-center justify-center">
            <div class="flex-grow border-t border-gray-300"></div>
            <span class="mx-3 text-sm font-medium text-gray-500">Or</span>
            <div class="flex-grow border-t border-gray-300"></div>
         </div>
         <div class="mt-6">
            <FlowbiteTextInput
               label="Email"
               placeholder="Email"
               class="mb-4"
            />
         </div>
         <div class="mt-6 flex justify-center">
            <FlowbiteButton :text="`Submit`" @click="$emit('sendRequest')" />
         </div>
      </div>
   </FlowbiteModal>
</template>

<script setup>
defineProps({
   isOpen: {
      type: Boolean,
      required: true
   },
   propertyAddress: {
      type: String,
      required: true
   }
});

defineEmits(['update:isOpen', 'sendRequest']);
</script> 