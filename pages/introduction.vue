<template>
   <section class="flex h-full flex-col items-center justify-between px-12 py-20">
      <!-- Main page layout -->
      <div class="flex h-full flex-col items-center justify-between">
         <NuxtImg src="/images/logos/resin-text.png" alt="Logo" class="h-10" />
         <div class="flex flex-col items-center justify-center gap-6">
            <div>
               <h1 class="text-center text-4xl font-bold leading-tight text-white">
                  {{ $t("rentToOwn") }}
               </h1>
               <h1 class="text-center text-4xl font-bold text-white">
                  {{ $t("withoutALoan") }}
               </h1>
            </div>

            <FlowbiteButton
               v-if="isAuthenticated" 
               class="px-5 py-3" 
               :text="$t('continueButton')"
               :show-icon="false" 
               @click="skipRegistration" 
            />

            <FlowbiteButton
               v-else
               class="px-5 py-3"
               :text="$t('introductionButton')"
               :show-icon="false"
               @click="openRegisterModal"
            />
            <NuxtImg
               src="/icons/arrow.png"
               alt="Arrow"
               class="absolute w-16 translate-x-36 translate-y-7"
            />
         </div>

         <div class="flex flex-col items-center justify-center gap-2">
            <FlowbiteButton
               v-if="!isAuthenticated"
               :text="$t('signIn')"
               class="border border-pirate-400 bg-transparent px-3 py-3 font-normal text-pirate-50"
               :show-icon="false"
               @click="openLoginModal"
            />
         </div>
      </div>

      <!-- Overlays -->
      <IntroductionLoginModal
         :show="showLoginModal"
         @open-nsec-drawer="openNsecDrawer"
         @open-phrase-drawer="openPhraseDrawer"
         @close="handleCloseModal"
      />

      <IntroductionPhraseDrawer
         :show="showPhraseDrawer"
         @close="handleCloseDrawer"
      />

      <IntroductionNsecBunkerDrawer
         :show="showNsecDrawer"
         @close="handleCloseDrawer"
      />

      <RegisterModal
         :show="showRegisterModal"
         @close="handleCloseModal"
      />
   </section>
</template>

<script setup lang="ts">
import { useNostr } from '~/composables/useNostr';

const { isAuthenticated } = useNostr();
const localePath = useLocalePath();
const { t } = useI18n();

useHead({
   title: t('introduction.title'),
});

definePageMeta({
   layout: "intro",
});

const showLoginModal = ref(false);
const showPhraseDrawer = ref(false);
const showNsecDrawer = ref(false);
const showRegisterModal = ref(false);

const openLoginModal = () => {
   showLoginModal.value = true;
};

const openNsecDrawer = () => {
   showNsecDrawer.value = true;
};

const openPhraseDrawer = () => {
   showPhraseDrawer.value = true;
};

const openRegisterModal = () => {
   showRegisterModal.value = true;
};

const handleCloseDrawer = () => {
   showNsecDrawer.value = false;
   showPhraseDrawer.value = false;
   showLoginModal.value = true;
};

const handleCloseModal = () => {
   showLoginModal.value = false;
   showRegisterModal.value = false;
};

const skipRegistration = () => {
   if (isAuthenticated) {
      navigateTo(localePath('properties'));
   }
}
</script>
