<template>
   <div class="flex h-dvh flex-col bg-hex-white bg-cover bg-center">
      <main class="h-full flex-1 overflow-scroll pb-14">
         <slot />
      </main>
      <MenuBar :show-drawer="showDrawer" @close="showDrawer = false" />
      <BottomBar v-if="!showDrawer" @toggle-menu-bar="handleToggleMenuBar" />
   </div>
</template>

<script setup>
const showDrawer = ref(false);

const handleToggleMenuBar = () => {
   showDrawer.value = true;
};
</script>
