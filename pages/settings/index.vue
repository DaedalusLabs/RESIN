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
            {{ $t("settings.title") }}
         </h1>
      </div>

      <!-- Settings Content -->
      <div class="flex-1 px-10">
         <div class="flex flex-col gap-6">
            <!-- Currency Selection -->
            <div class="flex flex-col gap-2">
               <label class="text-sm font-medium text-pirate-950">
                  {{ $t("settings.currency.label") }}
               </label>
               <select
                  v-model="settingsStore.display.currency"
                  disabled
                  class="w-full rounded-lg border border-gray-300 p-2.5 text-pirate-950 focus:border-resin-500 focus:ring-resin-500"
               >
                  <option value="USD">{{ $t("settings.currency.usd") }}</option>
                  <option value="SRD">{{ $t("settings.currency.srd") }}</option>
               </select>
            </div>

            <!-- Language Selection -->
            <div class="flex flex-col gap-2">
               <label class="text-sm font-medium text-pirate-950">
                  {{ $t("settings.language.label") }}
               </label>
               <select
                  :value="locale"
                  class="w-full rounded-lg border border-gray-300 p-2.5 text-pirate-950 focus:border-resin-500 focus:ring-resin-500"
                  @change="switchLanguage($event.target.value)"
               >
                  <option v-for="l in locales" :key="l.code" :value="l.code">
                     {{ $t(`settings.language.${l.code}`) }}
                  </option>
               </select>
            </div>

            <!-- Push Notifications -->
            <div
               class="flex items-center justify-between border-b border-gray-200 py-4"
            >
               <div>
                  <h3 class="text-base font-semibold text-pirate-950">
                     {{ $t("settings.pushNotifications") }}
                  </h3>
                  <p class="text-sm text-gray-500">
                     {{ $t("settings.pushNotificationsDescription") }}
                  </p>
               </div>
               <FlowbiteSwitch
                  :model-value="nostrStore.notificationsEnabled"
                  @update:model-value="togglePushNotifications"
               />
            </div>
         </div>
      </div>
   </section>
</template>

<script setup>
import { useSettingsStore } from "~/stores/settings";
import { PhCaretLeft } from "@phosphor-icons/vue";
import FlowbiteSwitch from "~/components/Flowbite/Switch.vue";
const { locale, locales } = useI18n();

const nostrStore = useNostrStore();
const settingsStore = useSettingsStore();
const switchLocalePath = useSwitchLocalePath();
const router = useRouter();

const { t } = useI18n();

useHead({
   title: t("settings.title"),
});

const goBack = () => {
   if (window.history.length > 1) {
      window.history.back();
   } else {
      // Fallback: navigate to home if there is no history
      const { localePath } = useNuxtApp();
      window.location.href = localePath("home");
   }
};

const switchLanguage = async (newLocale) => {
   settingsStore.display.language = newLocale;
   await router.push(switchLocalePath(newLocale));
};

const togglePushNotifications = async () => {
   await nostrStore.togglePushNotifications();
};

definePageMeta({
   layout: "white",
});
</script>
