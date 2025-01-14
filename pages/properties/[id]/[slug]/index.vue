<!-- This is a wrapper component that reuses the property detail page -->
<template>
   <PropertyDetail />
</template>

<script setup>
import PropertyDetail from "../index.vue";

definePageMeta({
   layout: "white",
});
</script>
