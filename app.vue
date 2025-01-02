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

const propertiesStore = usePropertiesStore();

async function getProperties() {
   const riversideMansion = {
      id: "7f2c46e7b3d4a8f9c1e5b2d8a4f7c3b6e9d2a5f8c1b4e7a3d6f9c2e5b8a4d7",
      name: "Riverside Mansion",
      location: {
         coordinates: {
            latitude: 5.85015,
            longitude: -55.133602,
         },
         address: {
            street: "Waterkant 23",
            city: "Paramaribo",
            country: "Suriname",
         },
      },
      images: [
         "/images/mock/house.png",
         "/images/mock/house.png",
         "/images/mock/house.png",
      ],
      pricingDetails: {
         rentPerMonth: 2500, // In USD
         propertyPrice: 750000, // In USD
      },
      propertyDetails: {
         size: {
            houseSizeM2: 350, // Size of the house in square meters
            lotSizeM2: 600, // Total lot size in square meters
         },
         bedrooms: 5, // Number of bedrooms
         keyFeatures: [
            "Riverside view",
            "Private dock",
            "Large patio",
            "Swimming pool",
            "Solar panels",
         ],
         additionalDetails: {
            type: "Mansion",
            yearBuilt: 2015,
            hasGarden: true,
            numberOfFloors: 2,
            heating: "Central heating",
            cooling: "Central air conditioning",
            parking: "2-car garage",
         },
      },
      message: "Brought to you by RESIN",
      summary:
         "An expansive mansion located by the river, complete with a private dock and a large patio for outdoor entertaining. This luxurious property offers stunning views and the highest standard of modern living.",
      nearbyProperties: [],
      popupHtml: "<h1>Luxury Riverside Mansion</h1>",
      isBitcasaHome: false,
   };

   return [
      {
         id: "1a5b8c2e4d7f3a6b9c2e5d8f4a7b3c6e9d2a5f8c1b4e7a3d6f9c2e5b8a4d",
         name: "Luxury Villa",
         location: {
            coordinates: {
               latitude: 5.852036,
               longitude: -55.154996,
            },
            address: {
               street: "Mahonylaan 5",
               city: "Paramaribo",
               country: "Suriname",
            },
         },
         images: [
            "/images/mock/house.png",
            "/images/mock/gallery1.png",
            "/images/mock/gallery2.png",
            "/images/mock/gallery3.png",
         ],
         pricingDetails: {
            rentPerMonth: 1950,
            propertyPrice: 468000,
         },
         propertyDetails: {
            size: {
               houseSizeM2: 195, // Size of the house in square meters
               lotSizeM2: 327, // Total lot size in square meters
            },
            bedrooms: 3, // Number of bedrooms
            keyFeatures: [
               "Security system",
               "Swimming pool",
               "Home office space",
               "Solar panels",
            ],
            additionalDetails: {
               type: "Single-family residence",
               yearBuilt: 2021,
               hasGarden: true,
               numberOfFloors: 2,
               heating: "Central heating",
               cooling: "Central air conditioning",
               parking: "2-car garage",
            },
         },
         message: "Brought to you by RESIN",
         summary:
            "This exquisite single-family residence, built in 2021, offers a blend of modern design and urban convenience, making it the perfect choice for contemporary living. Step into luxury with this apartment's open-concept layout, which seamlessly integrates the living, dining, and kitchen areas. This exquisite single-family residence, built in 2021, offers a blend of modern design and urban convenience, making it the perfect choice for contemporary living. Step into luxury with this apartment's open-concept layout, which seamlessly integrates the living, dining, and kitchen areas.",
         nearbyProperties: [riversideMansion],
         popupHtml: "<h1>This is Nantes</h1>",
         isBitcasaHome: false,
      },
      {
         id: "2b6c9d3f5e8a4b7c3f6e9d2a5f8c1b4e7a3d6f9c2e5b8a4d7f1a4b7c3e6",
         name: "Modern Bungalow",
         location: {
            coordinates: {
               latitude: 5.866942,
               longitude: -55.167011,
            },
            address: {
               street: "Kleine Combeweg 12",
               city: "Paramaribo",
               country: "Suriname",
            },
         },
         images: ["/images/mock/house.png"],
         pricingDetails: {
            propertyPrice: 350000, // In USD
         },
         propertyDetails: {
            size: {
               houseSizeM2: 180, // Size of the house in square meters
               lotSizeM2: 300, // Total lot size in square meters
            },
            bedrooms: 4, // Number of bedrooms
            keyFeatures: [
               "Large backyard",
               "Fireplace",
               "Covered parking",
               "Energy-efficient windows",
            ],
            additionalDetails: {
               type: "Single-family residence",
               yearBuilt: 2019,
               hasGarden: true,
               numberOfFloors: 1,
               heating: "Central heating",
               cooling: "Air conditioning",
               parking: "1-car garage",
            },
         },
         message: "Brought to you by Bitcasa Homes",
         summary:
            "A modern bungalow built for comfort, located in a quiet neighborhood in Paramaribo. Features a spacious open-concept living area, energy-efficient windows, and a cozy fireplace for a perfect family home.",
         nearbyProperties: [riversideMansion],
         popupHtml: "<h1>Modern Bungalow in Paramaribo</h1>",
         isBitcasaHome: true,
      },
      {
         id: "3c7d0e4f6f9b5c8d4f7e0e3b6f9c2e5b8a4d7f1a4b7c3e6d9f2a5b8c1e4",
         name: "Cozy Cottage",
         location: {
            coordinates: {
               latitude: 5.852036,
               longitude: -55.154996,
            },
            address: {
               street: "Kwattaweg 67",
               city: "Paramaribo",
               country: "Suriname",
            },
         },
         images: [
            "/images/mock/house.png",
            "/images/mock/house.png",
            "/images/mock/house.png",
         ],
         pricingDetails: {
            rentPerMonth: 1100, // In USD
            propertyPrice: 280000, // In USD
         },
         propertyDetails: {
            size: {
               houseSizeM2: 130, // Size of the house in square meters
               lotSizeM2: 250, // Total lot size in square meters
            },
            bedrooms: 2, // Number of bedrooms
            keyFeatures: [
               "Private garden",
               "Renovated kitchen",
               "Covered porch",
            ],
            additionalDetails: {
               type: "Cottage",
               yearBuilt: 2008,
               hasGarden: true,
               numberOfFloors: 1,
               heating: "Electric heating",
               cooling: "Ceiling fans",
               parking: "Street parking",
            },
         },
         message: "Brought to you by RESIN",
         summary:
            "This cozy cottage offers a private garden and a recently renovated kitchen. Located in a quiet area, it’s the perfect getaway for those seeking comfort and tranquility.",
         nearbyProperties: [riversideMansion],
         popupHtml: "<h1>Cozy Cottage in Kwattaweg</h1>",
         isBitcasaHome: false,
      },
      riversideMansion, // Directly use the Riverside Mansion variable
      {
         id: "5e9f2g6h8i1j7e0f5h8i1j4g7h0i3j6g9h2i5j8h1i4j7g0h3i6j9g2h5i8",
         name: "Suburban Family Home",
         location: {
            coordinates: {
               latitude: 5.850903,
               longitude: -55.16325,
            },
            address: {
               street: "Welgedacht A 55",
               city: "Paramaribo",
               country: "Suriname",
            },
         },
         images: [
            "/images/mock/house.png",
            "/images/mock/house.png",
            "/images/mock/house.png",
         ],
         pricingDetails: {
            rentPerMonth: 1600, // In USD
            propertyPrice: 400000, // In USD
         },
         propertyDetails: {
            size: {
               houseSizeM2: 220, // Size of the house in square meters
               lotSizeM2: 400, // Total lot size in square meters
            },
            bedrooms: 4, // Number of bedrooms
            keyFeatures: [
               "Spacious living area",
               "Private garden",
               "Playground nearby",
               "Home office",
            ],
            additionalDetails: {
               type: "Family home",
               yearBuilt: 2010,
               hasGarden: true,
               numberOfFloors: 2,
               heating: "Central heating",
               cooling: "Air conditioning",
               parking: "2-car garage",
            },
         },
         message: "Brought to you by RESIN",
         summary:
            "This suburban family home offers ample space for a growing family, with a large living area, private garden, and a nearby playground. Located in a family-friendly neighborhood in Paramaribo.",
         nearbyProperties: [riversideMansion],
         popupHtml: "<h1>Spacious Suburban Home</h1>",
         isBitcasaHome: false,
      },
   ];
}

async function getTrendingAreas() {
   return [
      {
         id: "1d8e1f5g7h0i3j6k9l2m5n8o1p4q7r0s3t6u9v2w5x8y1z4a7b0c3d6e9",
         name: "Nantes",
         city: "Nantes",
         country: "France",
         image: "/images/mock/area.png",
         price: 1200,
      },
      {
         id: "2e9f2g6h8i1j4k7l0m3n6o9p2q5r8s1t4u7v0w3x6y9z2a5b8c1d4e7f0",
         name: "Bordeaux",
         city: "Bordeaux",
         country: "France",
         image: "/images/mock/area.png",
         price: 1300,
      },
      {
         id: "3f0g3h7i9j2k5l8m1n4o7p0q3r6s9t2u5v8w1x4y7z0a3b6c9d2e5f8g1",
         name: "Lyon",
         city: "Lyon",
         country: "France",
         image: "/images/mock/area.png",
         price: 1100,
      },
      {
         id: "4g1h4i8j0k3l6m9n2o5p8q1r4s7t0u3v6w9x2y5z8a1b4c7d0e3f6g9h2",
         name: "Paris",
         city: "Paris",
         country: "France",
         image: "/images/mock/area.png",
         price: 1500,
      },
   ];
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
