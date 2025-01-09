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
      <p class="text-sm text-pirate-600">Last Updated: Jan 7, 2025</p>
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

const openSections = ref(new Array(13).fill(true)); // Updated to match new number of sections

const sections = [
  {
    title: "Acceptance of Terms",
    content: "By accessing or using RESIN, you agree to be bound by these Terms and Conditions (\"Terms\"). If you do not agree to all the terms and conditions of this agreement, then you may not access the website or use any services."
  },
  {
    title: "Description of Service",
    content: "RESIN provides an online platform for real estate transactions including the listing, marketing, and sale or long-term rental of residential properties. Our services include but are not limited to property listings, facilitating communication between buyers/renters and sellers/landlords, and providing tools for transaction management."
  },
  {
    title: "User Eligibility",
    content: "You must be at least 18 years old to use this service. By accessing or using RESIN, you represent and warrant that you have the right, authority, and capacity to enter into this agreement and to abide by all of the terms and conditions of this agreement."
  },
  {
    title: "User Accounts",
    content: "Account Creation: You may need to create an account to use certain features of the service. You agree to provide accurate, current, and complete information during the registration process and to update such information to keep it accurate, current, and complete.<br><br>Account Security: You are responsible for safeguarding the private key that you use to access the Service and for any activities or actions under your private key. RESIN cannot and will not be liable for any loss or damage from your failure to comply with this security obligation."
  },
  {
    title: "User Conduct",
    content: "You agree not to use RESIN to:<br>• Violate any local, state, national, or international law or regulation.<br>• Infringe upon or violate our intellectual property rights or the intellectual property rights of others.<br>• Transmit any viruses, worms, defects, Trojan horses, or any items of a destructive nature.<br>• Engage in any behavior that is abusive, harassing, threatening, or defamatory."
  },
  {
    title: "Content",
    content: "User Content: You are solely responsible for the content you post on RESIN. You retain all rights in, and are solely responsible for, the User Content you post to the Service."
  },
  {
    title: "Payment and Fees",
    content: "All fees associated with listing properties, transaction services, or any additional features are due immediately and are non-refundable unless otherwise stated. RESIN may use third-party payment processors to handle payment transactions."
  },
  {
    title: "Transactions",
    content: "All real estate transactions facilitated through RESIN are subject to the terms agreed upon by the parties involved. RESIN does not act as a real estate agent, broker, or intermediary in these transactions but merely provides a platform for these activities to occur."
  },
  {
    title: "Disclaimers and Limitation of Liability",
    content: "RESIN is provided \"as is\" and \"as available\" without warranty of any kind. We disclaim all warranties to the fullest extent permitted by law.<br><br>In no event will RESIN, its officers, directors, employees, or agents be liable to you for any indirect, incidental, special, consequential or punitive damages."
  },
  {
    title: "Termination",
    content: "We may terminate or suspend access to our Service immediately, without prior notice or liability, for any reason whatsoever, including without limitation if you breach the Terms."
  },
  {
    title: "Governing Law",
    content: "These Terms shall be governed and construed in accordance with the laws of the State of Delaware, without regard to its conflict of law provisions."
  },
  {
    title: "Changes to Terms",
    content: "RESIN reserves the right, at its sole discretion, to modify or replace these Terms at any time. If a revision is material, we will try to provide at least 30 days' notice prior to any new terms taking effect."
  },
  {
    title: "Contact Information",
    content: "For any questions about these Terms, please contact us at hello@daedaluslabs.io"
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