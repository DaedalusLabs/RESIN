<template>
   <div class="flex h-dvh flex-col bg-hex-white bg-cover bg-center">
      <main
         class="h-full flex-1 overflow-scroll"
         :class="{ 'pb-14': !$route.meta.hideBottomBar }"
      >
         <slot />
      </main>
      <MenuBar :show-drawer="showDrawer" @close="handleCloseDrawer" />
      <BottomBar
         v-if="!showDrawer && !$route.meta.hideBottomBar"
         @toggle-menu-bar="handleToggleMenuBar"
      />
   </div>
</template>

<script setup lang="ts">
import { useNostrStore } from "~/stores/nostr";
import { useChatStore } from "~/stores/chat";

const nostrStore = useNostrStore();
const chatStore = useChatStore();
const showDrawer = ref(false);

onMounted(async () => {
   if (nostrStore.authenticated) {
      await chatStore.init();
   }
});

const handleToggleMenuBar = () => {
   showDrawer.value = true;
};

const handleCloseDrawer = () => {
   showDrawer.value = false;
};
</script>
