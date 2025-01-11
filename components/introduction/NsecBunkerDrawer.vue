<template>
  <FlowbiteDrawer :is-open="showDrawer" @close="handleCloseDrawer">
    <template #title>{{ $t('introduction.nsecBunker.title') }}</template>
    <template #content>
      <div class="flex flex-col items-center gap-6 p-4">
        <!-- QR Code Section -->
        <div class="flex flex-col items-center">
          <div class="mb-4">
            <QRCode
              :data="nostrConnectUri"
              :width="200"
              :height="200"
            />
          </div>
          <div class="flex w-full items-center gap-2 rounded-lg border border-gray-300 p-2.5">
            <span class="flex-1 font-mono text-sm text-pirate-950 text-ellipsis overflow-hidden">{{ nostrConnectUri }}</span>
            <button 
              @click="copyNostrConnectUri"
              class="text-resin-500 hover:text-resin-600"
            >
              <PhCopy :size="20" />
            </button>
          </div>
        </div>

        <!-- OR Divider -->
        <div class="relative w-full">
          <div class="absolute inset-0 flex items-center">
            <div class="w-full border-t border-gray-300"></div>
          </div>
          <div class="relative flex justify-center">
            <span class="bg-white px-2 text-sm text-gray-500">{{ $t('or') }}</span>
          </div>
        </div>

        <!-- Bunker Input Section -->
        <div class="w-1/2">
          <div class="flex flex-row gap-2 align-center justify-center">
            <FlowbiteTextInput
              v-model="bunkerUrl"
              placeholder="bunker://"
              class=""
            />
            <FlowbiteButton @click="handleConnect" :text="t('introduction.nsecBunker.connect')">
              {{ $t('introduction.nsecBunker.connect') }}
            </FlowbiteButton>
          </div>
        </div>
      </div>
    </template>
  </FlowbiteDrawer>

  <ResinAlert
    :show="showAlert"
    :text="alertText"
    :type="alertType"
  />
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { PhCopy } from "@phosphor-icons/vue";
import { useNostrStore } from '../../stores/nostr';

const { t } = useI18n();
const props = defineProps({
  show: {
    type: Boolean,
    default: false
  }
});

const nostrStore = useNostrStore();

const emit = defineEmits(['close']);
const showDrawer = ref(false);
const bunkerUrl = ref('');

// Alert state
const showAlert = ref(false);
const alertText = ref('');
const alertType = ref<'success' | 'info' | 'warning' | 'danger'>('success');

// This should be generated based on your nostr connect requirements
const nostrConnectUri = ref('');

onMounted(async () => {
  nostrConnectUri.value = await nostrStore.getNostrConnectUri();
  await nostrStore.waitForNip46Signer();
  });

watchEffect(() => {
  showDrawer.value = props.show;
});

function handleCloseDrawer() {
  showDrawer.value = false;
  emit('close');
}

function showTemporaryAlert(text: string, type: 'success' | 'info' | 'warning' | 'danger') {
  alertText.value = text;
  alertType.value = type;
  showAlert.value = true;
  setTimeout(() => {
    showAlert.value = false;
  }, 3000);
}

function copyNostrConnectUri() {
  navigator.clipboard.writeText(nostrConnectUri.value)
    .then(() => {
      showTemporaryAlert(t('introduction.nsecBunker.uriCopied'), 'success');
    })
    .catch(() => {
      showTemporaryAlert(t('introduction.nsecBunker.copyFailed'), 'danger');
    });
}

async function handleConnect() {
  if (!bunkerUrl.value) {
    showTemporaryAlert(t('introduction.nsecBunker.enterUrl'), 'warning');
    return;
  }
  await nostrStore.connectToBunker(bunkerUrl.value);

  await nostrStore.loginWithRemoteSigner();
  // Implement bunker connection logic here
  nostrStore.checkAuthenticated().then((authenticated) => {
    if (authenticated) {
      const localeRoute = useLocaleRoute();
      const route = localeRoute({
        name: "properties",
      });
      if (route) {
        return navigateTo(route.fullPath);
      }
    }
  });

  console.log('Connecting to bunker:', bunkerUrl.value);
}
</script> 