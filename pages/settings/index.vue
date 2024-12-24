<template>
    <section class="flex max-h-full flex-col justify-between">
        <!-- Header -->
        <div class="mb-5 flex w-full flex-col gap-5 border-b border-gray-200 bg-white px-10 pb-5 pt-10">
            <NuxtLinkLocale @click="goBack">
                <PhCaretLeft :size="24" class="text-pirate-300" />
            </NuxtLinkLocale>
            <h1 class="text-2xl font-extrabold leading-tight text-pirate-950">
                Settings
            </h1>
        </div>

        <!-- Settings Content -->
        <div class="flex-1 px-10">
            <div class="flex flex-col gap-6">
                <!-- Currency Selection -->
                <div class="flex flex-col gap-2">
                    <label class="text-sm font-medium text-pirate-950">Currency</label>
                    <select v-model="settingsStore.display.currency"
                        class="w-full rounded-lg border border-gray-300 p-2.5 text-pirate-950 focus:border-resin-500 focus:ring-resin-500">
                        <option value="USD">USD - US Dollar</option>
                        <option value="SRD">SRD - Surinamese Dollar</option>
                    </select>
                </div>

                <!-- Language Selection -->
                <div class="flex flex-col gap-2">
                    <label class="text-sm font-medium text-pirate-950">Language</label>
                    <select :value="locale" @change="switchLanguage($event.target.value)"
                        class="w-full rounded-lg border border-gray-300 p-2.5 text-pirate-950 focus:border-resin-500 focus:ring-resin-500">
                        <option v-for="locale in locales" :key="locale.code" :value="locale.code">
                            {{ locale.name }}
                        </option>
                    </select>
                </div>
            </div>
        </div>
    </section>
</template>

<script setup>
import { useSettingsStore } from '~/stores/settings';
import { PhCaretLeft } from "@phosphor-icons/vue";
const { locale, locales, setLocale } = useI18n()

const settingsStore = useSettingsStore();
const switchLocalePath = useSwitchLocalePath()
const router = useRouter()

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
    console.log("newLocale", newLocale);
  await router.push(switchLocalePath(newLocale))
}

definePageMeta({
  layout: "white",
});
</script> 