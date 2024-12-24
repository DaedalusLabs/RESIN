<template>
   <FlowbiteDrawer
      :is-open="showDrawer"
      slide-from="side"
      class="always-top"
      @close="handleCloseDrawer"
   >
      <template #title>
         <div class="mb-5 flex items-center justify-between text-pirate-300">
            <h2 class="text-base font-semibold">Account</h2>
            <PhX :size="20" class="cursor-pointer" @click="handleCloseDrawer" />
         </div>
      </template>

      <template #content>
         <div class="flex h-full flex-col justify-between px-4">
            <ul class="rounded-md border bg-white">
               <li
                  v-for="(item, index) in menuItems"
                  :key="index"
                  class="border-b last:border-none"
               >
                  <NuxtLinkLocale
                     :to="item.link"
                     class="flex items-center gap-2 px-4 py-4 hover:bg-gray-100"
                     @click="handleCloseDrawer"
                  >
                     <div class="relative flex items-center">
                        <component
                           :is="item.icon"
                           :size="16"
                           class="text-pirate-500 group-hover:text-resin-500"
                        />
                        <span
                           v-show="item.hasNotification"
                           class="absolute right-0 top-0 h-2 w-2 rounded-full bg-resin-500 text-sm font-medium"
                        ></span>
                     </div>
                     <span class="text-pirate-500 group-hover:text-resin-500">
                        {{ item.label }}
                     </span>
                  </NuxtLinkLocale>
               </li>
            </ul>

            <div class="mb-10">
               <NuxtLinkLocale
                  class="flex items-center space-x-2 px-4 py-3 text-red-600 hover:text-pirate-500"
                  @click="logout"   
               >
                  <PhSignOut :size="16" />
                  <span>Log out</span>
               </NuxtLinkLocale>

               <div class="mt-4 px-4 text-pirate-500">
                  <a href="#" class="text-sm">Terms & conditions</a>
               </div>
            </div>
         </div>
      </template>
   </FlowbiteDrawer>
</template>

<script setup>

import {
   PhUser,
   PhChatCircle,
   PhKey,
   PhGear,
   PhQuestion,
   PhX,
   PhSignOut,
} from "@phosphor-icons/vue";
const nostrStore = useNostrStore();


const emit = defineEmits(["close"]);
defineProps({
   showDrawer: Boolean,
});

const hasUnreadMessages = computed(() => {
   const count = nostrStore.unreadMessagesCount;
   return count > 0;
});

const menuItems = [
   {
      label: computed(() => {
         const count = nostrStore.unreadMessagesCount;
         return count > 0 ? `Messages (${count})` : 'Messages';
      }),
      icon: PhChatCircle,
      link: "/messages",
      get hasNotification() {
         return hasUnreadMessages.value;
      },
   },
   {
      label: "Nostr Keys",
      icon: PhKey,
      link: "/settings/nostr-keys",
   },
   {
      label: "Settings",
      icon: PhGear,
      link: "/settings",
   },
];

const handleCloseDrawer = () => {
   emit("close");
};




const logout = () => {
   nostrStore.logout();
   navigateTo('/');
};
</script>

<style scoped>
.always-top {
   z-index: 9999;
}
</style>
