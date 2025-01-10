<template>
   <div :class="spinnerClasses" role="status">
      <div class="animate-spin rounded-full border-4 border-gray-200">
         <div class="h-full w-full rounded-full border-t-4 border-pirate-500"></div>
      </div>
      <span class="sr-only">Loading...</span>
   </div>
</template>

<script setup lang="ts">
const props = defineProps({
   size: {
      type: String,
      default: 'md',
      validator: (value: string) => ['sm', 'md', 'lg', 'xl'].includes(value),
   },
});

const sizeClasses = {
   sm: 'h-6 w-6',
   md: 'h-8 w-8',
   lg: 'h-12 w-12',
   xl: 'h-16 w-16',
};

const spinnerClasses = computed(() => [
   sizeClasses[props.size as keyof typeof sizeClasses],
]);
</script> 