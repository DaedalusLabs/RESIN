<template>
   <section class="space-y-2">
      <h3 class="font-semibold text-pirate-800">{{ title }}</h3>
      <div class="rounded-lg bg-gray-100 p-4 text-pirate-900">
         We are building this feature
      </div>
   </section>
</template>

<script setup>
defineProps({
   title: {
      type: String,
      required: true,
   },
});
</script>
