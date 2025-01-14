<template>
   <div class="flex items-center justify-between gap-4">
      <div class="flex items-center gap-4">
         <PhFile :size="28" />
         <div class="">
            <h3 class="text-lg font-semibold text-pirate-950">
               {{ agreement.title }}
            </h3>
            <p class="text-xs text-gray-500">
               {{
                  agreement.isSigned
                     ? `Signed ${formatDate(agreement.signedDate)}`
                     : "Not signed"
               }}
            </p>
         </div>
      </div>

      <NuxtLink :to="agreement.url" class="text-sm font-bold text-resin-500">
         Download
      </NuxtLink>
   </div>
</template>

<script setup lang="ts">
import { PhFile } from "@phosphor-icons/vue";
import type { Agreement } from "~/stores/transactions";

defineProps({
   agreement: {
      type: Object as PropType<Agreement>,
      required: true,
   },
});

function formatDate(dateString: string) {
   const date = new Date(dateString);
   const day = date.getDate().toString().padStart(2, "0");
   const month = (date.getMonth() + 1).toString().padStart(2, "0"); // +1 because months are 0-indexed
   const year = date.getFullYear();

   return `${day}/${month}/${year}`;
}
</script>
