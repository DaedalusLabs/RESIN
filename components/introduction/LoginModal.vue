<template>
   <FlowbiteModal :is-open="isModalOpen" @update:is-open="handleModalUpdate">
      <template #title>{{ $t('introduction.login.title') }}</template>
      <p class="my-6 text-sm">{{ $t('introduction.login.description') }}</p>
      <div class="mt-4 flex w-full flex-col gap-4">
         <FlowbiteBorderButton :text="$t('introduction.login.recoveryPhrase.button')" @click="() => {
               emit('openPhraseDrawer');
               handleModalUpdate(false);
            }" />

         <section v-if="hasExtension()">
            <FlowbiteBorderButton v-if="hasNip44()"
               :text="hasExtension() ? $t('introduction.login.extension.useExtension') : $t('introduction.login.extension.noExtension')"
               class="w-full border-gray text-gray"
               @click="hasExtension() ? handleExtensionLogin() : openExtensionApps()" />
            <button v-else-if="!hasNip44()"
               class="w-full inline-flex items-center justify-center rounded-lg border border-pirate-500 bg-transparent px-4 py-2 text-center text-sm font-medium text-pirate-500"
               disabled>
               {{ $t('introduction.login.extension.noNip44') }}
            </button>
         </section>
         <section v-else>
            <FlowbiteBorderButton :text="$t('introduction.login.extension.noExtension')" class="w-full" color="gray-500" hoverTextColor="gray-500"  @click="openExtensionApps()" />
         </section>

         <FlowbiteBorderButton 
            :text="$t('introduction.login.nsecBunker.button')" 
            class="w-full"
            @click="handleNsecBunkerLogin" 
         />
      </div>
   </FlowbiteModal>
</template>

<script setup lang="ts">
import { useNostr } from '~/composables/useNostr';
import { usePropertiesStore } from '~/stores/properties';

const { loginWithExtension, isAuthenticated, hasExtension, hasNip44, checkAuthenticated } = useNostr();

const emit = defineEmits(["openNsecDrawer", "openPhraseDrawer", "close"]);
const localePath = useLocalePath();

const isModalOpen = ref(false);

const props = defineProps({
   show: Boolean,
});

watchEffect(() => {
   if (props.show) {
      isModalOpen.value = true;
   }
});

const handleModalUpdate = (value: boolean) => {
   isModalOpen.value = value;
   if (!value) {
      emit("close");
   }
};

async function handleExtensionLogin() {
   try {
      await loginWithExtension();
      await checkAuthenticated();
      await usePropertiesStore().init();
      
      if (isAuthenticated) {
         navigateTo(localePath('properties'));
      }
   } catch (err) {
      // Error is handled in composable
      console.error('Login failed:', err);
   }
}

function handleNsecBunkerLogin() {
   emit('openNsecDrawer');
   handleModalUpdate(false);
}

function openExtensionApps() {
   window.open('https://getalby.com/', '_blank');
}
</script>
