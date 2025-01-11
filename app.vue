<template>
   <div>
      <VitePwaManifest />

      <!-- Maintenance Notice -->
      <ResinNotificationBar
         :show="maintenanceMode"
         position="top"
         color="resin"
         :title="$t('maintenance.title')"
         :description="$t('maintenance.description')"
         :countdown="$t('maintenance.nextRetry', { seconds: retryCountdown })"
      />

      <!-- Nostr Loading Notice -->
      <ResinNotificationBar
         :show="propertiesStore.isLoadingNostr"
         position="top"
         color="resin"
         :title="$t('nostr.loading.title')"
         :description="$t('nostr.loading.description')"
      />

      <!-- Chat Processing Notice -->
      <ResinNotificationBar
         :show="chatStore.isProcessing"
         position="top"
         color="blue"
         :title="$t('chat.processing.title')"
         :description="
            $t('chat.processing.description', {
               count: chatStore.processingCount,
            })
         "
      />

      <NuxtLayout>
         <NuxtPage />
      </NuxtLayout>

      <!-- Relay Status Overlay -->
      <div
         v-if="nostrStore.authenticated && showRelays"
         class="fixed bottom-4 right-4 z-50 max-w-sm rounded-lg bg-white p-4 text-sm shadow-lg"
      >
         <div class="flex items-center justify-between">
            <h3 class="font-semibold">
               Connected Relays ({{ connectedRelays.length }})
            </h3>
            &nbsp;
            <button
               class="text-gray-500 hover:text-gray-700"
               @click="showRelays = !showRelays"
            >
               <span v-if="showRelays">Hide</span>
               <span v-else>Show</span>
            </button>
         </div>
         <div v-if="showRelays" class="space-y-2">
            <div
               v-for="relay in connectedRelays"
               :key="relay"
               class="flex items-center gap-2"
            >
               <div class="h-2 w-2 rounded-full bg-green-500"></div>
               <span class="truncate text-xs" :title="relay">{{ relay }}</span>
            </div>
            <div
               v-if="connectedRelays.length === 0"
               class="text-xs text-gray-500"
            >
               No relays connected
            </div>
         </div>
      </div>
   </div>
</template>

<script setup>
import { usePropertiesStore } from "~/stores/properties";
import { useNostrStore } from "~/stores/nostr";
import { useChatStore } from "~/stores/chat";
import { useNDK } from "~/composables/useNDK";

const propertiesStore = usePropertiesStore();
const nostrStore = useNostrStore();
const chatStore = useChatStore();
const showRelays = ref(false);
const altKeyPressCount = ref(0);
const altKeyTimer = ref(null);
const maintenanceMode = ref(false);
const retryCountdown = ref(10);
const retryTimer = ref(null);

const TRIPLE_PRESS_TIMEOUT = 500; // 500ms timeout for triple press

const handleKeyDown = (event) => {
   if (event.key === "Alt") {
      altKeyPressCount.value++;

      // Clear existing timer
      if (altKeyTimer.value) {
         clearTimeout(altKeyTimer.value);
      }

      // Set new timer
      altKeyTimer.value = setTimeout(() => {
         if (altKeyPressCount.value >= 3) {
            showRelays.value = !showRelays.value;
         }
         altKeyPressCount.value = 0;
      }, TRIPLE_PRESS_TIMEOUT);
   }
};

propertiesStore.init();

const connectedRelays = computed(() => {
   const ndk = useNDK();
   if (!ndk) return [];

   return Array.from(ndk.pool.relays.values())
      .filter((relay) => relay.connected)
      .map((relay) => relay.url);
});

useHead({
   titleTemplate: (title) => (title ? `${title} | Resin` : "Resin"),
});

onMounted(() => {
   window.addEventListener("keydown", handleKeyDown);
   propertiesStore.watchNostrAuth();
});

onUnmounted(() => {
   window.removeEventListener("keydown", handleKeyDown);
   if (altKeyTimer.value) {
      clearTimeout(altKeyTimer.value);
   }
   if (retryTimer.value) {
      clearInterval(retryTimer.value);
   }
});

nostrStore.checkAuthenticated().then(async () => {
   if (nostrStore.authenticated) {
      await propertiesStore.loadNostrPreferences();
      await nostrStore.fetchDirectMessages();
   }
});
</script>

<style>
::selection {
   color: white;
   background: #f07e19;
}

html {
   scroll-behavior: smooth;
}

body {
   touch-action: pan-y;
   overflow: hidden;
}

/* font Inter for all text */
* {
   font-family: "Inter", sans-serif;
   font-optical-sizing: auto;
   font-style: normal;
   font-variation-settings: "slnt" 0;
}

/* clears the 'X' from Internet Explorer */
input[type="search"]::-ms-clear {
   display: none;
   width: 0;
   height: 0;
}
input[type="search"]::-ms-reveal {
   display: none;
   width: 0;
   height: 0;
} /* clears the 'X' from Chrome */
input[type="search"]::-webkit-search-decoration,
input[type="search"]::-webkit-search-cancel-button,
input[type="search"]::-webkit-search-results-button,
input[type="search"]::-webkit-search-results-decoration {
   display: none;
}

.z-top {
   z-index: 1000;
}
</style>
