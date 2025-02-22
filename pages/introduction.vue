<template>
   <section
      class="relative isolate flex h-full flex-col items-center justify-between px-12 py-20"
   >
      <!-- Overlays -->
      <IntroductionLoginModal
         :show="showLoginModal"
         @close="handleCloseModal"
         @open-nsec-drawer="openNsecDrawer"
         @open-phrase-drawer="openPhraseDrawer"
      />
      <FlowbiteNostrModal v-if="showRegisterModal" />
      <IntroductionNsecDrawer
         :show-nsec-drawer="showNsecDrawer"
         @close="handleCloseDrawer"
      />
      <IntroductionPhraseDrawer
         :show-phrase-drawer="showPhraseDrawer"
         @close="handleCloseDrawer"
      />

      <div class="flex h-full flex-col items-center justify-between">
         <!-- Main page layout -->
         <NuxtImg
            src="/images/logos/resin-text.png"
            alt="RESIN Logo"
            class="h-10"
         />
         <div class="flex flex-col items-center justify-center gap-6">
            <div>
               <h1
                  class="text-center text-4xl font-bold leading-tight text-white"
               >
                  {{ $t("rentToOwn") }}
               </h1>
               <h1 class="text-center text-4xl font-bold text-white">
                  {{ $t("withoutALoan") }}
               </h1>
            </div>

            <FlowbiteButton
               v-if="authenticationStatus"
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
               v-if="!authenticationStatus"
               :text="$t('signIn')"
               class="border border-pirate-400 bg-transparent px-3 py-3 font-normal text-pirate-50"
               :show-icon="false"
               @click="openLoginModal"
            />
            <NuxtLinkLocale
               to="#"
               class="mb-2 me-2 mt-10 rounded-lg px-5 py-2.5 text-sm font-medium text-pirate-400 hover:bg-white hover:text-pirate-700"
            ></NuxtLinkLocale>
         </div>
      </div>
   </section>
</template>

<script setup>
const { checkAuthenticated, isAuthenticated } = useNostr();
const localePath = useLocalePath();

const authenticationStatus = ref(false);

authenticationStatus.value = await checkAuthenticated();

const { t } = useI18n();

useHead({
   title: t("introduction.title"),
});

definePageMeta({
   layout: "intro",
});

const showLoginModal = ref(false);
const showRegisterModal = ref(false);
const showNsecDrawer = ref(false);
const showPhraseDrawer = ref(false);

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
      navigateTo(localePath("properties"));
   }
};
</script>
