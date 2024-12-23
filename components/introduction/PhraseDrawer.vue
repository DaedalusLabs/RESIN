<template>
   <div>
      <FlowbiteErrorToast
         :text="`Invalid Nsec key`"
         :show-toast="wrongPhrase"
         @close-toast="wrongPhrase = false"
      />
      <FlowbiteDrawer :is-open="showDrawer" @close="handleCloseDrawer">
         <template #title> Log in with recovery phrase </template>
         <template #content>
            <form action="#" method="POST" @submit.prevent="validatePhrase">
               <div class="grid grid-cols-2 gap-4">
                  <FlowbiteTextInput
                     v-for="(word, index) in recoveryPhrase"
                     :key="index"
                     v-model="recoveryPhrase[index]"
                     :placeholder="'Word ' + (index + 1)"
                     :error-messages="wordErrors[index]"
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
import { wordlist } from '@scure/bip39/wordlists/english';
const { loginWithMnemonic, isAuthenticated } = useNostr();

const recoveryPhrase = ref(Array(12).fill(""));
const wordErrors = ref(Array(12).fill([]));

const errorMessages = ref([]);
const wrongPhrase = ref(false);

const validatePhrase = async() => {
   if (recoveryPhrase.value.filter((e) => e == "").length || wordErrors.value.find((e) => e == "error")) {
      errorMessages.value.push("Please fill in your recovery phrase");
   } else {
      await loginWithMnemonic(recoveryPhrase.value.join(' '));

      if (isAuthenticated) {
         const localeRoute = useLocaleRoute();
         const route = localeRoute({
            name: "properties",
         });
         if (route) {
            return navigateTo(route.fullPath);
         }
      }
      errorMessages.value.push("Incorrect recovery phrase");

   }
};

const handlePaste = (event) => {
   event.preventDefault();
   const pastedText = event.clipboardData.getData('text/plain');
   const words = pastedText.trim().split(/\s+/);

   if (words.length <= 12) {
      words.forEach((word, index) => {
         if (index < 12) {
            recoveryPhrase.value[index] = word;
         }
      });
   }
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
         if (recoveryPhrase.value[w].length && !wordlist.includes(recoveryPhrase.value[w])) {
            wordErrors.value[w] = "error" ;
         } else {
            wordErrors.value[w] = "";
         }
      }
      //errorMessages.value.g
   }
});

const emit = defineEmits(["close"]);

const handleCloseDrawer = () => {
   showDrawer.value = false;
   emit("close");
};
</script>
