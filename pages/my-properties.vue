<template>
   <section class="mx-auto flex w-10/12 flex-col py-20">
      <div class="mb-5 flex items-center justify-between text-pirate-950">
         <div class="flex items-center gap-2">
            <NuxtLinkLocale to="my-resin">
               <PhCaretLeft :size="28" />
            </NuxtLinkLocale>
            <h1 class="text-2xl font-extrabold leading-tight">My properties</h1>
         </div>
         <button :v-if="isSupported" @click="startShare">
            <PhExport :size="28" class="text-xl text-pirate-950" />
         </button>
      </div>
      <!-- if no properties yet -->
      <div
         v-if="properties.length === 0"
         class="items center flex flex-col items-center justify-center"
      >
         <p class="mt-20 gap-2 font-semibold text-pirate-950">
            You have no properties yet
         </p>
         <NuxtLinkLocale to="properties">
            <p
               class="mt-4 inline-block rounded-lg border-2 border-resin-500 bg-resin-500 px-4 py-2 text-white"
            >
               Browse listings
            </p>
         </NuxtLinkLocale>
      </div>
      <div v-else class="space-y-4">
         <FavoritesCard
            v-for="property in properties"
            :key="property.id"
            :is-removable="false"
            :property="property"
         />
      </div>
   </section>
</template>

<script setup>
import { usePropertiesStore } from "~/stores/properties";
import { useShare } from "@vueuse/core";
import { PhExport, PhCaretLeft } from "@phosphor-icons/vue";

const { share, isSupported } = useShare();

function startShare() {
   share({
      title: "Resin",
      text: "Check out my properties on Resin",
      url: location.href,
   });
}

const propertiesStore = usePropertiesStore();

const properties = computed(() => propertiesStore.ownedProperties);

definePageMeta({
   layout: "white",
});

onMounted(() => {
   properties.value = propertiesStore.ownedProperties;
});
</script>
