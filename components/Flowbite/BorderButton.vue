<template>
   <button
      type="button"
      :class="`inline-flex items-center justify-center rounded-lg border border-${color} bg-transparent px-4 py-2 text-center text-sm font-medium text-${color} hover:bg-${color} hover:text-${hoverTextColor} focus:outline-none focus:ring-4 focus:ring-pirate-100`"
   >
      <span class="mr-2">
         <slot name="icon-prepend" />
      </span>

      {{ text }}

      <span class="ml-2">
         <slot name="icon-append" />
      </span>
   </button>
</template>

<script setup>
const props = defineProps({
   text: {
      type: String,
      required: true,
   },
   color: {
      type: String,
      default: 'resin-500',
   },
   hoverTextColor: {
      type: String,
      default: 'white',
   },
});
</script>
