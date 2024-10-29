<template>
   <FlowbiteDrawer :is-open="showDrawer" class="p-4" @close="handleCloseDrawer">
      <template #title>
         <h3
            class="flex items-center gap-2 text-xl font-semibold text-pirate-950"
         >
            Your information
         </h3>
         <p>For the Rent-To-Own agreement</p>
      </template>
      <template #content>
         <form @submit.prevent="handleNext">
            <div class="max-h-[300px] space-y-4 overflow-scroll">
               <input
                  v-model="firstName"
                  type="text"
                  placeholder="First name"
                  class="form-input"
                  required
               />
               <input
                  v-model="lastName"
                  type="text"
                  placeholder="Last name"
                  class="form-input"
                  required
               />
               <input
                  v-model="email"
                  type="email"
                  placeholder="Email"
                  class="form-input"
                  :class="{ 'invalid-input': noValidEmail }"
                  required
                  @blur="validateEmail"
               />
               <input
                  v-model="address"
                  type="text"
                  placeholder="Address"
                  class="form-input"
                  required
               />
               <input
                  v-model="city"
                  type="text"
                  placeholder="City"
                  class="form-input"
                  required
               />
               <input
                  v-model="state"
                  type="text"
                  placeholder="State"
                  class="form-input"
                  required
               />
               <input
                  v-model="zipCode"
                  type="text"
                  placeholder="Zip Code"
                  class="form-input"
                  required
               />
               <input
                  v-model="country"
                  type="text"
                  placeholder="Country"
                  class="form-input"
                  required
               />
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
                     'cursor-not-allowed bg-gray-100 opacity-50 hover:bg-gray-100':
                        !isCompleteAndCorrect || noValidEmail,
                  }"
                  :disabled="!isCompleteAndCorrect || noValidEmail"
                  @click="handleNext"
               >
                  <template #icon-append>
                     <PhArrowRight :size="24" weight="bold" />
                  </template>
               </FlowbiteBorderButton>
            </div>
         </form>
      </template>
   </FlowbiteDrawer>
</template>

<script setup>
import { PhArrowLeft, PhArrowRight } from "@phosphor-icons/vue";

const showDrawer = ref(true);
const emit = defineEmits(["close", "next", "back"]);
const props = defineProps({
   show: {
      type: Boolean,
      default: false,
   },
});
watchEffect(() => {
   showDrawer.value = props.show;
});

const firstName = ref("");
const lastName = ref("");
const email = ref("");
const address = ref("");
const city = ref("");
const state = ref("");
const zipCode = ref("");
const country = ref("");

const isCompleteAndCorrect = computed(() => {
   return (
      firstName.value &&
      lastName.value &&
      email.value &&
      address.value &&
      city.value &&
      state.value &&
      zipCode.value &&
      country.value
   );
});

const isValidEmail = (email) => {
   const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
   return emailRegex.test(email);
};

const noValidEmail = ref(false);

const handleCloseDrawer = () => {
   emit("close");
};

const handleNext = () => {
   if (!isValidEmail(email.value)) {
      noValidEmail.value = true;
      return;
   }
   if (isCompleteAndCorrect.value) {
      emit("next");
   }
};

const handleBack = () => {
   emit("back");
};

const validateEmail = () => {
   noValidEmail.value = !!email.value && !isValidEmail(email.value);
};
</script>

<style scoped>
.form-input {
   @apply w-full rounded-lg border border-gray-300 px-2 py-4 focus:border-resin-500 focus:outline-none focus:ring-0;
}

.invalid-input {
   @apply w-full rounded-lg border-2 border-red-500 px-2 py-4 focus:outline-none focus:ring-0 !important;
}
</style>
