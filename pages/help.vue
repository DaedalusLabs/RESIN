<template>
  <section class="flex max-h-full flex-col justify-between">
    <!-- Header -->
    <div class="mb-5 flex w-full flex-col gap-5 border-b border-gray-200 bg-white px-10 pb-5 pt-10">
      <NuxtLinkLocale @click="goBack">
        <PhCaretLeft :size="24" class="text-pirate-300" />
      </NuxtLinkLocale>
      <h1 class="text-2xl font-extrabold leading-tight text-pirate-950">
        {{ $t('help.title') }}
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
            :placeholder="$t('help.search')"
            class="w-full rounded-lg border border-gray-200 py-2.5 pl-10 pr-4 text-pirate-950 focus:border-resin-500 focus:ring-resin-500"
          />
        </div>
      </div>

      <!-- FAQ Categories -->
      <div class="mb-6 flex flex-wrap gap-2">
        <button
          v-for="category in categories"
          :key="category.key"
          @click="selectedCategory = category.key"
          class="rounded-full px-4 py-1.5 text-sm font-medium transition-colors"
          :class="selectedCategory === category.key ? 'bg-resin-500 text-white' : 'bg-gray-100 text-pirate-600 hover:bg-gray-200'"
        >
          {{ category.label }}
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
          {{ $t('help.cantFind') }}
          <a href="mailto:support@resin.estate" class="text-resin-500 hover:text-resin-600">{{ $t('help.contactSupport') }}</a>
        </p>
      </div>
    </div>
  </section>
</template>

<script setup>
import { PhCaretLeft, PhCaretDown, PhMagnifyingGlass } from "@phosphor-icons/vue";

const { t } = useI18n();

const openSections = ref([]);
const searchQuery = ref('');
const selectedCategory = ref('All');

useHead({
   title: t('help.title'),
});

const categories = [
  { key: 'all', label: t('help.categories.all') },
  { key: 'account', label: t('help.categories.account') },
  { key: 'properties', label: t('help.categories.properties') },
  { key: 'payments', label: t('help.categories.payments') },
  { key: 'technical', label: t('help.categories.technical') }
];

const faqs = [
  {
    category: 'account',
    question: t('help.faq.whatIsNostr.question'),
    answer: t('help.faq.whatIsNostr.answer')
  },
  {
    category: 'account',
    question: t('help.faq.accountRecovery.question'),
    answer: t('help.faq.accountRecovery.answer')
  },
  {
    category: 'properties',
    question: t('help.faq.rentToOwn.question'),
    answer: t('help.faq.rentToOwn.answer')
  },
  {
    category: 'properties',
    question: t('help.faq.listProperty.question'),
    answer: t('help.faq.listProperty.answer')
  },
  {
    category: 'payments',
    question: t('help.faq.paymentMethods.question'),
    answer: t('help.faq.paymentMethods.answer')
  },
  {
    category: 'payments',
    question: t('help.faq.equityCalculation.question'),
    answer: t('help.faq.equityCalculation.answer')
  },
  {
    category: 'technical',
    question: t('help.faq.browsers.question'),
    answer: t('help.faq.browsers.answer')
  },
  {
    category: 'technical',
    question: t('help.faq.dataSecurity.question'),
    answer: t('help.faq.dataSecurity.answer')
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