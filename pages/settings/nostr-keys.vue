<template>
   <section class="flex max-h-full flex-col justify-between">
      <!-- Header -->
      <div
         class="mb-5 flex w-full flex-col gap-5 border-b border-gray-200 bg-white px-10 pb-5 pt-10"
      >
         <NuxtLinkLocale @click="goBack">
            <PhCaretLeft :size="24" class="text-pirate-300" />
         </NuxtLinkLocale>
         <h1 class="text-2xl font-extrabold leading-tight text-pirate-950">
            {{ $t("settings.nostrKeys.title") }}
         </h1>
      </div>

      <!-- Keys Content -->
      <div class="flex-1 px-10">
         <div class="flex flex-col gap-6">
            <!-- Npub -->
            <div class="flex flex-col gap-2">
               <label class="text-sm font-medium text-pirate-950">
                  {{ $t("settings.nostrKeys.npub") }}
               </label>
               <div
                  class="flex items-center gap-2 rounded-lg border border-gray-300 p-2.5"
               >
                  <span
                     class="flex-1 overflow-hidden text-ellipsis font-mono text-sm text-pirate-950"
                  >
                     {{ npub }}
                  </span>
                  <button
                     class="text-resin-500 hover:text-resin-600"
                     @click="copyToClipboard(npub)"
                  >
                     <PhCopy :size="20" />
                  </button>
               </div>
               <div class="flex justify-center">
                  <div
                     class="inline-flex items-center justify-center rounded-lg border border-gray-300 p-2.5"
                  >
                     <QRCode
                        :data="npub"
                        :width="120"
                        :height="120"
                        :image="'/images/logos/Resin_Hexagon_Orange_Fill.svg'"
                     />
                  </div>
               </div>
            </div>

            <!-- Public Key -->
            <div class="flex flex-col gap-2">
               <label class="text-sm font-medium text-pirate-950">
                  {{ $t("settings.nostrKeys.publicKey") }}
               </label>
               <div
                  class="flex items-center gap-2 rounded-lg border border-gray-300 p-2.5"
               >
                  <span
                     class="flex-1 overflow-hidden text-ellipsis font-mono text-sm text-pirate-950"
                  >
                     {{ nostrStore.pubkey }}
                  </span>
                  <button
                     class="text-resin-500 hover:text-resin-600"
                     @click="copyToClipboard(nostrStore.pubkey)"
                  >
                     <PhCopy :size="20" />
                  </button>
               </div>
            </div>

            <!-- Recovery Phrase -->
            <div class="flex flex-col gap-2 pb-20">
               <div class="flex items-center justify-between">
                  <label
                     v-if="nostrStore.getTypeKey === 'mnemonic'"
                     class="text-sm font-medium text-pirate-950"
                  >
                     {{ $t("settings.nostrKeys.recoveryPhrase") }}
                  </label>
                  <label v-else class="text-sm font-medium text-pirate-950">
                     {{ $t("settings.nostrKeys.privateKey") }}
                  </label>

                  <button
                     v-if="nostrStore.getTypeKey === 'mnemonic'"
                     class="text-sm text-resin-500 hover:text-resin-600"
                     @click="showPhrase = !showPhrase"
                  >
                     {{
                        showPhrase
                           ? $t("settings.nostrKeys.hide")
                           : $t("settings.nostrKeys.show")
                     }}
                  </button>
               </div>

               <template v-if="nostrStore.getTypeKey === 'mnemonic'">
                  <div
                     v-if="showPhrase"
                     class="relative rounded-lg border border-gray-300 p-4"
                  >
                     <button
                        class="absolute right-4 top-4 text-resin-500 hover:text-resin-600"
                        @click="copyToClipboard(nostrStore.mnemonic)"
                     >
                        <PhCopy :size="20" />
                     </button>
                     <div class="grid grid-cols-2 gap-4">
                        <ol
                           class="list-decimal space-y-2 pl-6 text-xs text-pirate-300"
                           start="1"
                        >
                           <li
                              v-for="(word, index) in mnemonic.slice(0, 6)"
                              :key="index"
                           >
                              <span class="font-mono text-sm text-pirate-950">
                                 {{ word }}
                              </span>
                           </li>
                        </ol>
                        <ol
                           class="list-decimal space-y-2 pl-6 text-xs text-pirate-300"
                           start="7"
                        >
                           <li
                              v-for="(word, index) in mnemonic.slice(6, 12)"
                              :key="index + 6"
                           >
                              <span class="font-mono text-sm text-pirate-950">
                                 {{ word }}
                              </span>
                           </li>
                        </ol>
                     </div>
                  </div>
                  <div
                     v-else
                     class="relative rounded-lg border border-gray-300 p-4"
                  >
                     <div class="grid grid-cols-2 gap-4">
                        <ol
                           class="list-decimal space-y-2 pl-6 text-xs text-pirate-300"
                           start="1"
                        >
                           <li v-for="index in 6" :key="index">
                              <span class="font-mono text-sm text-pirate-300">
                                 ●●●●●●
                              </span>
                           </li>
                        </ol>
                        <ol
                           class="list-decimal space-y-2 pl-6 text-xs text-pirate-300"
                           start="7"
                        >
                           <li v-for="index in 6" :key="index + 6">
                              <span class="font-mono text-sm text-pirate-300">
                                 ●●●●●●
                              </span>
                           </li>
                        </ol>
                     </div>
                  </div>
               </template>

               <div v-else class="rounded-lg border border-gray-300 p-4">
                  <p class="text-sm text-pirate-600">
                     {{ $t("settings.nostrKeys.privateKeyNotAvailable") }}
                  </p>
               </div>
            </div>
         </div>
      </div>

      <!-- Alert -->
      <ResinAlert
         :show="showCopiedAlert"
         :text="$t('settings.nostrKeys.copied')"
      />
   </section>
</template>

<script setup>
import { useNostrStore } from "~/stores/nostr";
import { PhCaretLeft, PhCopy } from "@phosphor-icons/vue";
import { nip19 } from "nostr-tools";

const { t } = useI18n();

useHead({
   title: t("settings.nostrKeys.title"),
});

const nostrStore = useNostrStore();
const showPhrase = ref(false);
const showCopiedAlert = ref(false);

const mnemonic = computed(() => {
   return nostrStore.mnemonic ? nostrStore.mnemonic.split(" ") : [];
});

const npub = computed(() => {
   return nostrStore.pubkey ? nip19.npubEncode(nostrStore.pubkey) : "";
});

const copyToClipboard = async (text) => {
   try {
      await navigator.clipboard.writeText(text);
      showCopiedAlert.value = true;
      setTimeout(() => {
         showCopiedAlert.value = false;
      }, 2000);
   } catch (err) {
      console.error("Failed to copy text: ", err);
   }
};

const goBack = () => {
   if (window.history.length > 1) {
      window.history.back();
   } else {
      // Fallback: navigate to home if there is no history
      const { localePath } = useNuxtApp();
      if (typeof localePath === "function") {
         const path = localePath("home");
         if (typeof path === "string") {
            window.location.href = path;
         }
      }
   }
};

definePageMeta({
   layout: "white",
   middleware: ["auth"],
});
</script>
