<template>
   <div class="container mx-auto mt-8">
      <h2 class="text-base font-bold">
         {{ $t("property.details.map.title") }}
      </h2>
      <div class="mt-4 overflow-hidden rounded-2xl">
         <div ref="mapContainer" class="h-64">
            <NuxtLink @click="goToMap">
               <div
                  class="force-top absolute bottom-0 flex w-full cursor-pointer justify-center bg-black/20 p-4 text-sm text-white backdrop-blur-md"
               >
                  <p>{{ $t("property.details.map.viewOnMap") }}</p>
               </div>
            </NuxtLink>
         </div>
      </div>
   </div>
</template>

<script setup>
import "maplibre-gl/dist/maplibre-gl.css";

const { $createMap, $maplibregl: maplibregl } = useNuxtApp();
const mapContainer = ref(null);
const map = ref(null);
const props = defineProps({
   property: {
      type: Object,
      required: true,
   },
});

onMounted(async () => {
   try {
      map.value = await $createMap(mapContainer.value, {
         center: [
            props.property.location.coordinates[0],
            props.property.location.coordinates[1],
         ],
         zoom: 13,
         dragPan: false,
      });

      map.value.scrollZoom.disable();

      const el = document.createElement("div");
      el.className = "marker";

      el.style.background = "#F07E19";
      el.style.width = "20px";
      el.style.height = "20px";
      el.style.borderRadius = "50%";
      el.style.border = "2px solid #fff";

      new maplibregl.Marker({ element: el })
         .setLngLat([
            props.property.location.coordinates[0],
            props.property.location.coordinates[1],
         ])
         .addTo(map.value);
   } catch (error) {
      console.error("Error initializing map:", error);
   }
});

const goToMap = () => {
   const localeRoute = useLocaleRoute();
   const long = props.property.location.coordinates[0];
   const lat = props.property.location.coordinates[1];
   const route = localeRoute({
      name: "map",
      query: { lat, lng: long },
   });
   if (route) {
      return navigateTo(route.fullPath);
   }
};
</script>

<style scoped>
.force-top {
   z-index: 10;
}
</style>
