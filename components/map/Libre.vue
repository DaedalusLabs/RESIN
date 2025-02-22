<template>
   <div class="relative flex h-full w-full items-center justify-center">
      <div ref="mapContainer" class="h-full w-full" />

      <!-- Loading Indicator -->
      <div
         v-if="isMapLoading"
         class="absolute inset-0 flex items-center justify-center bg-white bg-opacity-75"
      >
         <div class="loader"></div>
      </div>

      <!-- Button to View Properties -->

      <NuxtLinkLocale class="fixed bottom-20 lg:hidden" to="properties">
         <FlowbiteButton
            :text="$t('map.viewProperties', { count: visibleLocationsAmount })"
            class="rounded bg-resin-500 px-4 py-2 text-white hover:bg-resin-600"
         />
      </NuxtLinkLocale>

      <!-- GPS Location Button -->
      <button
         class="fixed bottom-20 right-4 flex h-12 w-12 items-center justify-center rounded-full bg-white p-2 shadow-md"
         @click="getUserLocation"
      >
         <PhGps :size="32" />
      </button>

      <!-- Property Card Transition -->
      <transition
         enter-active-class="slide-up-enter"
         leave-active-class="slide-up-leave"
      >
         <PropertyCard
            v-if="clickedLocation"
            :key="clickedLocation.id"
            :show-media-icon="false"
            :property="clickedLocation"
            class="force-top property-card fixed bottom-0 w-screen rounded-none bg-white shadow-lg"
         />
         <MultiPropertyCard
            v-else-if="clickedLocations.length"
            :key="'multi'"
            :properties="clickedLocations"
            class="force-top property-card fixed bottom-0 w-screen rounded-none bg-white shadow-lg"
         />
      </transition>

      <!-- Add Alert component -->
      <ResinAlert
         :show="showOverlapAlert"
         :text="$t('map.multipleProperties', { count: overlapCount })"
         type="info"
         position="top"
         class="z-[10000]"
      />
   </div>
</template>

<script setup>
import { usePropertiesStore } from "~/stores/properties";
import { PhGps } from "@phosphor-icons/vue";
import { fixNestedStrings } from "~/utils/jsonParser";
import MultiPropertyCard from "~/components/property-card/MultiPropertyCard.vue";

const { $createMap } = useNuxtApp();

const propertiesStore = usePropertiesStore();
let properties = propertiesStore.properties;
const zoom = ref(4);
const mapContainer = ref(null);
const map = ref(null);
const visibleLocationsAmount = ref(propertiesStore.filteredProperties.length);
const userLocation = ref(null);
const clickedLocation = ref(null);
const clickedLocations = ref([]);
const route = useRoute();
const showOverlapAlert = ref(false);
const overlapCount = ref(0);
const isMapLoading = ref(true);

const props = defineProps({
   mapCenter: {
      type: Object,
      required: true,
   },
   results: {
      type: Array,
      required: true,
   },
});

const getUserLocation = () => {
   if (navigator.geolocation && userLocation.value === null) {
      navigator.geolocation.getCurrentPosition((position) => {
         userLocation.value = {
            lat: position.coords.latitude,
            lng: position.coords.longitude,
         };
         map.value.setCenter([userLocation.value.lng, userLocation.value.lat]);
         map.value.setZoom(15);
      });
   }
};

const calculateVisibleLocations = () => {
   const bounds = map.value.getBounds();
   const visibleLocations = properties.filter((property) =>
      bounds.contains([
         property.location.coordinates[0],
         property.location.coordinates[1],
      ]),
   );

   visibleLocationsAmount.value = visibleLocations.length;
   propertiesStore.setFilteredLocations(visibleLocations);
   refreshProperties();
};

let geojson;

const refreshProperties = () => {
   geojson = {
      type: "FeatureCollection",
      features: properties.map((location) => ({
         type: "Feature",
         geometry: {
            type: "Point",
            coordinates: [
               location.location.coordinates[0],
               location.location.coordinates[1],
            ],
         },
         properties: location,
      })),
   };

   map.value.getSource("properties")?.setData(geojson);
};

onMounted(async () => {
   try {
      map.value = await $createMap(mapContainer.value, {
         center: [props.mapCenter.lng, props.mapCenter.lat],
         zoom: zoom.value,
         maxZoom: 15,
      });

      map.value.on("styleimagemissing", (e) => {
         const emptyImage = new Uint8Array(4).fill(0);
         map.value.addImage(e.id, { width: 1, height: 1, data: emptyImage });
      });

      map.value.on("load", () => {
         isMapLoading.value = false;
         refreshProperties();

         map.value.addSource("properties", {
            type: "geojson",
            data: geojson,
            cluster: true,
            clusterMaxZoom: 10,
            clusterRadius: 50,
         });

         // Add cluster circles
         map.value.addLayer({
            id: "clusters",
            type: "circle",
            source: "properties",
            filter: ["has", "point_count"],
            paint: {
               "circle-stroke-width": 3,
               "circle-stroke-color": "#fff",

               "circle-color": [
                  "step",
                  ["get", "point_count"],
                  "#F07E19",
                  10,
                  "#F07E19",
                  100,
                  "#F07E19",
               ],
               "circle-radius": [
                  "step",
                  ["get", "point_count"],
                  15,
                  10,
                  20,
                  100,
                  20,
               ],
            },
            // layout: {
            //    'cursor': 'pointer'
            // }
         });

         map.value.addLayer({
            id: "cluster-count",
            type: "symbol",
            source: "properties",
            filter: ["has", "point_count"],
            layout: {
               "text-field": "{point_count_abbreviated}",
               "text-size": 15,
               "text-font": ["Noto Sans Bold"], // Use Inter if it's available
            },
            paint: {
               "text-color": "#ffffff",
            },
         });

         map.value.addLayer({
            id: "unclustered-point",
            type: "circle",
            source: "properties",
            filter: ["!", ["has", "point_count"]],
            paint: {
               "circle-color": "#F07E19",
               "circle-radius": [
                  "interpolate",
                  ["linear"],
                  ["zoom"],
                  0,
                  8, // zoom level 0-12: radius 8
                  10,
                  10, // zoom level 13: radius 8
                  14,
                  22, // zoom level 14+: radius 12
                  17,
                  30, // zoom level 17+: radius 30
               ],
               "circle-stroke-width": 2,
               "circle-stroke-color": "#fff",
            },
         });

         map.value.on("click", "unclustered-point", (e) => {
            e.originalEvent.stopPropagation();

            // Query all points at the clicked location
            const bbox = [
               [e.point.x - 5, e.point.y - 5],
               [e.point.x + 5, e.point.y + 5],
            ];
            const features = map.value.queryRenderedFeatures(bbox, {
               layers: ["unclustered-point"],
            });

            if (features.length > 1) {
               overlapCount.value = features.length;
               showOverlapAlert.value = true;
               setTimeout(() => {
                  showOverlapAlert.value = false;
               }, 3000);
               clickedLocations.value = features.map((f) =>
                  fixNestedStrings(f.properties),
               );
               clickedLocation.value = null;
            } else {
               clickedLocation.value = fixNestedStrings(features[0].properties);
               clickedLocations.value = [];
            }
         });

         // click on a cluster
         map.value.on("click", "clusters", async (e) => {
            e.originalEvent.stopPropagation();
            const features = map.value.queryRenderedFeatures(e.point, {
               layers: ["clusters"],
            });
            const clusterId = features[0].properties.cluster_id;
            const zoom = await map.value
               .getSource("properties")
               .getClusterExpansionZoom(clusterId);
            map.value.easeTo({
               center: features[0].geometry.coordinates,
               zoom,
            });
         });
         calculateVisibleLocations();

         map.value.on("mouseenter", "clusters", () => {
            map.value.getCanvas().style.cursor = "pointer";
         });

         map.value.on("mouseleave", "clusters", () => {
            map.value.getCanvas().style.cursor = "grab";
         });

         map.value.on("mouseenter", "unclustered-point", () => {
            map.value.getCanvas().style.cursor = "pointer";
         });

         map.value.on("mouseleave", "unclustered-point", () => {
            map.value.getCanvas().style.cursor = "grab";
         });

         map.value.on("moveend", calculateVisibleLocations);
         map.value.on("zoomend", calculateVisibleLocations);
      });

      if (route.query.lat && route.query.lng) {
         map.value.flyTo({
            center: [route.query.lng, route.query.lat],
            zoom: 15,
            essential: true,
         });
      } else {
         const bounds = map.value.getBounds();
         propertiesStore.properties.forEach((location) => {
            bounds.extend([
               location.location.coordinates[0],
               location.location.coordinates[1],
            ]);
         });

         map.value.fitBounds(bounds, {
            // padding: 100,
            linear: false,
            duration: 500,
         });
      }
   } catch (error) {
      console.error("Error initializing map:", error);
      isMapLoading.value = false;
   }
});

onUnmounted(() => {
   map.value?.remove();
});

watch(
   () => props.mapCenter,
   (newCenter) => {
      if (map.value && newCenter) {
         map.value.setCenter([newCenter.lng, newCenter.lat]);
         map.value.setZoom(15);
      }
   },
   { immediate: true },
);

watch(
   () => props.results,
   (newResults) => {
      // Handle the search results change
      propertiesStore.properties = newResults;
      properties = propertiesStore.properties;
      if (properties.length && map.value) {
         calculateVisibleLocations();
      }
   },
   { deep: true, immediate: true },
);

// Click Outside Handling
const clickedOutside = (event) => {
   const propertyCard = document.querySelector(".property-card");
   if (propertyCard && !propertyCard.contains(event.target)) {
      clickedLocation.value = null;
      clickedLocations.value = [];
      removeClickOutsideListener();
   }
};

const addClickOutsideListener = () => {
   setTimeout(() => {
      document.addEventListener("click", clickedOutside);
   }, 100);
};

const removeClickOutsideListener = () => {
   document.removeEventListener("click", clickedOutside);
};

// Watch for click events outside of the property card
watchEffect(() => {
   if (clickedLocation.value || clickedLocations.value.length) {
      addClickOutsideListener();
   } else {
      removeClickOutsideListener();
   }
});
</script>

<style scoped>
/* High z-index to ensure elements appear above others */
.force-top {
   z-index: 9999;
}

/* Slide Up/Down Animation */
@keyframes slideUp {
   0% {
      transform: translateY(100%);
      opacity: 0;
   }
   100% {
      transform: translateY(0);
      opacity: 1;
   }
}

@keyframes slideDown {
   0% {
      transform: translateY(0);
      opacity: 1;
   }
   100% {
      transform: translateY(100%);
      opacity: 0;
   }
}

.slide-up-enter {
   animation: slideUp 0.2s ease-out forwards;
}

.slide-up-leave {
   animation: slideDown 0.2s ease-in forwards;
}

:deep(.maplibregl-canvas-container) {
   cursor: grab;
}

:deep(.maplibregl-canvas-container.maplibregl-interactive:active) {
   cursor: grabbing;
}

/* Cluster and point interaction cursors
:deep(.maplibregl-canvas-container.maplibregl-interactive[class*="clusters"]:hover),
:deep(.maplibregl-canvas-container.maplibregl-interactive[class*="unclustered-point"]:hover) {
  cursor: pointer !important;
} */

.loader {
   width: 48px;
   height: 48px;
   border: 5px solid #f07e19;
   border-bottom-color: transparent;
   border-radius: 50%;
   display: inline-block;
   box-sizing: border-box;
   animation: rotation 1s linear infinite;
}

@keyframes rotation {
   0% {
      transform: rotate(0deg);
   }
   100% {
      transform: rotate(360deg);
   }
}
</style>
