<template>
   <div
      class="flex items-center justify-between border-b border-gray-200 bg-white p-4"
   >
      <div class="flex items-center gap-3">
         <img
            :src="
               userProfile?.image ||
               '/images/logos/Resin_Hexagon_Orange_Fill.svg'
            "
            class="h-10 w-10 rounded-full"
            alt="Profile"
         />
         <div>
            <h2 class="font-semibold">
               {{ displayName }}
            </h2>
            <p class="text-xs text-gray-500">
               {{ npub }}
            </p>
         </div>
      </div>
      <slot name="actions"></slot>
   </div>
</template>

<script setup>
import { nip19 } from "nostr-tools";

const props = defineProps({
   userProfile: {
      type: Object,
      default: () => ({}),
   },
   pubkey: {
      type: String,
      required: true,
   },
});

const displayName = computed(() => {
   return props.userProfile?.name || props.pubkey.slice(0, 8);
});

const npub = computed(() => {
   return nip19.npubEncode(props.pubkey);
});
</script>
