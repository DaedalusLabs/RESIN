<template>
   <div class="mx-auto mt-8 px-4 md:container md:px-0">
      <h2 class="text-xl font-bold">
         {{ $t("property.details.nearby.title") }}
      </h2>
      <div class="mt-4">
         <div
            class="grid w-full justify-items-stretch gap-6 lg:grid-cols-2 xl:grid-cols-3"
         >
            <div v-if="!nearbyProperties.length" class="w-full">
               <p>{{ $t("property.details.nearby.noProperties") }}</p>
            </div>
            <div
               v-for="nearbyProperty in nearbyProperties"
               :key="nearbyProperty.id"
            >
               <PropertyCard
                  :property="nearbyProperty"
                  :show-media-icon="false"
               />
            </div>
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
