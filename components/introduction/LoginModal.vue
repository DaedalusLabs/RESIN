<template>
   <FlowbiteModal :is-open="isModalOpen" @update:is-open="handleModalUpdate">
      <template #title>Log in with NOSTR</template>
      <p class="my-6 text-sm">Choose an option below to access your account.</p>
      <div class="mt-4 flex w-full flex-col gap-4">
         <NuxtLink :to="localePath('properties')" class="block">
            <FlowbiteBorderButton
               :text="`Use browser extension`"
               class="w-full"
            />
         </NuxtLink>

         <FlowbiteBorderButton
            :text="`Use private key (NSEC)`"
            @click="emit('openNsecDrawer')"
         />
         <FlowbiteBorderButton
            :text="`Use recovery phrase (12 words)`"
            @click="emit('openPhraseDrawer')"
         />
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

<script setup>
const emit = defineEmits(["openNsecDrawer", "openPhraseDrawer", "close"]);

const isModalOpen = ref(false);

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
</script>
