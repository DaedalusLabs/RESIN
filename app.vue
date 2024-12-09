<template>
   <div>
      <VitePwaManifest />
      <NuxtLayout>
         <NuxtPage />
      </NuxtLayout>
   </div>
</template>

<script setup lang="ts">
import { usePropertiesStore } from "~/stores/properties";

const propertiesStore = usePropertiesStore();
const appConfig = useAppConfig()

async function getProperties() {
   const properties = await fetch(`${appConfig.BACKEND_ENDPOINT}/listings`);
   let ret = await properties.json();

   ret = ret.map((e) => {
      const m = {
         id:  e.id,
         name:  e.title,
         location: {
            coordinates: {
                  latitude: e.location.coordinates[1],
                  longitude: e.location.coordinates[0],
            },
            address: {
                street: e.title,
               city: e.tags.district[0],
               country: e.tags.country ? e.tags.country[0] : "",
            },
         },
         images: e.images.map((i) => { return `${appConfig.BACKEND_ENDPOINT}/${i.url}` }),
         pricingDetails: {
            rentPerMonth: Math.round(e.amount), // In USD
            propertyPrice: Math.round(e.amount), // In USD
         },
         propertyDetails: {
            size: {
               houseSizeM2: e.tags.size[0], // Size of the house in square meters
               lotSizeM2: 600, // Total lot size in square meters
            },
            bedrooms: e.tags.bedrooms[0], // Number of bedrooms
            keyFeatures: [
               // "Riverside view",
               // "Private dock",
               // "Large patio",
               // "Swimming pool",
               // "Solar panels",
            ],
            additionalDetails: {
               // type: "Mansion",
               // yearBuilt: 2015,
               // hasGarden: true,
               // numberOfFloors: 2,
               // heating: "Central heating",
               // cooling: "Central air conditioning",
               // parking: "2-car garage",
            },
         },
         message: "Brought to you by RESIN",
         summary:
            e.content,
         nearbyProperties: [],
         popupHtml: "<h1>Luxury Riverside Mansion</h1>",
         isBitcasaHome: e.tags["resin-type"] == "buy-now" ? true : false,
      }

      return m;
   })

   // console.log(ret);
   return ret;
   //    return ((await properties.json()).map((e) => { 
   //       let ret = {
   //          id: e.id,
   //          name: e.title,
   //          location: {
   //             coordinates: {
   //                latitude: e.location.coordinates[1],
   //                longitude: e.location.coordinates[0],
   //             },
   // re            address: {
   //                street: e.title,
   //                city: e.tags.district[0],
   //                country: "Suriname",
   //             },         },
   //          images: e.images.map((i) => { return `${appConfig.BACKEND_ENDPOINT}/${i.url}` }),         pricingDetails: {
   //             // rentPerMonth: 2500, // In USD
   //             propertyPrice: e.amount, // In USD
   //             currency: e.currency
   //          },
   //          propertyDetails: {
   //             size: {
   //                houseSizeM2: e.tags.size[0], // Size of the house in square meters
   //                lotSizeM2: e.tags.lot_size[0], // Total lot size in square meters
   //             },
   //             bedrooms: e.tags.bedrooms[0], // Number of bedrooms
   //             keyFeatures: [
   //                "Riverside view",
   //                // "Private dock",
   //                // "Large patio",
   //                // "Swimming pool",
   //                // "Solar panels",
   //             ],
   //             additionalDetails: {
   //                type: "Mansion",
   //                yearBuilt: 2015,
   //                hasGarden: true,
   //                numberOfFloors: 2,
   //                heating: "Central heating",
   //                cooling: "Central air conditioning",
   //                parking: "2-car garage",
   //             },
   //          },
   //          summary:
   //             e.content,
   //          nearbyProperties: [],
   //          popupHtml: "<h1>Luxury Riverside Mansion</h1>",
   //       } 
   //       return ret;
   //    }));
}

async function getTrendingAreas() {
   return [
   ]
}

propertiesStore.properties = await getProperties();
propertiesStore.filteredProperties = propertiesStore.properties;
propertiesStore.trendingAreas = await getTrendingAreas();
propertiesStore.ownedProperties = propertiesStore.properties.slice(0, 3); // mock 3 owned properties
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
}

/* clears the ‘X’ from Chrome */
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
