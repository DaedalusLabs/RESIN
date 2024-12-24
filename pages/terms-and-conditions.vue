<template>
  <section class="flex max-h-full flex-col justify-between">
    <!-- Header -->
    <div class="mb-5 flex w-full flex-col gap-5 border-b border-gray-200 bg-white px-10 pb-5 pt-10">
      <NuxtLinkLocale @click="goBack">
        <PhCaretLeft :size="24" class="text-pirate-300" />
      </NuxtLinkLocale>
      <h1 class="text-2xl font-extrabold leading-tight text-pirate-950">
        Terms & Conditions
      </h1>
    </div>

    <!-- Content -->
    <div class="flex-1 px-10 pb-24">
      <div class="flex flex-col gap-4">
        <!-- Accordion Items -->
        <div v-for="(section, index) in sections" :key="index" class="rounded-lg border border-gray-200">
          <button
            @click="toggleSection(index)"
            class="flex w-full items-center justify-between rounded-lg px-4 py-3 text-left hover:bg-gray-50"
            :class="{ 'rounded-b-none': openSections[index] }"
          >
            <h2 class="text-base font-semibold text-pirate-950">{{ section.title }}</h2>
            <PhCaretDown
              :size="20"
              :class="{ 'rotate-180 transform': openSections[index] }"
              class="text-pirate-300 transition-transform duration-200"
            />
          </button>
          <div
            v-show="openSections[index]"
            class="border-t border-gray-200 px-4 py-3"
          >
            <p class="text-sm text-pirate-600" v-html="section.content"></p>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<script setup>
import { PhCaretLeft, PhCaretDown } from "@phosphor-icons/vue";

const openSections = ref(new Array(8).fill(true)); // Initialize all sections as open

const sections = [
  {
    title: "1. Acceptance of Terms",
    content: "By accessing and using RESIN, you agree to be bound by these Terms and Conditions. If you do not agree with any part of these terms, you may not use our services."
  },
  {
    title: "2. User Registration",
    content: "To use certain features of RESIN, you must register for an account. You agree to provide accurate and complete information during registration and to keep your account information updated."
  },
  {
    title: "3. Privacy Policy",
    content: "Your use of RESIN is also governed by our Privacy Policy. By using RESIN, you consent to the collection and use of your information as described in the Privacy Policy."
  },
  {
    title: "4. Property Listings",
    content: "Property listings on RESIN are provided for informational purposes only. We do not guarantee the accuracy or completeness of any listing information."
  },
  {
    title: "5. User Conduct",
    content: "You agree not to use RESIN for any unlawful purpose or in any way that could damage, disable, overburden, or impair our services."
  },
  {
    title: "6. Intellectual Property",
    content: "All content on RESIN, including text, graphics, logos, and software, is the property of RESIN or its licensors and is protected by intellectual property laws."
  },
  {
    title: "7. Limitation of Liability",
    content: "RESIN is provided 'as is' without any warranties. We shall not be liable for any damages arising from your use of our services."
  },
  {
    title: "8. Changes to Terms",
    content: "We reserve the right to modify these Terms and Conditions at any time. Continued use of RESIN after changes constitutes acceptance of the modified terms."
  }
];

const toggleSection = (index) => {
  openSections.value[index] = !openSections.value[index];
};

const goBack = () => {
  if (window.history.length > 1) {
    window.history.back();
  } else {
    const { localePath } = useNuxtApp();
    window.location.href = localePath("home");
  }
};

definePageMeta({
  layout: "white",
});
</script> 