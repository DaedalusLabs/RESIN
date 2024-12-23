export const tsDocToResult = (appConfig, e) => {
    const m = {
        id: e.id,
        name: e.title,
        location: {
            coordinates: {
                latitude: e.location.coordinates[1],
                longitude: e.location.coordinates[0],
            },
            address: {
                street: e.location.street,
                city: e.location.city,
                country: e.location.country
            },
        },
        images: e.images, // .map((i) => { return `https://api.resin.estate/${i}` }),
        pricingDetails: {
            rentPerMonth: e.price, // In USD
            propertyPrice: e.price, // In USD
        },
        price: e.price, 
        propertyDetails: {
            size: {
                houseSizeM2: e.size, // Size of the house in square meters
                lotSizeM2: 600, // Total lot size in square meters
            },
            bedrooms: e.bedrooms, // Number of bedrooms
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
        message: e.attribution,
        summary:
            e.description,
        nearbyProperties: [],
        popupHtml: "<h1>Luxury Riverside Mansion</h1>",
        isBitcasaHome: e["resin-type"] == "Buy Now" ? true : false,
    }

    return m;
}