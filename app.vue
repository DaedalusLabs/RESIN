<template>
   <div>
      <VitePwaManifest />
      <NuxtLayout>
         <NuxtPage />
      </NuxtLayout>
   </div>
</template>

<script setup>
import { usePropertiesStore } from "~/stores/properties";
const runtimeConfig = useRuntimeConfig();

const propertiesStore = usePropertiesStore();
propertiesStore.init();
const nostrStore = useNostrStore();
nostrStore.checkAuthenticated().then(() => {
   nostrStore.fetchDirectMessages();
});

async function getProperties() {
   const properties = await fetch(`${runtimeConfig.public.BACKEND_ENDPOINT}/listings`);
   return (await properties.json()).hits.map(d => d.document);
}

async function getTrendingAreas() {
   return [
    
   ];
}

propertiesStore.properties = await getProperties();
// propertiesStore.filteredProperties = propertiesStore.properties;
// propertiesStore.trendingAreas = await getTrendingAreas();
// propertiesStore.ownedProperties = propertiesStore.properties.slice(0, 3); // mock 3 owned properties
</script>

<style>
::-moz-selection {
   /* Code for Firefox */
   color: white;
   background: #f07e19;
}

::selection {
   color: white;
   background: #f07e19;
}

html {
   scroll-behavior: smooth;
}

/* font Inter for all text */
* {
   font-family: "Inter", sans-serif;
   font-optical-sizing: auto;
   font-style: normal;
   font-variation-settings: "slnt" 0;
}

/* clears the ‘X’ from Internet Explorer */
input[type="search"]::-ms-clear {
   display: none;
   width: 0;
   height: 0;
}
input[type="search"]::-ms-reveal {
   display: none;
   width: 0;
   height: 0;
} /* clears the ‘X’ from Chrome */
input[type="search"]::-webkit-search-decoration,
input[type="search"]::-webkit-search-cancel-button,
input[type="search"]::-webkit-search-results-button,
input[type="search"]::-webkit-search-results-decoration {
   display: none;
}

.z-top {
   z-index: 1000;
}
</style>
