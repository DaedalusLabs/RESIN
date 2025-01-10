<template>
   <div>
      <VitePwaManifest />
      <!-- Maintenance Notice -->
      <Transition
         enter-active-class="transition duration-300 ease-out"
         enter-from-class="transform -translate-y-full opacity-0"
         enter-to-class="transform translate-y-0 opacity-100"
         leave-active-class="transition duration-200 ease-in"
         leave-from-class="transform translate-y-0 opacity-100"
         leave-to-class="transform -translate-y-full opacity-0"
      >
         <div v-if="maintenanceMode" class="fixed inset-x-0 top-0 z-50 bg-gradient-to-r from-resin-500 to-resin-600 text-white px-4 py-3 shadow-lg">
            <div class="container mx-auto flex items-center justify-between">
               <div class="flex items-center gap-3">
                  <div class="animate-spin">
                     <PhSpinner :size="24" weight="bold" />
                  </div>
                  <div>
                     <p class="font-medium">{{ $t('maintenance.title') }}</p>
                     <p class="text-sm opacity-90">{{ $t('maintenance.description') }}</p>
                  </div>
               </div>
               <div class="text-sm opacity-90">
                  {{ $t('maintenance.nextRetry', { seconds: retryCountdown }) }}
               </div>
            </div>
         </div>
      </Transition>

      <NuxtLayout>
         <NuxtPage />
      </NuxtLayout>
      
      <!-- Relay Status Overlay -->
      <div v-if="nostrStore.authenticated && showRelays" class="fixed bottom-4 right-4 z-50 max-w-sm bg-white rounded-lg shadow-lg p-4 text-sm">
         <div class="flex items-center justify-between">
            <h3 class="font-semibold">Connected Relays ({{ connectedRelays.length }})</h3>
            &nbsp;
            <button @click="showRelays = !showRelays" class="text-gray-500 hover:text-gray-700">
               <span v-if="showRelays">Hide</span>
               <span v-else>Show</span>
            </button>
         </div>
         <div v-if="showRelays" class="space-y-2">
            <div v-for="relay in connectedRelays" :key="relay" class="flex items-center gap-2">
               <div class="w-2 h-2 rounded-full bg-green-500"></div>
               <span class="text-xs truncate" :title="relay">{{ relay }}</span>
            </div>
            <div v-if="connectedRelays.length === 0" class="text-gray-500 text-xs">
               No relays connected
            </div>
         </div>
      </div>
   </div>
</template>

<script setup>
import { usePropertiesStore } from "~/stores/properties";
import { useNostrStore } from "~/stores/nostr";
import { useNDK } from "~/composables/useNDK";
import { PhSpinner } from "@phosphor-icons/vue";

const runtimeConfig = useRuntimeConfig();
const propertiesStore = usePropertiesStore();
const nostrStore = useNostrStore();
const showRelays = ref(false);
const altKeyPressCount = ref(0);
const altKeyTimer = ref(null);
const maintenanceMode = ref(false);
const retryCountdown = ref(10);
const retryTimer = ref(null);

const TRIPLE_PRESS_TIMEOUT = 500; // 500ms timeout for triple press
const RETRY_INTERVAL = 10000; // 10 seconds

const handleKeyDown = (event) => {
   if (event.key === 'Alt') {
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
      .filter(relay => relay.connected)
      .map(relay => relay.url);
});

useHead({
   titleTemplate: (title) => title ? `${title} | Resin` : 'Resin',
});

onMounted(() => {
   window.addEventListener('keydown', handleKeyDown);
   propertiesStore.watchNostrAuth();
});

onUnmounted(() => {
   window.removeEventListener('keydown', handleKeyDown);
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

async function getProperties() {
   try {
      const properties = await fetch(`${runtimeConfig.public.BACKEND_ENDPOINT}/listings`);
      if (!properties.ok) throw new Error('Failed to fetch properties');
      const data = await properties.json();
      maintenanceMode.value = false;
      if (retryTimer.value) {
         clearInterval(retryTimer.value);
         retryTimer.value = null;
      }
      return data.hits.map(d => d.document);
   } catch (error) {
      console.error('Failed to fetch properties:', error);
      maintenanceMode.value = true;
      startRetryCountdown();
      return [];
   }
}

function startRetryCountdown() {
   if (retryTimer.value) {
      clearInterval(retryTimer.value);
   }
   
   retryCountdown.value = 10;
   retryTimer.value = setInterval(async () => {
      retryCountdown.value--;
      if (retryCountdown.value <= 0) {
         propertiesStore.properties = await getProperties();
         retryCountdown.value = 10;
      }
   }, 1000);
}

propertiesStore.properties = await getProperties();
</script>

<style>
::-moz-selection {
   /* Code for Firefox */
   color: white;
   background: #f07e19;
}

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

/* clears the ‘X’ from Internet Explorer */
input[type="search"]::-ms-clear {
   display: none;
   width: 0;
   height: 0;
}
input[type="search"]::-ms-reveal {
   display: none;
   width: 0;
   height: 0;
} /* clears the ‘X’ from Chrome */
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
