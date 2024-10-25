<template>
   <FlowbiteDrawer :is-open="showDrawer" class="p-4" @close="handleCloseDrawer">
      <template #title>
         <h3 class="text-xl font-semibold text-gray-900">
            Back up your recovery phrase
         </h3>
         <p class="mt-2 text-sm font-normal text-pirate-800">
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
const showDrawer = ref(true);
const recoveryPhrase = ref([]);

const emit = defineEmits(["close", "next"]);

const props = defineProps({
   show: {
      type: Boolean,
      default: false,
   },
});

// Recovery phrase word list (you can expand this with more words)
const wordList = [
   "rain",
   "utility",
   "nerve",
   "flock",
   "useless",
   "comic",
   "sphere",
   "resemble",
   "science",
   "key",
   "amateur",
   "salad",
   "apple",
   "banana",
   "cloud",
   "digital",
   "energy",
   "flow",
   "gravity",
   "horizon",
   "ice",
   "jungle",
   "karma",
   "lunar",
];

// Generate a random recovery phrase (12 words)
const generateRecoveryPhrase = () => {
   const selectedWords = [];
   const totalWords = 12;

   // Randomly pick 12 unique words from the wordList
   while (selectedWords.length < totalWords) {
      const randomIndex = Math.floor(Math.random() * wordList.length);
      const word = wordList[randomIndex];

      // Ensure no duplicates
      if (!selectedWords.includes(word)) {
         selectedWords.push(word);
      }
   }

   return selectedWords;
};

watchEffect(() => {
   showDrawer.value = props.show;
   recoveryPhrase.value = generateRecoveryPhrase();
});

const handleCloseDrawer = () => {
   emit("close");
};

const handleNext = () => {
   emit("next");
};
</script>
