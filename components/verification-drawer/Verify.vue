<template>
   <FlowbiteDrawer :is-open="showDrawer" class="p-4" @close="handleCloseDrawer">
      <template #title>
         <h3 class="text-xl font-semibold text-gray-900">
            Verify your recovery phrase
         </h3>
         <p class="mt-2 text-sm font-normal text-pirate-800">
            Tap the words in the order they appear in your recovery phrase and
            click next.
         </p>
      </template>
      <template #content>
         <div class="mt-4 grid grid-cols-2 gap-4 text-gray-800">
            <button
               v-for="(word, index) in shuffledWords"
               :key="index"
               class="flex items-center rounded-md border p-2 px-3 transition-colors duration-200"
               :disabled="shouldDisableWord(word)"
               :class="{
                  'bg-green-500 text-white':
                     isWordSelected(word) && !isIncorrectWord(word),
                  'bg-red-500 text-white': isIncorrectWord(word),
                  'hover:bg-red-100': isIncorrectWord(word),
                  'cursor-not-allowed opacity-50':
                     shouldDisableWord(word) && !isIncorrectWord(word),
                  'cursor-pointer hover:bg-red-600': isIncorrectWord(word),
               }"
               @click="handleWordClick(word)"
            >
               <p
                  v-if="isWordSelected(word)"
                  class="flex w-full items-center justify-between font-semibold"
               >
                  <span>{{ getWordOrder(word) }}. {{ word }}</span>
                  <PhCheck
                     v-if="!isIncorrectWord(word)"
                     :size="20"
                     weight="bold"
                  />
                  <PhX v-else :size="20" weight="bold" />
               </p>
               <p v-else>
                  {{ word }}
               </p>
            </button>
         </div>
         <!-- Navigation Buttons -->
         <div class="mt-6 flex justify-center gap-4">
            <FlowbiteBorderButton
               text="Back"
               class="px-5 py-3"
               :show-icon="false"
               @click="handleBack"
            >
               <template #icon-prepend>
                  <PhArrowLeft :size="24" weight="bold" />
               </template>
            </FlowbiteBorderButton>
            <FlowbiteBorderButton
               text="Next"
               class="px-5 py-3 transition-all duration-200"
               :class="{
                  'cursor-not-allowed bg-gray-100 opacity-50':
                     !isCompleteAndCorrect,
                  'hover:bg-gray-100': !isCompleteAndCorrect,
               }"
               :disabled="!isCompleteAndCorrect"
               @click="handleNext"
            >
               <template #icon-append>
                  <PhArrowRight :size="24" weight="bold" />
               </template>
            </FlowbiteBorderButton>
         </div>
      </template>
   </FlowbiteDrawer>
</template>

<script setup>
import { PhArrowLeft, PhArrowRight, PhCheck, PhX } from "@phosphor-icons/vue";
import { usePropertiesStore } from "~/stores/properties";
import { ref, watchEffect, computed } from "vue";

const propertiesStore = usePropertiesStore();
const showDrawer = ref(true);
const shuffledWords = ref([]);
const emit = defineEmits(["close", "next", "back"]);
const recoveryPhrase = ref(propertiesStore.recoveryPhrase || []);
const selectedWords = ref([]);
const incorrectWords = ref(new Set());

const props = defineProps({
   show: {
      type: Boolean,
      default: false,
   },
});

const isCompleteAndCorrect = computed(() => {
   return (
      selectedWords.value.length === recoveryPhrase.value.length &&
      incorrectWords.value.size === 0 &&
      selectedWords.value.every(
         (word, index) => word === recoveryPhrase.value[index],
      )
   );
});

const hasIncorrectSelection = computed(() => {
   return incorrectWords.value.size > 0;
});

watchEffect(() => {
   showDrawer.value = props.show;
   if (propertiesStore.recoveryPhrase) {
      recoveryPhrase.value = propertiesStore.recoveryPhrase;
      shuffledWords.value = [...recoveryPhrase.value].sort(
         () => Math.random() - 0.5,
      );
      // Reset selections when drawer is opened
      selectedWords.value = [];
      incorrectWords.value.clear();
   }
});

const getWordOrder = (word) => {
   return selectedWords.value.indexOf(word) + 1;
};

const isIncorrectWord = (word) => {
   return incorrectWords.value.has(word);
};

const shouldDisableWord = (word) => {
   return (
      (isWordSelected(word) && !isIncorrectWord(word)) ||
      (hasIncorrectSelection.value && !isIncorrectWord(word))
   );
};

const handleWordClick = (word) => {
   if (isIncorrectWord(word)) {
      // Unselect incorrect word
      selectedWords.value = selectedWords.value.filter((w) => w !== word);
      incorrectWords.value.delete(word);
      return;
   }

   if (isWordSelected(word) || hasIncorrectSelection.value) {
      return;
   }

   // Check if the word is the next correct word in the sequence
   const nextIndex = selectedWords.value.length;
   if (word === recoveryPhrase.value[nextIndex]) {
      selectedWords.value.push(word);
   } else {
      // Mark as incorrect if wrong order
      selectedWords.value.push(word);
      incorrectWords.value.add(word);
   }
};

const isWordSelected = (word) => {
   return selectedWords.value.includes(word);
};

const handleCloseDrawer = () => {
   emit("close");
};

const handleNext = () => {
   if (isCompleteAndCorrect.value) {
      emit("next");
   }
};

const handleBack = () => {
   emit("back");
};
</script>
