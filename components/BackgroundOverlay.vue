<script setup>
defineProps({
   show: {
      type: Boolean,
      required: true,
   },
});

const emit = defineEmits(["close"]);
</script>

<template>
   <transition name="fade" appear>
      <div
         v-if="show"
         class="z-top fixed inset-0 w-full bg-black bg-opacity-50"
         @click.self="emit('close')"
      />
   </transition>
</template>

<style>
.fade-enter-active,
.fade-leave-active {
   transition: opacity 0.3s ease;
}
.fade-enter-from,
.fade-leave-to {
   opacity: 0;
}
</style>
