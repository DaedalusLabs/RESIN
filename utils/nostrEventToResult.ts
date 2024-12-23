export const nostrEventToResult = (appConfig, e) => {
    const m = {
        id: e.id,
        name: e.title,
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
        images: e.images, //.map((i) => { return `${appConfig.BACKEND_ENDPOINT}/${i.url}` }),
        pricingDetails: {
            rentPerMonth: Math.round(e.price), // In USD
            propertyPrice: Math.round(e.price), // In USD
        },
        price: e.price, 
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
        message: "Brought to you by Bitcasahomes",
        summary:
            e.content,
        nearbyProperties: [],
        popupHtml: "<h1>Luxury Riverside Mansion</h1>",
        isBitcasaHome: e.tags["resin-type"][0] == "Buy Now" ? true : false,
    }

    return m;
}