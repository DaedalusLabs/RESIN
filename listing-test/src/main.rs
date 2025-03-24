static USER_KEYS: std::sync::LazyLock<nostro2_signer::keypair::NostrKeypair> = std::sync::LazyLock::new(|| {
    nostro2_signer::keypair::NostrKeypair::try_from("").unwrap()
});
static NOSTR_TEST_LISTING_1: std::sync::LazyLock<NostrListing> =
    std::sync::LazyLock::new(|| NostrListing {
        title: "Sunset Villa".to_string(),
        position: ListingPosition {
            geopoint: Geopoint {
                lat: 34.052235,
                lon: -118.243683,
            },
            geocode: "us90210".to_string(),
            full_address: "123 Sunset Blvd, Los Angeles, CA, USA".to_string(),
        },
        price: ListingPrice {
            price: 1_200_000.0,
            currency: Currency::Dollar,
        },
        description: "A luxurious villa with stunning ocean views and modern amenities."
            .to_string(),
        property_type: PropertyType::House,
        availability: ListingAvailability::LongTerm,
        bedrooms: 4,
        bathrooms: 3,
        size: ListingSize {
            size: 250.0,
            unit: PropertySizeUnit::SqM,
        },
        lot_size: Some(ListingSize {
            size: 1200.0,
            unit: PropertySizeUnit::SqM,
        }),
        contract_type: ListingContractType::Sale,
        location: Some(ListingLocation::City),
        amenities: vec![
            ListingAmenity::Pool,
            ListingAmenity::Gym,
            ListingAmenity::Parking,
        ],
        images: vec![
            "/public/examples/example1.png".to_string(),
            "/public/examples/example2.png".to_string(),
            "/public/examples/example3.png".to_string(),
        ],
    });

static NOSTR_TEST_LISTING_2: std::sync::LazyLock<NostrListing> =
    std::sync::LazyLock::new(|| NostrListing {
        title: "Mountain Chalet".to_string(),
        position: ListingPosition {
            geopoint: Geopoint {
                lat: 47.3769,
                lon: 8.5417,
            },
            geocode: "ch8001".to_string(),
            full_address: "123 Mountain Rd, Zurich, Switzerland".to_string(),
        },
        price: ListingPrice {
            price: 1_500_000.0,
            currency: Currency::Dollar,
        },
        description: "A cozy chalet with breathtaking mountain views and a fireplace.".to_string(),
        property_type: PropertyType::House,
        availability: ListingAvailability::LongTerm,
        bedrooms: 3,
        bathrooms: 2,
        size: ListingSize {
            size: 200.0,
            unit: PropertySizeUnit::SqM,
        },
        lot_size: Some(ListingSize {
            size: 800.0,
            unit: PropertySizeUnit::SqM,
        }),
        contract_type: ListingContractType::Sale,
        location: Some(ListingLocation::City),
        amenities: vec![ListingAmenity::Fireplace, ListingAmenity::Parking],
        images: vec![
            "/public/examples/example4.png".to_string(),
            "/public/examples/example5.png".to_string(),
            "/public/examples/example6.png".to_string(),
        ],
    });

static NOSTR_TEST_LISTING_3: std::sync::LazyLock<NostrListing> =
    std::sync::LazyLock::new(|| NostrListing {
        title: "Mountain Retreat".to_string(),
        position: ListingPosition {
            geopoint: Geopoint {
                lat: 39.113014,
                lon: -106.445534,
            },
            geocode: "us81611".to_string(),
            full_address: "456 Pine Trail, Aspen, CO, USA".to_string(),
        },
        price: ListingPrice {
            price: 850_000.0,
            currency: Currency::Dollar,
        },
        description: "A cozy cabin nestled in the mountains, perfect for nature lovers."
            .to_string(),
        property_type: PropertyType::Villa,
        availability: ListingAvailability::ShortTerm,
        bedrooms: 3,
        bathrooms: 2,
        size: ListingSize {
            size: 180.0,
            unit: PropertySizeUnit::SqM,
        },
        lot_size: Some(ListingSize {
            size: 800.0,
            unit: PropertySizeUnit::SqM,
        }),
        contract_type: ListingContractType::Rent,
        location: Some(ListingLocation::Rural),
        amenities: vec![ListingAmenity::Fireplace, ListingAmenity::Parking],
        images: vec![
            "/public/examples/example7.png".to_string(),
            "/public/examples/example8.png".to_string(),
            "/public/examples/example9.png".to_string(),
        ],
    });
static NOSTR_TEST_LISTING_4: std::sync::LazyLock<NostrListing> =
    std::sync::LazyLock::new(|| NostrListing {
        title: "Urban Loft".to_string(),
        position: ListingPosition {
            geopoint: Geopoint {
                lat: 40.712776,
                lon: -74.005974,
            },
            geocode: "us10001".to_string(),
            full_address: "789 Broadway, New York, NY, USA".to_string(),
        },
        price: ListingPrice {
            price: 2_500_000.0,
            currency: Currency::Dollar,
        },
        description: "A stylish loft in the heart of Manhattan with panoramic city views."
            .to_string(),
        property_type: PropertyType::Apartment,
        availability: ListingAvailability::LongTerm,
        bedrooms: 2,
        bathrooms: 2,
        size: ListingSize {
            size: 120.0,
            unit: PropertySizeUnit::SqM,
        },
        lot_size: None,
        contract_type: ListingContractType::Sale,
        location: Some(ListingLocation::City),
        amenities: vec![ListingAmenity::Elevator, ListingAmenity::Gym],
        images: vec![
            "/public/examples/example10.png".to_string(),
            "/public/examples/example11.png".to_string(),
            "/public/examples/example12.png".to_string(),
        ],
    });
static NOSTR_TEST_LISTING_5: std::sync::LazyLock<NostrListing> =
    std::sync::LazyLock::new(|| NostrListing {
        title: "Beach Bungalow".to_string(),
        position: ListingPosition {
            geopoint: Geopoint {
                lat: 21.306944,
                lon: -157.858337,
            },
            geocode: "us96815".to_string(),
            full_address: "101 Ocean Drive, Honolulu, HI, USA".to_string(),
        },
        price: ListingPrice {
            price: 950_000.0,
            currency: Currency::Dollar,
        },
        description:
            "A charming bungalow steps away from the beach, perfect for a tropical getaway."
                .to_string(),
        property_type: PropertyType::House,
        availability: ListingAvailability::ShortTerm,
        bedrooms: 3,
        bathrooms: 2,
        size: ListingSize {
            size: 150.0,
            unit: PropertySizeUnit::SqM,
        },
        lot_size: Some(ListingSize {
            size: 500.0,
            unit: PropertySizeUnit::SqM,
        }),
        contract_type: ListingContractType::Rent,
        location: Some(ListingLocation::Rural),
        amenities: vec![ListingAmenity::Pool, ListingAmenity::Parking],
        images: vec![
            "/public/examples/example13.png".to_string(),
            "/public/examples/example14.png".to_string(),
            "/public/examples/example15.png".to_string(),
        ],
    });

static NOSTR_LISTINGS: std::sync::LazyLock<Vec<NostrListing>> = std::sync::LazyLock::new(|| {
    vec![
        NOSTR_TEST_LISTING_1.clone(),
        NOSTR_TEST_LISTING_2.clone(),
        NOSTR_TEST_LISTING_3.clone(),
        NOSTR_TEST_LISTING_4.clone(),
        NOSTR_TEST_LISTING_5.clone(),
    ]
});

use nostro2_relay::nostro2::note::NostrNote;
use nostro2_relay::pool::NostrPool;
use resin_core::{
    Currency, Geopoint, ListingAmenity, ListingAvailability, ListingContractType,
    ListingLocation, ListingPosition, ListingPrice, ListingSize, NostrListing, PropertySizeUnit,
    PropertyType,
};

#[tokio::main]
async fn main() {
    let relay_pool =
        NostrPool::new(&["wss://relay.illuminodes.com", "wss://relay.arrakis.lat"]).await;
    for new_listing in NOSTR_LISTINGS.iter() {
        let mut nostr_note: NostrNote = new_listing.clone().into();
        nostr_note.pubkey = USER_KEYS.public_key();
        println!("Signing NostrNote: {}", nostr_note);
        USER_KEYS.sign_nostr_event(&mut nostr_note);
        relay_pool.send(&nostr_note).await.unwrap();
    }
}
