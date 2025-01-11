<template>
   <div>
      <FlowbiteErrorToast
         :text="$t('introduction.nsec.invalidKey')"
         :show-toast="wrongPhrase"
         @close-toast="wrongPhrase = false"
      />
      <FlowbiteDrawer :is-open="showDrawer" slide-from="bottom" @close="handleCloseDrawer">
         <template #title>
            {{ $t("introduction.login.recoveryPhrase.title") }}
         </template>
         <template #content>
            <form action="#" method="POST" @submit.prevent="validatePhrase" class="pb-safe">
               <div class="grid grid-cols-2 gap-4">
                  <FlowbiteTextInput
                     v-for="(word, index) in recoveryPhrase"
                     :key="index"
                     v-model="recoveryPhrase[index]"
                     :placeholder="
                        $t(
                           'introduction.login.recoveryPhrase.wordPlaceholder',
                           { number: index + 1 },
                        )
                     "
                     :error-messages="wordErrors[index]"
                     autocapitalize="none"
                     autocorrect="off"
                     autocomplete="off"
                     spellcheck="false"
                     @paste="handlePaste"
                  />
               </div>
               <p v-if="showError" class="mt-1 text-sm text-red-600">
                  {{ errorMessages[0] }}
               </p>
               <FlowbiteButton
                  :text="$t('logIn')"
                  class="mt-4 flex w-full items-center justify-center px-5 py-3"
                  :is-login="true"
                  :show-icon="false"
                  @click="validatePhrase"
               />
            </form>
         </template>
      </FlowbiteDrawer>
   </div>
</template>

<script setup>
import { wordlist } from "@scure/bip39/wordlists/english";
const { loginWithMnemonic, isAuthenticated, checkAuthenticated } = useNostr();
const { t } = useI18n();

const recoveryPhrase = ref(Array(12).fill(""));
const wordErrors = ref(Array(12).fill([]));

const errorMessages = ref([]);
const wrongPhrase = ref(false);

const validatePhrase = async () => {
   if (
      recoveryPhrase.value.filter((e) => e == "").length ||
      wordErrors.value.find((e) => e == "error")
   ) {
      errorMessages.value.push(t("introduction.login.recoveryPhrase.error"));
   } else {
      await loginWithMnemonic(recoveryPhrase.value.join(" "));
      await checkAuthenticated();

      if (isAuthenticated) {
         const localeRoute = useLocaleRoute();
         const route = localeRoute({
            name: "properties",
         });
         if (route) {
            return navigateTo(route.fullPath);
         }
      }
      errorMessages.value.push(
         t("introduction.login.recoveryPhrase.invalidPhrase"),
      );
   }
};

const handlePaste = (event) => {
   const pastedText = event.clipboardData.getData("text/plain");
   const words = pastedText.trim().split(/\s+/);

   // If pasting multiple words, handle it specially
   if (words.length > 1) {
      event.preventDefault();
      if (words.length <= 12) {
         words.forEach((word, index) => {
            if (index < 12) {
               recoveryPhrase.value[index] = word;
            }
         });
      }
   }
   // For single word paste, let the default behavior handle it
};

const props = defineProps({
   showPhraseDrawer: Boolean,
});

const showDrawer = ref(false);

watchEffect(() => {
   showDrawer.value = props.showPhraseDrawer;
});

watchEffect(() => {
   if (recoveryPhrase.value.some((word) => word.trim() !== "")) {
      for (const w in recoveryPhrase.value) {
         if (
            recoveryPhrase.value[w].length &&
            !wordlist.includes(recoveryPhrase.value[w])
         ) {
            wordErrors.value[w] = "error";
         } else {
            wordErrors.value[w] = "";
         }
      }
   }
});

const emit = defineEmits(["close"]);

const handleCloseDrawer = () => {
   showDrawer.value = false;
   emit("close");
};
</script>

<style scoped>
.pb-safe {
   padding-bottom: env(safe-area-inset-bottom);
}
</style>
