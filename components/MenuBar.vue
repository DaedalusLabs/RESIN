<template>
   <FlowbiteDrawer
      :is-open="showDrawer"
      slide-from="side"
      @close="handleCloseDrawer"
   >
      <template #title>
         <div class="mb-5 flex justify-between text-gray-400">
            <h2 class="text-base font-semibold">Account</h2>
            <PhX :size="16" @click="handleCloseDrawer" />
         </div>
      </template>

      <template #content>
         <div class="flex h-full flex-col justify-between px-4">
            <ul class="space-y-2 rounded-md border bg-white">
               <li class="border-b last:border-none">
                  <NuxtLink
                     class="group flex items-center space-x-3 px-4 py-3 hover:bg-gray-100"
                  >
                     <PhUser
                        :size="16"
                        class="text-gray-400 group-hover:text-resin-500"
                     />
                     <span class="text-pirate-500 group-hover:text-resin-500">
                        Profile
                     </span>
                  </NuxtLink>
               </li>
               <li class="border-b last:border-none">
                  <NuxtLink
                     :to="localePath('messages')"
                     class="group flex items-center space-x-3 px-4 py-3 hover:bg-gray-100"
                     @click="handleCloseDrawer"
                  >
                     <div class="relative flex items-center">
                        <PhChatCircle
                           :size="16"
                           class="text-gray-400 group-hover:text-resin-500"
                        />
                        <span
                           class="absolute right-0 top-0 h-2 w-2 rounded-full bg-resin-500"
                        ></span>
                     </div>
                     <span
                        class="flex items-center text-pirate-500 group-hover:text-resin-500"
                     >
                        Messages
                     </span>
                  </NuxtLink>
               </li>
               <li class="border-b last:border-none">
                  <NuxtLink
                     class="group flex items-center space-x-3 px-4 py-3 hover:bg-gray-100"
                  >
                     <PhKey
                        :size="16"
                        class="text-gray-400 group-hover:text-resin-500"
                     />
                     <span class="text-pirate-500 group-hover:text-resin-500">
                        NOSTR keys
                     </span>
                  </NuxtLink>
               </li>
               <li class="border-b last:border-none">
                  <NuxtLink
                     class="group flex items-center space-x-3 px-4 py-3 hover:bg-gray-100"
                  >
                     <PhGear
                        :size="16"
                        class="text-gray-400 group-hover:text-resin-500"
                     />
                     <span class="text-pirate-500 group-hover:text-resin-500">
                        Settings
                     </span>
                  </NuxtLink>
               </li>
               <li>
                  <NuxtLink
                     class="group flex items-center space-x-3 px-4 py-3 hover:bg-gray-100"
                  >
                     <PhQuestion
                        :size="16"
                        class="text-gray-400 group-hover:text-resin-500"
                     />
                     <span class="text-pirate-500 group-hover:text-resin-500">
                        Help
                     </span>
                  </NuxtLink>
               </li>
            </ul>

            <div class="mb-20">
               <a
                  href="#"
                  class="flex items-center space-x-2 px-4 py-3 text-red-600 hover:bg-gray-100"
               >
                  <PhSignOut :size="16" />
                  <span>Log out</span>
               </a>

               <div class="mt-4 px-4 text-gray-400">
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

const emit = defineEmits(["close"]);
defineProps({
   showDrawer: Boolean,
});

const handleCloseDrawer = () => {
   emit("close");
};
</script>
