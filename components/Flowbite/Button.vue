<template>
   <button
      type="button"
      class="inline-flex items-center rounded-lg px-4 py-2 text-center text-sm font-medium text-white focus:outline-none"
      :class="{
         'bg-resin-500 hover:bg-resin-600': !disabled,
         'cursor-not-allowed bg-gray-500': disabled,
      }"
      :disabled="disabled"
   >
      <PhSignIn v-if="isLogin" :size="24" class="mr-3" />

      {{ text }}

      <PhArrowRight v-if="showIcon" :size="24" weight="bold" class="ml-3" />
   </button>
</template>

<script setup>
import { PhSignIn, PhArrowRight } from "@phosphor-icons/vue";

defineProps({
   text: {
      type: String,
      required: true,
   },
   isLogin: {
      type: Boolean,
      default: false,
   },
   showIcon: {
      type: Boolean,
      default: true,
   },
   disabled: {
      type: Boolean,
      default: false,
   },
});
</script>
