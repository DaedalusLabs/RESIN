<template>
   <button
      type="button"
      class="inline-flex items-center rounded-lg bg-white bg-opacity-25 px-4 py-2 text-center text-sm font-medium text-pirate-300 hover:cursor-not-allowed"
   >
      {{ text }}

      <PhArrowRight :size="24" weight="bold" class="ml-3" />
   </button>
</template>

<script setup>
import { PhArrowRight } from "@phosphor-icons/vue";

defineProps({
   text: {
      type: String,
      required: true,
   },
});
</script>
