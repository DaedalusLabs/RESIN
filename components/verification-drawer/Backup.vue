<template>
   <FlowbiteDrawer :is-open="showDrawer" @close="handleCloseDrawer">
      <template #title>
         <h3 class="text-xl font-semibold text-pirate-950">
            Back up your recovery phrase
         </h3>
         <p class="mt-2 text-xs font-normal text-pirate-800">
            These words let you recreate your account in case you lose access.
            Write them down in a safe place.
         </p>
      </template>

      <template #content>
         <!-- Recovery Phrase List -->
         <div class="mx-10 mt-4 grid grid-cols-2 gap-4 text-gray-800">
            <div
               v-for="(word, index) in recoveryPhrase"
               :key="index"
               class="flex items-center space-x-2 font-semibold"
            >
               <span class="text-pirate-300">{{ index + 1 }}</span>
               <span class="text-pirate-950">{{ word }}</span>
            </div>
         </div>

         <!-- Next Button -->
         <div class="mt-6 flex w-full justify-center">
            <FlowbiteButton
               text="I'm ready"
               class="px-5 py-3"
               :show-icon="false"
               @click="handleNext"
            />
         </div>
      </template>
   </FlowbiteDrawer>
</template>

<script setup>
import { usePropertiesStore } from "~/stores/properties";

import { useNostrStore } from "~/stores/nostr";
const propertiesStore = usePropertiesStore();
const nostrStore = useNostrStore();

const showDrawer = ref(true);
const recoveryPhrase = ref([]);

const emit = defineEmits(["close", "next"]);

const props = defineProps({
   show: {
      type: Boolean,
      default: false,
   },
});

// Generate a random recovery phrase (12 words)
const getRecoveryPhrase = () => {
   return nostrStore.mnemonic.split(" ");
};

watchEffect(() => {
   showDrawer.value = props.show;
   if (showDrawer.value) {
      recoveryPhrase.value = getRecoveryPhrase();
   }
});

const handleCloseDrawer = () => {
   emit("close");
};

const handleNext = () => {
   propertiesStore.recoveryPhrase = recoveryPhrase.value;
   emit("next");
};
</script>
