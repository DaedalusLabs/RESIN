<!-- This is a wrapper component that reuses the rent-to-own page -->
<template>
   <RentToOwn />
</template>

<script setup>
import RentToOwn from "../rent-to-own.vue";

definePageMeta({
   layout: "white",
   middleware: ["auth"],
});
</script>
