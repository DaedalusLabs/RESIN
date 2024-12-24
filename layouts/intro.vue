<script setup>
const isDesktop = ref(false);

onMounted(() => {
   isDesktop.value = window.innerWidth >= 1024; // same as lg breakpoint from tailwind
});
</script>

<template>
   <div
      class="flex h-screen flex-col bg-pirate-950 bg-hex-dark bg-cover bg-center"
   >
      <main class="flex h-full flex-1">
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
         <div class="h-full flex-1">
            <slot />
         </div>
      </main>
   </div>
</template>
