// eslint-disable-next-line vue/no-v-html
<template>
   <section class="flex max-h-full flex-col justify-between">
      <!-- Header -->
      <div
         class="mb-5 flex w-full flex-col gap-5 border-b border-gray-200 bg-white px-10 pb-5 pt-10"
      >
         <NuxtLinkLocale @click="goBack">
            <PhCaretLeft :size="24" class="text-pirate-300" />
         </NuxtLinkLocale>
         <h1 class="text-2xl font-extrabold leading-tight text-pirate-950">
            {{ $t("termsAndConditions.title") }}
         </h1>
         <p class="text-sm text-pirate-600">
            {{ $t("termsAndConditions.lastUpdated", { date: "Jan 7, 2025" }) }}
         </p>
      </div>

      <!-- Content -->
      <div class="flex-1 px-10 pb-24">
         <div class="flex flex-col gap-4">
            <!-- Accordion Items -->
            <div
               v-for="(section, index) in sections"
               :key="index"
               class="rounded-lg border border-gray-200"
            >
               <button
                  class="flex w-full items-center justify-between rounded-lg px-4 py-3 text-left hover:bg-gray-50"
                  :class="{ 'rounded-b-none': openSections[index] }"
                  @click="toggleSection(index)"
               >
                  <h2 class="text-base font-semibold text-pirate-950">
                     {{ section.title }}
                  </h2>
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
                  <div class="prose prose-sm text-pirate-600">
                     <MarkdownRenderer :content="section.content" />
                  </div>
               </div>
            </div>
         </div>
      </div>
   </section>
</template>

<script setup lang="ts">
import { PhCaretLeft, PhCaretDown } from "@phosphor-icons/vue";
import MarkdownRenderer from "~/components/MarkdownRenderer.vue";

interface Section {
   title: string;
   content: string;
}

const { t } = useI18n();
const router = useRouter();

const openSections = ref<boolean[]>([]);
const sections = ref<Section[]>([]);

onMounted(() => {
   // Define section keys
   const sectionKeys = [
      "acceptanceOfTerms",
      "descriptionOfService",
      "userEligibility",
      "userAccounts",
      "userConduct",
      "content",
      "paymentAndFees",
      "transactions",
      "disclaimers",
      "termination",
      "governingLaw",
      "changes",
      "contactInfo",
   ];

   // Initialize sections safely
   sections.value = sectionKeys.map((key) => {
      const title = t(`termsAndConditions.sections.${key}.title`) || key;
      const content =
         t(`termsAndConditions.sections.${key}.content`, {
            contact: "hello@daedaluslabs.io",
         }) || "Content not available";

      return { title, content };
   });

   // Initialize openSections array based on actual sections length
   openSections.value = new Array(sections.value.length).fill(true);
});

const toggleSection = (index: number) => {
   openSections.value[index] = !openSections.value[index];
};

const goBack = () => {
   router.back();
};

useHead({
   title: t("termsAndConditions.title"),
});

definePageMeta({
   layout: "white",
});
</script>
