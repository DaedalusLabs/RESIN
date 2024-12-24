<template>
  <section class="flex max-h-full flex-col justify-between">
    <!-- Header -->
    <div class="mb-5 flex w-full flex-col gap-5 border-b border-gray-200 bg-white px-10 pb-5 pt-10">
      <NuxtLinkLocale @click="goBack">
        <PhCaretLeft :size="24" class="text-pirate-300" />
      </NuxtLinkLocale>
      <h1 class="text-2xl font-extrabold leading-tight text-pirate-950">
        Help
      </h1>
    </div>

    <!-- Content -->
    <div class="flex-1 px-10 pb-24">
      <!-- Search -->
      <div class="mb-6">
        <div class="relative">
          <PhMagnifyingGlass class="absolute left-3 top-1/2 -translate-y-1/2 text-pirate-300" :size="20" />
          <input
            v-model="searchQuery"
            type="search"
            placeholder="Search"
            class="w-full rounded-lg border border-gray-200 py-2.5 pl-10 pr-4 text-pirate-950 focus:border-resin-500 focus:ring-resin-500"
          />
        </div>
      </div>

      <!-- FAQ Categories -->
      <div class="mb-6 flex flex-wrap gap-2">
        <button
          v-for="category in categories"
          :key="category"
          @click="selectedCategory = category"
          class="rounded-full px-4 py-1.5 text-sm font-medium transition-colors"
          :class="selectedCategory === category ? 'bg-resin-500 text-white' : 'bg-gray-100 text-pirate-600 hover:bg-gray-200'"
        >
          {{ category }}
        </button>
      </div>

      <!-- FAQ Accordion -->
      <div class="flex flex-col gap-4">
        <div 
          v-for="(qa, index) in filteredFAQs" 
          :key="index" 
          class="rounded-lg border border-gray-200"
        >
          <button
            @click="toggleSection(index)"
            class="flex w-full items-center justify-between rounded-lg px-4 py-3 text-left hover:bg-gray-50"
            :class="{ 'rounded-b-none': openSections[index] }"
          >
            <h2 class="text-base font-semibold text-pirate-950">{{ qa.question }}</h2>
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
            <p class="text-sm text-pirate-600" v-html="qa.answer"></p>
          </div>
        </div>
      </div>

      <!-- Contact Support -->
      <div class="mt-8 rounded-lg border border-gray-200 bg-gray-50 p-4 text-center">
        <p class="text-sm text-pirate-600">
          Can't find what you're looking for?
          <a href="mailto:support@resin.estate" class="text-resin-500 hover:text-resin-600">Contact Support</a>
        </p>
      </div>
    </div>
  </section>
</template>

<script setup>
import { PhCaretLeft, PhCaretDown, PhMagnifyingGlass } from "@phosphor-icons/vue";

const openSections = ref([]);
const searchQuery = ref('');
const selectedCategory = ref('All');

const categories = ['All', 'Account', 'Properties', 'Payments', 'Technical'];

const faqs = [
  {
    category: 'Account',
    question: "What is NOSTR and why do I need it?",
    answer: "NOSTR is a decentralized protocol that allows you to own your identity and data. RESIN uses NOSTR to ensure your account and transactions are secure and private."
  },
  {
    category: 'Account',
    question: "How do I recover my account?",
    answer: "You can recover your account using your 12-word recovery phrase or your NOSTR private key. Keep these safe as they are the only way to regain access to your account."
  },
  {
    category: 'Properties',
    question: "How does rent-to-own work?",
    answer: "Rent-to-own allows you to rent a property while building equity towards ownership. Part of your monthly payment goes towards your future ownership of the property."
  },
  {
    category: 'Properties',
    question: "Can I list my property on RESIN?",
    answer: "Yes, property owners can list their properties on RESIN. Contact our support team to learn more about the listing process and requirements."
  },
  {
    category: 'Payments',
    question: "What payment methods are accepted?",
    answer: "RESIN currently accepts various payment methods including bank transfers and cryptocurrency. The available options may vary by region."
  },
  {
    category: 'Payments',
    question: "How is my equity calculated?",
    answer: "Your equity is calculated based on the portion of your monthly payments that goes towards ownership, as specified in your rent-to-own agreement."
  },
  {
    category: 'Technical',
    question: "What browsers are supported?",
    answer: "RESIN works best on modern browsers like Chrome, Firefox, Safari, and Edge. Make sure to keep your browser updated for the best experience."
  },
  {
    category: 'Technical',
    question: "Is my data secure?",
    answer: "Yes, RESIN uses NOSTR protocol and industry-standard encryption to protect your data. Your private keys and sensitive information never leave your device."
  }
];

const filteredFAQs = computed(() => {
  let filtered = faqs;
  
  // Filter by category
  if (selectedCategory.value !== 'All') {
    filtered = filtered.filter(faq => faq.category === selectedCategory.value);
  }
  
  // Filter by search query
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase();
    filtered = filtered.filter(faq => 
      faq.question.toLowerCase().includes(query) || 
      faq.answer.toLowerCase().includes(query)
    );
  }
  
  return filtered;
});

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