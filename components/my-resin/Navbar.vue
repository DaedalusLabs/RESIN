<template>
   <NuxtLinkLocale :to="link" class="block">
      <nav class="flex items-center justify-between">
         <h2 class="text-lg font-semibold text-pirate-800">{{ title }}</h2>
         <PhArrowRight :size="20" weight="bold" class="text-resin-500" />
      </nav>
   </NuxtLinkLocale>
</template>

<script setup>
import { PhArrowRight } from "@phosphor-icons/vue";

defineProps({
   title: {
      type: String,
      required: true,
   },
   link: {
      type: String,
      required: true,
   },
});
</script>
