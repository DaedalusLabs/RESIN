<template>
   <FlowbiteDrawer
      :is-open="showDrawer"
      slide-from="side"
      class="always-top"
      @close="handleCloseDrawer"
   >
      <template #title>
         <div class="mb-5 flex items-center justify-between text-pirate-300">
            <h2 class="text-base font-semibold">{{ $t('menu.account') }}</h2>
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
                  class="flex items-center space-x-2 px-4 py-3 text-red-600 cursor-pointer hover:underline"
                  @click="logout"   
               >
                  <PhSignOut :size="16" />
                  <span>{{ $t('menu.logout') }}</span>
               </NuxtLinkLocale>

               <div class="mt-4 px-4 text-pirate-500">
                  <NuxtLinkLocale to="/terms-and-conditions" @click="handleCloseDrawer">   
                     {{ $t('menu.termsAndConditions') }}
                  </NuxtLinkLocale>
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
const { t } = useI18n();
const localePath = useLocalePath()
const router = useRouter();
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
      label: t('menu.profile'),
      icon: PhUser,
      link: "/settings/profile",
   },
   {
      label: computed(() => {
         const count = nostrStore.unreadMessagesCount;
         return count > 0 ? t('menu.messagesWithCount', { count }) : t('menu.messages');
      }),
      icon: PhChatCircle,
      link: "/messages",
      get hasNotification() {
         return hasUnreadMessages.value;
      },
   },
   {
      label: t('menu.nostrKeys'),
      icon: PhKey,
      link: "/settings/nostr-keys",
   },
   {
      label: t('menu.settings'),
      icon: PhGear,
      link: "/settings",
   },
   {
      label: t('menu.help'),
      icon: PhQuestion,
      link: "/help",
   },
];

const handleCloseDrawer = () => {
   emit("close");
};

const logout = async () => {
   console.log('logout');
   try {
      await nostrStore.logout();
   } catch (error) {
      console.error('Error logging out', error);
   }
   window.location.href = localePath('/');
};
</script>

<style scoped>
.always-top {
   z-index: 9999;
}
</style>
