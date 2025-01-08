<script setup>
const isDesktop = ref(false);

onMounted(() => {
   isDesktop.value = window.innerWidth >= 1024; // same as lg breakpoint from tailwind
});
</script>

<template>
   <div
      class="fixed inset-0 bg-pirate-950 bg-hex-dark bg-cover bg-center"
   >
      <main class="relative h-screen flex">
         <div class="hidden h-full w-3/5 lg:block">
            <NuxtImg
               v-if="isDesktop"
               class="h-full w-full object-cover"
               src="/images/introduction-desktop.webp"
               alt="background"
               placeholder
               loading="lazy"
               width="50vw"
               height="100vw"
            />
         </div>
         <div class="h-full flex-1 overflow-y-auto">
            <slot />
         </div>
      </main>
   </div>
</template>
