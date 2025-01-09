<template>
  <section class="flex max-h-full flex-col justify-between">
    <!-- Header -->
    <div class="mb-5 flex w-full flex-col gap-5 border-b border-gray-200 bg-white px-10 pb-5 pt-10">
      <NuxtLinkLocale @click="goBack">
        <PhCaretLeft :size="24" class="text-pirate-300" />
      </NuxtLinkLocale>
      <h1 class="text-2xl font-extrabold leading-tight text-pirate-950">
        {{ $t('settings.nostrKeys.title') }}
      </h1>
    </div>

    <!-- Keys Content -->
    <div class="flex-1 px-10">
      <div class="flex flex-col gap-6">
        <!-- Npub -->
        <div class="flex flex-col gap-2">
          <label class="text-sm font-medium text-pirate-950">{{ $t('settings.nostrKeys.npub') }}</label>
          <div class="flex items-center gap-2 rounded-lg border border-gray-300 p-2.5">
            <span class="flex-1 font-mono text-sm text-pirate-950 text-ellipsis overflow-hidden">{{ npub }}</span>
            <button 
              @click="copyToClipboard(npub)"
              class="text-resin-500 hover:text-resin-600"
            >
              <PhCopy :size="20" />
            </button>
          </div>
        </div>

        <!-- Public Key -->
        <div class="flex flex-col gap-2">
          <label class="text-sm font-medium text-pirate-950">{{ $t('settings.nostrKeys.publicKey') }}</label>
          <div class="flex items-center gap-2 rounded-lg border border-gray-300 p-2.5">
            <span class="flex-1 font-mono text-sm text-pirate-950 text-ellipsis overflow-hidden">{{ nostrStore.pubkey }}</span>
            <button 
              @click="copyToClipboard(nostrStore.pubkey)"
              class="text-resin-500 hover:text-resin-600"
            >
              <PhCopy :size="20" />
            </button>
          </div>
        </div>

        <!-- Recovery Phrase -->
        <div class="flex flex-col gap-2">
          <div class="flex items-center justify-between">
            <label class="text-sm font-medium text-pirate-950" v-if="nostrStore.getTypeKey === 'mnemonic'">{{ $t('settings.nostrKeys.recoveryPhrase') }}</label>
            <label class="text-sm font-medium text-pirate-950" v-else>{{ $t('settings.nostrKeys.privateKey') }}</label>

            <button 
              v-if="nostrStore.getTypeKey === 'mnemonic'"
              @click="showPhrase = !showPhrase"
              class="text-sm text-resin-500 hover:text-resin-600"
            >
              {{ showPhrase ? $t('settings.nostrKeys.hide') : $t('settings.nostrKeys.show') }}
            </button>
          </div>
          
          <template v-if="nostrStore.getTypeKey === 'mnemonic'">
            <div v-if="showPhrase" class="grid grid-cols-2 gap-2 rounded-lg border border-gray-300 p-4">
              <div 
                v-for="(word, index) in mnemonic" 
                :key="index"
                class="flex items-center gap-2"
              >
                <span class="text-xs text-pirate-300">{{ index + 1 }}</span>
                <span class="font-mono text-sm text-pirate-950">{{ word }}</span>
              </div>
            </div>
            <div v-else class="grid grid-cols-2 gap-2 rounded-lg border border-gray-300 p-4">
              <div 
                v-for="index in 12" 
                :key="index"
                class="flex items-center gap-2"
              >
                <span class="text-xs text-pirate-300">{{ index }}</span>
                <span class="font-mono text-sm text-pirate-300">●●●●●●</span>
              </div>
            </div>
          </template>
          
          <div v-else class="rounded-lg border border-gray-300 p-4">
            <p class="text-sm text-pirate-600">
                {{ $t('settings.nostrKeys.privateKeyNotAvailable') }}
            </p>
          </div>
        </div>
      </div>
    </div>

    <!-- Alert -->
    <ResinAlert :show="showCopiedAlert" :text="$t('settings.nostrKeys.copied')" />
  </section>
</template>

<script setup>
import { useNostrStore } from '~/stores/nostr';
import { PhCaretLeft, PhCopy } from "@phosphor-icons/vue";
import { nip19 } from 'nostr-tools';

const nostrStore = useNostrStore();
const showPhrase = ref(false);
const showCopiedAlert = ref(false);

const mnemonic = computed(() => {
  return nostrStore.mnemonic ? nostrStore.mnemonic.split(' ') : [];
});

const npub = computed(() => {
  return nostrStore.pubkey ? nip19.npubEncode(nostrStore.pubkey) : '';
});

const copyToClipboard = async (text) => {
  try {
    await navigator.clipboard.writeText(text);
    showCopiedAlert.value = true;
    setTimeout(() => {
      showCopiedAlert.value = false;
    }, 2000);
  } catch (err) {
    console.error('Failed to copy text: ', err);
  }
};

const goBack = () => {
   if (window.history.length > 1) {
      window.history.back();
   } else {
      // Fallback: navigate to home if there is no history
      const { localePath } = useNuxtApp();
      if (typeof localePath === 'function') {
        const path = localePath('home');
        if (typeof path === 'string') {
          window.location.href = path;
        }
      }
   }
};

definePageMeta({
  layout: "white",
  middleware: ['auth'],
});
</script> 