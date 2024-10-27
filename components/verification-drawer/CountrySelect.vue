<template>
   <FlowbiteDrawer :is-open="showDrawer" class="p-4" @close="handleCloseDrawer">
      <template #title>
         <h3 class="text-lg font-bold text-pirate-950">
            Select the type of issuing country of your identity document
         </h3>
      </template>
      <template #content>
         <div class="mx-auto flex flex-col gap-5">
            <!-- Country Selector -->
            <div>
               <label class="mb-1 block font-bold text-pirate-950"
                  >Issuing country</label
               >
               <div class="relative">
                  <button
                     class="flex w-full items-center justify-between rounded-md border border-gray-300 bg-gray-100 p-2.5 text-left focus:border-resin-500 focus:ring-resin-500"
                     @click="toggleDropdown"
                  >
                     <p v-if="selectedCountry" class="text-sm">
                        <span class="flag-emoji">{{
                           getFlagEmoji(selectedCountry.cca2)
                        }}</span>
                        {{ selectedCountry.name.common }}
                     </p>
                     <span v-else class="text-sm"> Select a country </span>
                     <PhCaretDown :size="20" />
                  </button>
                  <div
                     v-if="isOpen"
                     class="absolute z-10 mt-1 max-h-60 w-full overflow-auto rounded-md border border-gray-300 bg-white shadow-lg"
                  >
                     <div
                        v-for="country in countries"
                        :key="country.cca2"
                        class="flex cursor-pointer items-center p-2 hover:bg-gray-100"
                        @click="selectCountry(country)"
                     >
                        <span class="flag-emoji mr-2">
                           {{ getFlagEmoji(country.cca2) }}
                        </span>
                        {{ country.name.common }}
                     </div>
                  </div>
               </div>
            </div>

            <!-- Document Type Selector -->
            <div>
               <label class="mb-1 block font-bold text-pirate-950"
                  >Document Type</label
               >
               <div class="space-y-2">
                  <div
                     v-for="option in documentTypeOptions"
                     :key="option.value"
                     class="flex cursor-pointer items-center justify-between rounded-lg border border-gray-300 p-3 hover:bg-gray-50"
                     :class="{
                        'border-pirate-950':
                           selectedDocumentType === option.value,
                     }"
                  >
                     <label
                        :for="option.value"
                        class="flex-grow cursor-pointer text-sm"
                     >
                        {{ option.label }}
                     </label>
                     <input
                        :id="option.value"
                        v-model="selectedDocumentType"
                        type="radio"
                        :value="option.value"
                        class="h-4 w-4 border-gray-300 text-resin-600 focus:ring-resin-500"
                     />
                  </div>
               </div>
            </div>
         </div>

         <!-- Navigation Buttons -->
         <div class="mt-6 flex w-full justify-center gap-4">
            <FlowbiteBorderButton
               text="Back"
               :show-icon="false"
               @click="handleBack"
            >
               <template #icon-prepend>
                  <PhArrowLeft :size="24" weight="bold" />
               </template>
            </FlowbiteBorderButton>
            <FlowbiteButton
               text="Next"
               class="px-5 py-3"
               :disabled="!isFormValid"
               :class="{
                  'cursor-not-allowed bg-gray-100 opacity-50 hover:bg-gray-100':
                     !isFormValid,
               }"
               @click="handleNext"
            />
         </div>
      </template>
   </FlowbiteDrawer>
</template>

<script setup>
import { PhArrowLeft, PhCaretDown } from "@phosphor-icons/vue";

const showDrawer = ref(true);
const emit = defineEmits(["close", "next", "back"]);
const documentTypeOptions = ref([
   { label: "ID card", value: "ID card" },
   { label: "Residence permit", value: "Residence permit" },
   { label: "Passport", value: "Passport" },
   { label: "Driving license", value: "Driving license" },
]);
const selectedDocumentType = ref(null);
const selectedCountry = ref(null);
const isOpen = ref(false);
const countries = ref([]);
const props = defineProps({
   show: {
      type: Boolean,
      default: false,
   },
});

const getFlagEmoji = (countryCode) => {
   const flag = countryCode
      .toUpperCase()
      .replace(/./g, (char) =>
         String.fromCodePoint(127397 + char.charCodeAt()),
      );
   return flag;
};

const isFormValid = computed(() => {
   return selectedCountry.value && selectedDocumentType.value;
});

const handleNext = () => {
   if (isFormValid.value) {
      emit("next");
   }
};

onMounted(async () => {
   try {
      const response = await fetch(
         "https://restcountries.com/v3.1/all?fields=name,cca2,flag",
      );
      const data = await response.json();
      countries.value = data.sort((a, b) =>
         a.name.common.localeCompare(b.name.common),
      );
   } catch (error) {
      console.error("Error fetching countries:", error);
   }
});

const toggleDropdown = (event) => {
   event.stopPropagation();
   isOpen.value = !isOpen.value;
};

const selectCountry = (country) => {
   selectedCountry.value = country;
   isOpen.value = false;
};

watchEffect(() => {
   showDrawer.value = props.show;
});

const handleCloseDrawer = () => {
   emit("close");
};

const handleBack = () => {
   emit("back");
};
</script>

<style>
.flag-emoji {
   font-family: "Noto Color Emoji", "Segoe UI Emoji", "Apple Color Emoji",
      sans-serif;
}
</style>
