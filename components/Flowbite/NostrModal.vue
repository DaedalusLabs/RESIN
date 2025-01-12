<template>
   <!-- Background overlay (dimming effect) -->
   <div class="fixed inset-0 z-40 bg-black bg-opacity-50" />

   <!-- Main modal with smooth expand animation -->
   <div
      class="fixed inset-0 z-50 flex items-center justify-center transition-all ease-in-out"
   >
      <div
         class="w-10/12 max-w-lg rounded-lg bg-white p-8 text-center shadow-lg transition-all duration-500 ease-in-out"
         :class="{
            'max-h-[100vh] overflow-visible': !loading,
            'max-h-[200px] overflow-hidden': loading,
         }"
      >
         <!-- Dynamic content based on loading state -->
         <section class="relative mx-auto w-24">
            <NuxtImg
               :src="
                  loading
                     ? '/images/running-nostrich.webp'
                     : '/images/still-nostrich.webp'
               "
               class="mx-auto w-24"
               :alt="loading ? 'Loading nostrich' : 'Nostrich'"
            />
            <PhCheckCircle
               v-if="!loading"
               class="absolute -right-6 -top-2 w-8"
               weight="bold"
               size="36"
               color="#9C27B0"
            />
         </section>
         <p class="mt-4 font-extrabold text-gray-900">
            {{ loading ? $t("nostr.creating") : $t("nostr.created") }}
         </p>

         <!-- Wait for modal expansion before showing text and button -->
         <transition name="fade-in" mode="out-in" appear>
            <div v-if="!loading" key="success-message" class="mt-4">
               <p class="text-sm text-gray-500">
                  {{ $t("nostr.addDetails") }}
               </p>

               <!-- Button for success state, only visible after loading -->
               <NuxtLinkLocale to="choose-property-type">
                  <FlowbiteButton
                     :show-icon="false"
                     :text="$t('nostr.continue')"
                     class="mt-4 text-center"
                  />
               </NuxtLinkLocale>
            </div>
         </transition>
      </div>
   </div>
</template>

<script setup>
import { PhCheckCircle } from "@phosphor-icons/vue";

const loading = ref(true);
const { generateKeyPair, isAuthenticated } = useNostr();

onMounted(async () => {
   setTimeout(() => {
      loading.value = false;
   }, 2000);
   await generateKeyPair();

   const timeout = setTimeout(() => {
      if (isAuthenticated) {
         loading.value = false;
         clearTimeout(timeout);
      }
   }, 500);
});
</script>

<style scoped>
/* Fade-in effect for text and button after modal expansion */
.fade-in-enter-active,
.fade-in-leave-active {
   transition: opacity 0.5s ease-in-out;
}
.fade-in-enter-from,
.fade-in-leave-to {
   opacity: 0;
}
.fade-in-enter-to,
.fade-in-leave-from {
   opacity: 1;
}
</style>
