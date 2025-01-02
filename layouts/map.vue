<template>
   <div
      class="flex h-screen flex-col bg-pirate-950 bg-hex-dark bg-cover bg-center"
   >
      <main class="h-full flex-1">
         <slot />
      </main>
      <MenuBar :show-drawer="showDrawer" @close="showDrawer = false" />

      <BottomBar
         v-if="!showDrawer"
         class="hidden lg:block"
         @toggle-menu-bar="handleToggleMenuBar"
      />
   </div>
</template>

<script setup>
const showDrawer = ref(false);

const handleToggleMenuBar = () => {
   showDrawer.value = true;
};
</script>
