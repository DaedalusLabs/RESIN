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
   // const riversideMansion = {
   //    id: "7f2c46e7b3d4a8f9c1e5b2d8a4f7c3b6e9d2a5f8c1b4e7a3d6f9c2e5b8a4d7",
   //    name: "Riverside Mansion",
   //    location: {
   //       coordinates: {
   //          latitude: 5.85015,
   //          longitude: -55.133602,
   //       },
   //       address: {
   //          street: "Waterkant 23",
   //          city: "Paramaribo",
   //          country: "Suriname",
   //       },
   //    },
   //    images: [
   //       "/images/mock/house.png",
   //       "/images/mock/house.png",
   //       "/images/mock/house.png",
   //    ],
   //    pricingDetails: {
   //       rentPerMonth: 2500, // In USD
   //       propertyPrice: 750000, // In USD
   //    },
   //    propertyDetails: {
   //       size: {
   //          houseSizeM2: 350, // Size of the house in square meters
   //          lotSizeM2: 600, // Total lot size in square meters
   //       },
   //       bedrooms: 5, // Number of bedrooms
   //       keyFeatures: [
   //          "Riverside view",
   //          "Private dock",
   //          "Large patio",
   //          "Swimming pool",
   //          "Solar panels",
   //       ],
   //       additionalDetails: {
   //          type: "Mansion",
   //          yearBuilt: 2015,
   //          hasGarden: true,
   //          numberOfFloors: 2,
   //          heating: "Central heating",
   //          cooling: "Central air conditioning",
   //          parking: "2-car garage",
   //       },
   //    },
   //    message: "Brought to you by RESIN",
   //    summary:
   //       "An expansive mansion located by the river, complete with a private dock and a large patio for outdoor entertaining. This luxurious property offers stunning views and the highest standard of modern living.",
   //    nearbyProperties: [],
   //    popupHtml: "<h1>Luxury Riverside Mansion</h1>",
   //    isBitcasaHome: false,
   // };

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
   //          "This exquisite single-family residence, built in 2021, offers a blend of modern design and urban convenience, making it the perfect choice for contemporary living. Step into luxury with this apartment's open-concept layout, which seamlessly integrates the living, dining, and kitchen areas.",
   //       nearbyProperties: [riversideMansion],
   //       popupHtml: "<h1>This is Nantes</h1>",
   //       isBitcasaHome: false,
   //    },
   //    {
   //       id: "2b6c9d3f5e8a4b7c3f6e9d2a5f8c1b4e7a3d6f9c2e5b8a4d7f1a4b7c3e6",
   //       name: "Modern Bungalow",
   //       location: {
   //          coordinates: {
   //             latitude: 5.866942,
   //             longitude: -55.167011,
   //          },
   //          address: {
   //             street: "Kleine Combeweg 12",
   //             city: "Paramaribo",
   //             country: "Suriname",
   //          },
   //       },
   //       images: ["/images/mock/house.png"],
   //       pricingDetails: {
   //          propertyPrice: 350000, // In USD
   //       },
   //       propertyDetails: {
   //          size: {
   //             houseSizeM2: 180, // Size of the house in square meters
   //             lotSizeM2: 300, // Total lot size in square meters
   //          },
   //          bedrooms: 4, // Number of bedrooms
   //          keyFeatures: [
   //             "Large backyard",
   //             "Fireplace",
   //             "Covered parking",
   //             "Energy-efficient windows",
   //          ],
   //          additionalDetails: {
   //             type: "Single-family residence",
   //             yearBuilt: 2019,
   //             hasGarden: true,
   //             numberOfFloors: 1,
   //             heating: "Central heating",
   //             cooling: "Air conditioning",
   //             parking: "1-car garage",
   //          },
   //       },
   //       message: "Brought to you by Bitcasa Homes",
   //       summary:
   //          "A modern bungalow built for comfort, located in a quiet neighborhood in Paramaribo. Features a spacious open-concept living area, energy-efficient windows, and a cozy fireplace for a perfect family home.",
   //       nearbyProperties: [riversideMansion],
   //       popupHtml: "<h1>Modern Bungalow in Paramaribo</h1>",
   //       isBitcasaHome: true,
   //    },
   //    {
   //       id: "3c7d0e4f6f9b5c8d4f7e0e3b6f9c2e5b8a4d7f1a4b7c3e6d9f2a5b8c1e4",
   //       name: "Cozy Cottage",
   //       location: {
   //          coordinates: {
   //             latitude: 5.852036,
   //             longitude: -55.154996,
   //          },
   //          address: {
   //             street: "Kwattaweg 67",
   //             city: "Paramaribo",
   //             country: "Suriname",
   //          },
   //       },
   //       images: [
   //          "/images/mock/house.png",
   //          "/images/mock/house.png",
   //          "/images/mock/house.png",
   //       ],
   //       pricingDetails: {
   //          rentPerMonth: 1100, // In USD
   //          propertyPrice: 280000, // In USD
   //       },
   //       propertyDetails: {
   //          size: {
   //             houseSizeM2: 130, // Size of the house in square meters
   //             lotSizeM2: 250, // Total lot size in square meters
   //          },
   //          bedrooms: 2, // Number of bedrooms
   //          keyFeatures: [
   //             "Private garden",
   //             "Renovated kitchen",
   //             "Covered porch",
   //          ],
   //          additionalDetails: {
   //             type: "Cottage",
   //             yearBuilt: 2008,
   //             hasGarden: true,
   //             numberOfFloors: 1,
   //             heating: "Electric heating",
   //             cooling: "Ceiling fans",
   //             parking: "Street parking",
   //          },
   //       },
   //       message: "Brought to you by RESIN",
   //       summary:
   //          "This cozy cottage offers a private garden and a recently renovated kitchen. Located in a quiet area, it’s the perfect getaway for those seeking comfort and tranquility.",
   //       nearbyProperties: [riversideMansion],
   //       popupHtml: "<h1>Cozy Cottage in Kwattaweg</h1>",
   //       isBitcasaHome: false,
   //    },
   //    riversideMansion, // Directly use the Riverside Mansion variable
   //    {
   //       id: "5e9f2g6h8i1j7e0f5h8i1j4g7h0i3j6g9h2i5j8h1i4j7g0h3i6j9g2h5i8",
   //       name: "Suburban Family Home",
   //       location: {
   //          coordinates: {
   //             latitude: 5.850903,
   //             longitude: -55.16325,
   //          },
   //          address: {
   //             street: "Welgedacht A 55",
   //             city: "Paramaribo",
   //             country: "Suriname",
   //          },
   //       },
   //       images: [
   //          "/images/mock/house.png",
   //          "/images/mock/house.png",
   //          "/images/mock/house.png",
   //       ],
   //       pricingDetails: {
   //          rentPerMonth: 1600, // In USD
   //          propertyPrice: 400000, // In USD
   //       },
   //       propertyDetails: {
   //          size: {
   //             houseSizeM2: 220, // Size of the house in square meters
   //             lotSizeM2: 400, // Total lot size in square meters
   //          },
   //          bedrooms: 4, // Number of bedrooms
   //          keyFeatures: [
   //             "Spacious living area",
   //             "Private garden",
   //             "Playground nearby",
   //             "Home office",
   //          ],
   //          additionalDetails: {
   //             type: "Family home",
   //             yearBuilt: 2010,
   //             hasGarden: true,
   //             numberOfFloors: 2,
   //             heating: "Central heating",
   //             cooling: "Air conditioning",
   //             parking: "2-car garage",
   //          },
   //       },
   //       message: "Brought to you by RESIN",
   //       summary:
   //          "This suburban family home offers ample space for a growing family, with a large living area, private garden, and a nearby playground. Located in a family-friendly neighborhood in Paramaribo.",
   //       nearbyProperties: [riversideMansion],
   //       popupHtml: "<h1>Spacious Suburban Home</h1>",
   //       isBitcasaHome: false,
   //    },
   // ];
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
