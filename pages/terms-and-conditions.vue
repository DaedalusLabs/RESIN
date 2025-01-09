<template>
  <section class="flex max-h-full flex-col justify-between">
    <!-- Header -->
    <div class="mb-5 flex w-full flex-col gap-5 border-b border-gray-200 bg-white px-10 pb-5 pt-10">
      <NuxtLinkLocale @click="goBack">
        <PhCaretLeft :size="24" class="text-pirate-300" />
      </NuxtLinkLocale>
      <h1 class="text-2xl font-extrabold leading-tight text-pirate-950">
        {{ $t('termsAndConditions.title') }}
      </h1>
      <p class="text-sm text-pirate-600">{{ $t('termsAndConditions.lastUpdated', { date: 'Jan 7, 2025' }) }}</p>
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

const openSections = ref(new Array(13).fill(true));

const sections = [
  {
    title: $t('termsAndConditions.sections.acceptanceOfTerms.title'),
    content: $t('termsAndConditions.sections.acceptanceOfTerms.content')
  },
  {
    title: $t('termsAndConditions.sections.descriptionOfService.title'),
    content: $t('termsAndConditions.sections.descriptionOfService.content')
  },
  {
    title: $t('termsAndConditions.sections.userEligibility.title'),
    content: $t('termsAndConditions.sections.userEligibility.content')
  },
  {
    title: $t('termsAndConditions.sections.userAccounts.title'),
    content: $t('termsAndConditions.sections.userAccounts.content')
  },
  {
    title: $t('termsAndConditions.sections.userConduct.title'),
    content: $t('termsAndConditions.sections.userConduct.content')
  },
  {
    title: $t('termsAndConditions.sections.content.title'),
    content: $t('termsAndConditions.sections.content.content')
  },
  {
    title: $t('termsAndConditions.sections.paymentAndFees.title'),
    content: $t('termsAndConditions.sections.paymentAndFees.content')
  },
  {
    title: $t('termsAndConditions.sections.transactions.title'),
    content: $t('termsAndConditions.sections.transactions.content')
  },
  {
    title: $t('termsAndConditions.sections.disclaimers.title'),
    content: $t('termsAndConditions.sections.disclaimers.content')
  },
  {
    title: $t('termsAndConditions.sections.termination.title'),
    content: $t('termsAndConditions.sections.termination.content')
  },
  {
    title: $t('termsAndConditions.sections.governingLaw.title'),
    content: $t('termsAndConditions.sections.governingLaw.content')
  },
  {
    title: $t('termsAndConditions.sections.changes.title'),
    content: $t('termsAndConditions.sections.changes.content')
  },
  {
    title: $t('termsAndConditions.sections.contactInfo.title'),
    content: $t('termsAndConditions.sections.contactInfo.content')
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