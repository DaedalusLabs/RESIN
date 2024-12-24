<template>
  <Transition
    enter-active-class="transition duration-300 ease-out"
    enter-from-class="transform translate-y-2 opacity-0"
    enter-to-class="transform translate-y-0 opacity-100"
    leave-active-class="transition duration-200 ease-in"
    leave-from-class="transform translate-y-0 opacity-100"
    leave-to-class="transform translate-y-2 opacity-0"
  >
    <div
      v-if="show"
      class="fixed bottom-20 left-1/2 z-50 -translate-x-1/2 transform"
    >
      <div
        class="flex items-center gap-2 rounded-lg bg-green-100 px-4 py-2 text-sm font-medium text-green-800"
        role="alert"
      >
        <PhCheckCircle :size="20" weight="fill" />
        <span>{{ text }}</span>
      </div>
    </div>
  </Transition>
</template>

<script setup>
import { PhCheckCircle } from "@phosphor-icons/vue";

defineProps({
  show: {
    type: Boolean,
    required: true,
  },
  text: {
    type: String,
    required: true,
  },
});
</script> 