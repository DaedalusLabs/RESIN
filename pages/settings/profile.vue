<template>
  <section class="flex max-h-full flex-col justify-between">
    <!-- Header -->
    <div class="mb-5 flex w-full flex-col gap-5 border-b border-gray-200 bg-white px-10 pb-5 pt-10">
      <NuxtLinkLocale @click="goBack">
        <PhCaretLeft :size="24" class="text-pirate-300" />
      </NuxtLinkLocale>
      <h1 class="text-2xl font-extrabold leading-tight text-pirate-950">
        {{ $t('settings.profile.title') }}
      </h1>
    </div>

    <!-- Profile Content -->
    <div class="flex-1 px-10">
      <div class="flex flex-col gap-8">
        <!-- Profile Picture -->
        <div class="flex flex-col items-center gap-4">
          <div class="relative h-24 w-24">
            <img 
              :src="profilePicture || '/images/logos/Resin_Hexagon_Orange_Fill.svg'" 
              class="h-full w-full rounded-full object-cover"
              :alt="$t('settings.profile.picture.alt')"
            />
            <template v-if="enableProfileUpload">
            <button 
              @click="triggerFileInput"
              class="absolute bottom-0 right-0 rounded-full bg-resin-500 p-2 text-white hover:bg-resin-600"
              :title="$t('settings.profile.picture.upload')"
            >
              <PhCamera :size="20" />
            </button>
            <input 
              ref="fileInput"
              type="file"
              accept="image/*"
              class="hidden"
              @change="handleFileChange"
            />
            </template>
          </div>
        </div>

        <!-- Personal Information -->
        <div class="flex flex-col gap-6">
          <h2 class="text-lg font-semibold text-pirate-950">{{ $t('settings.profile.personalInfo') }}</h2>
          
          <!-- Name -->
          <div class="flex flex-col gap-2">
            <label class="text-sm font-medium text-pirate-950">{{ $t('settings.profile.displayName.label') }}</label>
            <input 
              v-model="profile.name"
              type="text"
              class="w-full rounded-lg border border-gray-300 p-2.5 text-pirate-950 focus:border-resin-500 focus:ring-resin-500"
              :placeholder="$t('settings.profile.displayName.placeholder')"
            />
          </div>

          <!-- Description -->
          <div class="flex flex-col gap-2">
            <label class="text-sm font-medium text-pirate-950">{{ $t('settings.profile.about.label') }}</label>
            <textarea 
              v-model="profile.about"
              rows="2"
              class="w-full rounded-lg border border-gray-300 p-2.5 text-pirate-950 focus:border-resin-500 focus:ring-resin-500"
              :placeholder="$t('settings.profile.about.placeholder')"
            ></textarea>
          </div>

          <!-- Save Button -->
          <button 
            @click="saveProfile"
            class="mt-4 w-full rounded-lg bg-resin-500 px-4 py-2.5 text-center text-sm font-medium text-white hover:bg-resin-600"
          >
            {{ $t('settings.profile.saveChanges') }}
          </button>
        </div>
      </div>
    </div>

    <!-- Alert -->
    <ResinAlert :show="showSavedAlert" :text="$t('settings.profile.saved')" />
  </section>
</template>

<script setup lang="ts">
import { useNostrStore } from '~/stores/nostr';
import { PhCaretLeft, PhCamera } from "@phosphor-icons/vue";
import { useNDK } from '~/composables/useNDK';
import type { NDKUserProfile } from '@nostr-dev-kit/ndk';

const nostrStore = useNostrStore();
const ndk = useNDK();
const showSavedAlert = ref(false);
const fileInput = ref<HTMLInputElement | null>(null);

const profile = ref<NDKUserProfile>({
  name: '',
  about: '',
  picture: undefined
});

const enableProfileUpload = ref(false);

const profilePicture = computed(() => profile.value?.picture);

onMounted(async () => {
  if (nostrStore.user) {
    const metadata = await nostrStore.user.profile;
    if (metadata) {  
      profile.value = metadata;
    }
  }
});

const triggerFileInput = () => {
  if (fileInput.value) {
    fileInput.value.click();
  }
};

const handleFileChange = async (event: Event) => {
  const input = event.target as HTMLInputElement;
  const file = input.files?.[0];
  if (file) {
    try {
      // Convert to base64
      const reader = new FileReader();
      reader.onload = (e) => {
        const result = e.target?.result;
        if (typeof result === 'string') {
          profile.value.picture = result;
        }
      };
      reader.readAsDataURL(file);
    } catch (err) {
      console.error('Failed to upload image:', err);
    }
  }
};

const saveProfile = async () => {
  try {
    if (!ndk || !nostrStore.user) return;

    const event = {
      kind: 0,
      content: JSON.stringify({
        name: profile.value.name,
        about: profile.value.about,
      })
    };

    nostrStore.user.profile = profile.value;

    await nostrStore.user.publish(event);
    showSavedAlert.value = true;
    setTimeout(() => {
      showSavedAlert.value = false;
    }, 2000);
  } catch (err) {
    console.error('Failed to save profile:', err);
  }
};

const goBack = () => {
  if (window.history.length > 1) {
    window.history.back();
  } else {
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