<template>
   <div class="container mx-auto mt-8 w-10/12">
      <h2 class="text-xl font-bold">Nearby Properties</h2>
      <div class="mt-4 grid grid-cols-1 gap-8 md:grid-cols-3">
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
import { nostrEventToResult } from '~/utils/nostrEventToResult';

// const appConfig = useAppConfig()
const nearbyProperties = ref([]);
const appConfig = useAppConfig()

let props = defineProps({
   property: {
      type: Object,
      required: true,
   },
});

onMounted(async() => {
   let data = [];
   data = await(await fetch(`${appConfig.BACKEND_ENDPOINT}/listings/get_nearby/${props.property.id}`)).json();

  nearbyProperties.value = data.map((e) => nostrEventToResult(appConfig, e)); 
});
</script>
