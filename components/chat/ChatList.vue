<template>
   <div class="flex w-96 flex-col border-r border-gray-200 bg-white">
      <!-- Search -->
      <div class="border-b border-gray-200 p-4">
         <div class="relative">
            <input
               v-model="searchQuery"
               type="text"
               :placeholder="$t('chat.search')"
               class="w-full rounded-lg border border-gray-300 bg-gray-50 p-2.5 pl-10 text-sm text-gray-900 focus:border-gray-500 focus:ring-0"
            />
            <PhMagnifyingGlass
               class="absolute left-3 top-2.5 text-gray-500"
               :size="20"
            />
         </div>
      </div>
      <!-- Conversations List -->
      <div class="flex-1 overflow-y-auto">
         <div
            v-for="chat in filteredChats"
            :key="chat.pubkey"
            class="group relative cursor-pointer border-b border-gray-100 p-4 hover:bg-gray-50"
            :class="{ 'bg-gray-100': selectedPubkey === chat.pubkey }"
            @click="$emit('select', chat)"
         >
            <div class="flex items-center gap-3">
               <img
                  :src="
                     chat.userProfile?.image ||
                     '/images/logos/Resin_Hexagon_Orange_Fill.svg'
                  "
                  class="h-12 w-12 rounded-full"
                  alt="Profile"
               />
               <div class="min-w-0 flex-1">
                  <div class="flex items-center justify-between">
                     <h3 class="truncate font-semibold">
                        {{
                           formatDisplayName(
                              chat.userProfile?.name,
                              chat.pubkey,
                           )
                        }}
                     </h3>
                     <span class="text-xs text-gray-500">
                        {{ formatTime(chat.lastMessage?.created_at) }}
                     </span>
                  </div>
                  <p class="mt-1 truncate text-xs text-gray-500">
                     {{ pubkeyToNpub(chat.pubkey) }}
                  </p>
                  <p class="mt-1 line-clamp-1 text-sm text-gray-600">
                     {{ chat.lastMessage?.content || "" }}
                  </p>
               </div>
            </div>
            <!-- Hide chat button -->
            <button
               v-if="showHideButton"
               class="absolute right-2 top-2 p-1 text-gray-400 opacity-0 transition-opacity hover:text-gray-600 group-hover:opacity-100"
               title="Hide chat"
               @click.stop="$emit('hide', chat.pubkey)"
            >
               <PhX :size="16" />
            </button>
         </div>
      </div>
   </div>
</template>

<script setup lang="ts">
import { PhMagnifyingGlass, PhX } from "@phosphor-icons/vue";
import { nip19 } from "nostr-tools";
import type { Chat } from "~/stores/chat";

const props = defineProps({
   chats: {
      type: Array as PropType<Chat[]>,
      required: true,
   },
   selectedPubkey: {
      type: String,
      default: null,
   },
   showHideButton: {
      type: Boolean,
      default: false,
   },
});

const _emit = defineEmits<{
   select: [chat: Chat];
   hide: [pubkey: string];
}>();

const searchQuery = ref("");

const filteredChats = computed(() => {
   let chats = props.chats;

   if (searchQuery.value) {
      const query = searchQuery.value.toLowerCase();
      chats = chats.filter((chat) => {
         const name = chat.userProfile?.name?.toLowerCase() || "";
         const pubkey = chat.pubkey.toLowerCase();
         return name.includes(query) || pubkey.includes(query);
      });
   }

   return chats;
});

function formatTime(timestamp: number | undefined): string {
   if (!timestamp) return "";

   const date = new Date(timestamp * 1000);
   const now = new Date();

   if (date.toDateString() === now.toDateString()) {
      return date.toLocaleTimeString(undefined, {
         hour: "2-digit",
         minute: "2-digit",
      });
   }

   if (now.getFullYear() === date.getFullYear()) {
      return date.toLocaleDateString(undefined, {
         month: "short",
         day: "numeric",
      });
   }

   return date.toLocaleDateString(undefined, {
      year: "numeric",
      month: "short",
      day: "numeric",
   });
}

function formatDisplayName(
   name: string | undefined | null,
   pubkey: string | undefined | null,
): string {
   if (name) return name;
   if (pubkey) return pubkey.slice(0, 8);
   return "Unknown";
}

function pubkeyToNpub(pubkey: string | undefined | null): string {
   if (!pubkey) return "";
   return nip19.npubEncode(pubkey);
}
</script>

<style scoped>
.line-clamp-1 {
   display: -webkit-box;
   -webkit-line-clamp: 1;
   -webkit-box-orient: vertical;
   overflow: hidden;
}
</style>
