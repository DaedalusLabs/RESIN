<template>
   <div class="container mx-auto mt-8">
      <h2 class="text-xl font-bold">Nearby Properties</h2>
      <div class="mt-4 grid grid-cols-1 gap-8">
         <div v-if="!nearbyProperties.length">
            <p>No nearby properties found</p>
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
// const appConfig = useAppConfig()
const nearbyProperties = ref([]);
const runtimeConfig = useRuntimeConfig();

let props = defineProps({
   property: {
      type: Object,
      required: true,
   },
});

onMounted(async() => {
   let data = [];
   try {
      data = await(await fetch(`${runtimeConfig.public.BACKEND_ENDPOINT}/listings/get_nearby/${props.property.id}`)).json();   
      nearbyProperties.value = data.hits.map((h)=> h.document);
   }
   catch  {}
});
</script>
