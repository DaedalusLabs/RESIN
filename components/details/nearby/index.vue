<template>
   <div class="container mx-auto mt-8">
      <h2 class="text-xl font-bold">
         {{ $t("property.details.nearby.title") }}
      </h2>
      <div class="mt-4 grid grid-cols-1 gap-8">
         <div v-if="!nearbyProperties.length">
            <p>{{ $t("property.details.nearby.noProperties") }}</p>
         </div>
         <div
            v-for="nearbyProperty in nearbyProperties"
            :key="nearbyProperty.id"
         >
            <PropertyCard :property="nearbyProperty" :show-media-icon="false" />
         </div>
      </div>
   </div>
</template>

<script setup>
const nearbyProperties = ref([]);
const runtimeConfig = useRuntimeConfig();

const props = defineProps({
   property: {
      type: Object,
      required: true,
   },
});

onMounted(async () => {
   let data = [];
   try {
      data = await (
         await fetch(
            `${runtimeConfig.public.BACKEND_ENDPOINT}/listings/get_nearby/${props.property.id}`,
         )
      ).json();
      nearbyProperties.value = data.hits.map((h) => h.document);
   } catch (error) {
      console.error("Failed to fetch nearby properties:", error);
   }
});
</script>
