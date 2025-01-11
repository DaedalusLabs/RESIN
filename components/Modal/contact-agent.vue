<template>
   <FlowbiteModal
      :is-open="isOpen"
      @update:is-open="$emit('update:isOpen', $event)"
   >
      <template #title>
         <h3 class="text-lg font-bold text-gray-900">Contact agent</h3>
      </template>
      <div class="mt-4">
         <p class="text-gray-900">Property</p>
         <p class="font-bold text-gray-900">{{ propertyAddress }}</p>
         <br />
         <p>
            Thank you for your interest in this property. Please leave your
            preferred contact information and our agent will contact you
            shortly.
         </p>
         <div class="mt-6">
            <FlowbiteTextInput
               v-model="phone"
               label="Phone"
               placeholder="Phone"
               class="mb-1"
               type="tel"
               :color="phoneError ? 'red' : 'gray'"
            />
            <p v-if="phoneError" class="mb-4 text-sm text-red-600">
               Please enter a valid phone number
            </p>
         </div>
         <div class="mt-6 flex items-center justify-center">
            <div class="flex-grow border-t border-gray-300"></div>
            <span class="mx-3 text-sm font-medium text-gray-500">Or</span>
            <div class="flex-grow border-t border-gray-300"></div>
         </div>
         <div class="mt-6">
            <FlowbiteTextInput
               v-model="email"
               label="Email"
               placeholder="Email"
               class="mb-1"
               type="email"
               :color="emailError ? 'red' : 'gray'"
            />
            <p v-if="emailError" class="mb-4 text-sm text-red-600">
               Please enter a valid email address
            </p>
         </div>
         <div class="mt-6 flex justify-center">
            <FlowbiteButton
               :text="`Submit`"
               :disabled="!isFormValid"
               @click="handleSendRequest"
            />
         </div>
      </div>
   </FlowbiteModal>
</template>

<script setup>
import { ref, computed } from "vue";

const nostrStore = useNostrStore();
const runtimeConfig = useRuntimeConfig();

const props = defineProps({
   isOpen: {
      type: Boolean,
      required: true,
   },
   property: {
      type: Object,
      required: true,
   },
});

const propertyAddress = computed(() => {
   return `${props.property.title} ${props.property.location.street}, ${props.property.location.city}, ${props.property.location.country}`;
});

const phone = ref("");
const email = ref("");
const formError = ref(false);

const emit = defineEmits(["update:isOpen", "sendRequest"]);

// Validation functions
const isValidEmail = (email) => {
   const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
   return email === "" || emailRegex.test(email);
};

const isValidPhone = (phone) => {
   // Allows formats like: +1234567890, 123-456-7890, (123) 456-7890
   const phoneRegex = /^$|^\+?[\d\s-()]{10,}$/;
   return phone === "" || phoneRegex.test(phone);
};

// Computed properties for validation
const emailError = computed(
   () => email.value !== "" && !isValidEmail(email.value),
);
const phoneError = computed(
   () => phone.value !== "" && !isValidPhone(phone.value),
);

const isFormValid = computed(() => {
   // At least one field must be filled and valid
   return (
      (email.value === "" || isValidEmail(email.value)) &&
      (phone.value === "" || isValidPhone(phone.value)) &&
      (email.value !== "" || phone.value !== "")
   );
});

const handleSendRequest = async () => {
   if (isFormValid.value) {
      // Create message content based on provided contact details
      let contactMessage = `I want contact with an agent about ${propertyAddress.value}.`;

      if (email.value && phone.value) {
         contactMessage += ` You can reach me by email at ${email.value} or by phone at ${phone.value}.`;
      } else if (email.value) {
         contactMessage += ` You can reach me by email at ${email.value}.`;
      } else if (phone.value) {
         contactMessage += ` You can reach me by phone at ${phone.value}.`;
      }

      await nostrStore.sendDirectMessage(
         runtimeConfig.public.MESSAGES_PUBKEY,
         contactMessage,
         props.property.id,
         props.property.kind,
      );

      emit("sendRequest", {
         email: email.value,
         phone: phone.value,
      });
   } else {
      formError.value = true;
   }
};
</script>
