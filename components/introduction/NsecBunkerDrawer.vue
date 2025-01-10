<template>
   <FlowbiteDrawer :is-open="show" class="p-4 pb-20 z-50" @close="handleClose">
      <template #title>
         <h3 class="text-xl font-semibold text-pirate-950">
            {{ t('introduction.nsecBunker.title') }}
         </h3>
      </template>
      <template #content>
         <div class="mx-auto mt-3 flex flex-col gap-5">
            <div v-if="isLoading" class="text-center">
               <p>{{ t('introduction.nsecBunker.connecting') }}</p>
               <div class="mt-4">
                  <FlowbiteSpinner size="lg" />
               </div>
            </div>
            <div v-else-if="error" class="text-center text-red-500">
               <p>{{ error }}</p>
               <FlowbiteButton
                  text="Try Again"
                  class="mt-4 px-5 py-3"
                  @click="resetState"
               />
            </div>
            <div v-else>
               <p class="text-center mb-4">{{ t('introduction.nsecBunker.scanQr') }}</p>
               
               <!-- QR Code Section -->
               <div class="flex flex-col items-center gap-4">
                  <div class="relative bg-white p-4 rounded-lg">
                     <QRCode
                        :data="connectionString"
                        :width="200"
                        :height="200"
                        :image="'/images/logos/Resin_Hexagon_Orange_Fill.svg'"
                     />
                  </div>
                  <div class="w-full rounded-lg bg-gray-100 p-4">
                     <p class="mb-2 text-sm font-medium text-gray-500">{{ t('introduction.nsecBunker.connectString') }}</p>
                     <div class="flex items-center gap-2">
                        <code class="flex-1 break-all text-sm">{{ connectionString }}</code>
                        <button 
                           class="text-pirate-500 hover:text-pirate-600"
                           @click="copyToClipboard(connectionString)"
                        >
                           <PhCopy :size="20" />
                        </button>
                     </div>
                  </div>
               </div>

               <!-- Divider -->
               <div class="relative my-6 text-center">
                  <div class="absolute inset-0 flex items-center">
                     <div class="w-full border-t border-gray-300"></div>
                  </div>
                  <span class="relative bg-white px-4 text-sm text-gray-500">{{ t('or') }}</span>
               </div>

               <!-- Manual URL Input -->
               <div class="flex flex-col gap-4">
                  <p class="text-center">{{ t('introduction.nsecBunker.enterUrl') }}</p>
                  <FlowbiteTextInput
                     v-model="bunkerUrl"
                     :placeholder="t('introduction.nsecBunker.urlPlaceholder')"
                  />
                  <FlowbiteButton
                     text="Connect"
                     class="mt-2 w-full px-5 py-3"
                     @click="handleBunkerUrl"
                  />
               </div>
            </div>
         </div>
      </template>
   </FlowbiteDrawer>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { useNostrStore } from '~/stores/nostr';
import { PhCopy } from '@phosphor-icons/vue';

const props = defineProps<{
   show: boolean;
}>();

const emit = defineEmits<{
   (e: 'close'): void;
}>();

const nostrStore = useNostrStore();
const { t } = useI18n();
const isLoading = ref(false);
const error = ref<string | null>(null);
const bunkerUrl = ref<string>('');
const secret = ref(generateRandomString(8)); // Generate a random secret for this session
const connectionString = computed(() => nostrStore.nostrConnectUrl || ''); // Provide empty string as fallback

// Generate a random string for the secret
function generateRandomString(length: number): string {
   const chars = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789';
   let result = '';
   for (let i = 0; i < length; i++) {
      result += chars.charAt(Math.floor(Math.random() * chars.length));
   }
   return result;
}

// Handle manual URL input
async function handleBunkerUrl() {
   if (!bunkerUrl.value) return;
   
   try {
      const url = new URL(bunkerUrl.value);
      const pubkey = url.searchParams.get('public_key');
      if (!pubkey) {
         error.value = t('introduction.nsecBunker.invalidResponse');
         return;
      }
      
      await startNsecBunkerLogin(pubkey);
   } catch (err) {
      error.value = t('introduction.nsecBunker.error');
      console.error('Failed to parse bunker URL:', err);
   }
}

// Start the login process
async function startNsecBunkerLogin(bunkerPubkey?: string) {
   isLoading.value = true;
   error.value = null;
   
   try {
      await nostrStore.loginWithNsecBunker(secret.value, bunkerPubkey);
   } catch (err) {
      error.value = err instanceof Error ? err.message : t('introduction.nsecBunker.error');
      console.error('NSec Bunker login failed:', err);
   } finally {
      isLoading.value = false;
   }
}

// Start client-initiated login
async function startClientLogin() {
   await startNsecBunkerLogin();
}

function resetState() {
   error.value = null;
   bunkerUrl.value = '';
   isLoading.value = false;
}

function handleClose() {
   resetState();
   emit('close');
}

function copyToClipboard(text: string) {
   navigator.clipboard.writeText(text);
}

// Start client-initiated login when drawer opens
watch(() => props.show, (newShow) => {
   if (newShow) {
      startClientLogin();
   }
});
</script> 