<template>
   <FlowbiteModal :is-open="isModalOpen" @update:is-open="handleModalUpdate">
      <template #title>Log in with NOSTR</template>
      <p class="my-6 text-sm">Choose an option below to access your account.</p>
      <div class="mt-4 flex w-full flex-col gap-4">
         <FlowbiteBorderButton :text="`Use recovery phrase (12 words)`" @click="emit('openPhraseDrawer')" />

         <FlowbiteBorderButton
:text="hasExtension() ? 'Use browser extension' : 'No extension found - more info'"
            class="w-full border-gray text-gray" 
            @click="hasExtension() ? handleExtensionLogin() : openExtensionApps()" />
      </div>
      <p class="mt-4 text-center text-xs">
         Or
         <span class="text-resin-500">create a new account</span>
         if you don't have one yet. Have questions?
         <NuxtLink class="text-resin-500" to="#">Contact us</NuxtLink>
         for assistance.
      </p>
   </FlowbiteModal>
</template>

<script setup lang="ts">
import { useNostr } from '~/composables/useNostr';

const { loginWithExtension, isAuthenticated, hasExtension } = useNostr();

const emit = defineEmits(["openNsecDrawer", "openPhraseDrawer", "close"]);

const isModalOpen = ref(false);
const localePath = useLocalePath()

const props = defineProps({
   show: Boolean,
});

watchEffect(() => {
   if (props.show) {
      isModalOpen.value = true;
   }
});

const handleModalUpdate = (value) => {
   isModalOpen.value = value;
   if (!value) {
      emit("close");
   }
};

async function handleExtensionLogin() {
   try {
      await loginWithExtension();

      if (isAuthenticated) {
         navigateTo(localePath('properties'));
      }
   } catch (err) {
      // Error is handled in composable
      console.error('Login failed:', err);
   }
}

function openExtensionApps() {
   navigateTo('https://nostrapps.com/#signers', {
      open: {
         target: '_blank'
      }
   });
}
</script>
