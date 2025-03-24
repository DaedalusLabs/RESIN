// #![warn(
//     clippy::all,
//     clippy::missing_errors_doc,
//     clippy::style,
//     clippy::unseparated_literal_suffix,
//     clippy::pedantic,
//     clippy::nursery
// )]

use sha2::Digest;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, PartialOrd, Default)]
pub struct Geopoint {
    pub lat: f64,
    pub lon: f64,
}
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, PartialOrd, Default)]
pub struct ListingPosition {
    pub geopoint: Geopoint,
    pub geocode: String,
    pub full_address: String,
}
impl ListingPosition {
    #[must_use]
    pub fn sha_hash(&self) -> String {
        let mut hasher = sha2::Sha256::new();
        hasher.update(serde_json::to_string(&self).unwrap_or_default());
        format!("{:x}", hasher.finalize())
    }
}
#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, PartialOrd, Default,
)]
pub enum Currency {
    #[default]
    Dollar,
    Euro,
    Bitcoin,
    SurinameseDollar,
}
impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let currency = match self {
            Self::Dollar => "$",
            Self::Euro => "€",
            Self::Bitcoin => "₿",
            Self::SurinameseDollar => "SRD",
        };
        write!(f, "{currency}")
    }
}
impl std::str::FromStr for Currency {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "$" => Ok(Self::Dollar),
            "€" => Ok(Self::Euro),
            "₿" => Ok(Self::Bitcoin),
            "SRD" => Ok(Self::SurinameseDollar),
            _ => Err(()),
        }
    }
}

#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, PartialOrd, Default,
)]
pub enum PropertyType {
    #[default]
    House,
    Apartment,
    Condo,
    Townhouse,
    Villa,
    Land,
    Commercial,
    Other,
}
impl std::fmt::Display for PropertyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let property = match self {
            Self::House => "House",
            Self::Apartment => "Apartment",
            Self::Condo => "Condo",
            Self::Townhouse => "Townhouse",
            Self::Villa => "Villa",
            Self::Land => "Land",
            Self::Commercial => "Commercial",
            Self::Other => "Other",
        };
        write!(f, "{property}")
    }
}
impl std::str::FromStr for PropertyType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "House" => Ok(Self::House),
            "Apartment" => Ok(Self::Apartment),
            "Condo" => Ok(Self::Condo),
            "Townhouse" => Ok(Self::Townhouse),
            "Villa" => Ok(Self::Villa),
            "Land" => Ok(Self::Land),
            "Commercial" => Ok(Self::Commercial),
            "Other" => Ok(Self::Other),
            _ => Err(()),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, PartialOrd, Default)]
pub struct ListingPrice {
    pub price: f64,
    pub currency: Currency,
}
impl std::fmt::Display for ListingPrice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.currency,
            self.price
                .to_string()
                .as_bytes()
                .rchunks(3)
                .rev()
                .map(std::str::from_utf8)
                .collect::<Result<Vec<&str>, _>>()
                .unwrap()
                .join(",")
        )
    }
}
#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, PartialOrd, Default,
)]
pub enum PropertySizeUnit {
    SqFt,
    #[default]
    SqM,
}
impl std::fmt::Display for PropertySizeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let unit = match self {
            Self::SqFt => "ft2",
            Self::SqM => "m2",
        };
        write!(f, "{unit}")
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, PartialOrd, Default)]
pub struct ListingSize {
    pub size: f64,
    pub unit: PropertySizeUnit,
}
impl std::fmt::Display for ListingSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.size, self.unit)
    }
}
#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, PartialOrd, Default,
)]
pub enum ListingAvailability {
    LongTerm,
    ShortTerm,
    #[default]
    PerDirect,
}
impl std::fmt::Display for ListingAvailability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let availability = match self {
            Self::LongTerm => "Long Term",
            Self::ShortTerm => "Short Term",
            Self::PerDirect => "Per Direct",
        };
        write!(f, "{availability}")
    }
}
impl std::str::FromStr for ListingAvailability {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Long Term" => Ok(Self::LongTerm),
            "Short Term" => Ok(Self::ShortTerm),
            "Per Direct" => Ok(Self::PerDirect),
            _ => Err(()),
        }
    }
}
#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, PartialOrd, Default,
)]
pub enum ListingContractType {
    #[default]
    Sale,
    Rent,
    RentToOwn,
}
impl std::fmt::Display for ListingContractType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let contract = match self {
            Self::Sale => "For Sale",
            Self::Rent => "For Rent",
            Self::RentToOwn => "Rent To Own",
        };
        write!(f, "{contract}")
    }
}
impl std::str::FromStr for ListingContractType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "For Sale" => Ok(Self::Sale),
            "For Rent" => Ok(Self::Rent),
            "Rent To Own" => Ok(Self::RentToOwn),
            _ => Err(()),
        }
    }
}
#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, PartialOrd, Default,
)]
pub enum ListingLocation {
    #[default]
    City,
    Suburb,
    Rural,
}
impl std::fmt::Display for ListingLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let location = match self {
            Self::City => "City",
            Self::Suburb => "Suburb",
            Self::Rural => "Rural",
        };
        write!(f, "{location}")
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, PartialOrd)]
pub enum ListingAmenity {
    Pool,
    Gym,
    Parking,
    SecuritySystem,
    Garden,
    Balcony,
    Terrace,
    Elevator,
    AirConditioning,
    Furnished,
    PetFriendly,
    WheelchairAccessible,
    Fireplace,
    Waterfront,
    SolarPanels,
    WaterHeater,
}
impl std::fmt::Display for ListingAmenity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let amenity = match self {
            Self::Pool => "Pool",
            Self::Gym => "Gym",
            Self::Parking => "Parking",
            Self::SecuritySystem => "Security System",
            Self::Garden => "Garden",
            Self::Balcony => "Balcony",
            Self::Terrace => "Terrace",
            Self::Elevator => "Elevator",
            Self::AirConditioning => "Air Conditioning",
            Self::Furnished => "Furnished",
            Self::PetFriendly => "Pet Friendly",
            Self::WheelchairAccessible => "Wheelchair Accessible",
            Self::Fireplace => "Fireplace",
            Self::Waterfront => "Waterfront",
            Self::SolarPanels => "Solar Panels",
            Self::WaterHeater => "Water Heater",
        };
        write!(f, "{amenity}")
    }
}
impl std::str::FromStr for ListingAmenity {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Pool" => Ok(Self::Pool),
            "Gym" => Ok(Self::Gym),
            "Parking" => Ok(Self::Parking),
            "Security System" => Ok(Self::SecuritySystem),
            "Garden" => Ok(Self::Garden),
            "Balcony" => Ok(Self::Balcony),
            "Terrace" => Ok(Self::Terrace),
            "Elevator" => Ok(Self::Elevator),
            "Air Conditioning" => Ok(Self::AirConditioning),
            "Furnished" => Ok(Self::Furnished),
            "Pet Friendly" => Ok(Self::PetFriendly),
            "Wheelchair Accessible" => Ok(Self::WheelchairAccessible),
            "Fireplace" => Ok(Self::Fireplace),
            "Waterfront" => Ok(Self::Waterfront),
            "Solar Panels" => Ok(Self::SolarPanels),
            "Water Heater" => Ok(Self::WaterHeater),
            _ => Err(()),
        }
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, PartialOrd, Default)]
pub struct NostrListing {
    pub title: String,
    pub position: ListingPosition,
    pub price: ListingPrice,
    pub description: String,
    pub property_type: PropertyType,
    pub availability: ListingAvailability,
    pub bedrooms: u8,
    pub bathrooms: u8,
    pub size: ListingSize,
    pub lot_size: Option<ListingSize>,
    pub contract_type: ListingContractType,
    pub location: Option<ListingLocation>,
    pub amenities: Vec<ListingAmenity>,
    pub images: Vec<String>,
}
impl From<NostrListing> for nostro2::note::NostrNote {
    fn from(listing: NostrListing) -> Self {
        let content = serde_json::to_string(&listing).unwrap_or_default();
        let mut note = Self {
            content,
            kind: 30403,
            ..Default::default()
        };
        note.tags
            .add_parameter_tag(listing.position.sha_hash().as_str());
        note
    }
}
impl TryFrom<nostro2::note::NostrNote> for NostrListing {
    type Error = serde_json::Error;
    fn try_from(note: nostro2::note::NostrNote) -> Result<Self, Self::Error> {
        let listing: Self = serde_json::from_str(note.content.as_str())?;
        Ok(listing)
    }
}
impl TryFrom<&nostro2::note::NostrNote> for NostrListing {
    type Error = serde_json::Error;
    fn try_from(note: &nostro2::note::NostrNote) -> Result<Self, Self::Error> {
        let listing: Self = serde_json::from_str(note.content.as_str())?;
        Ok(listing)
    }
}
impl TryFrom<serde_json::Value> for NostrListing {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
impl From<NostrListing> for serde_json::Value {
    fn from(listing: NostrListing) -> Self {
        serde_json::to_value(listing).expect("Failed to serialize NostrListing.")
    }
}
impl From<&NostrListing> for serde_json::Value {
    fn from(listing: &NostrListing) -> Self {
        serde_json::to_value(listing).expect("Failed to serialize NostrListing.")
    }
}
impl TryFrom<&str> for NostrListing {
    type Error = serde_json::Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(s)
    }
}
impl std::fmt::Display for NostrListing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap_or_default())
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct ResinUserProfile {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub image_url: String,
}
impl From<ResinUserProfile> for nostro2::note::NostrNote {
    fn from(profile: ResinUserProfile) -> Self {
        let content = serde_json::to_string(&profile).unwrap_or_default();
        
        Self {
            content,
            kind: 10400,
            ..Default::default()
        }
    }
}
impl TryFrom<nostro2::note::NostrNote> for ResinUserProfile {
    type Error = serde_json::Error;
    fn try_from(note: nostro2::note::NostrNote) -> Result<Self, Self::Error> {
        let profile: Self = serde_json::from_str(note.content.as_str())?;
        Ok(profile)
    }
}
impl TryFrom<&nostro2::note::NostrNote> for ResinUserProfile {
    type Error = serde_json::Error;
    fn try_from(note: &nostro2::note::NostrNote) -> Result<Self, Self::Error> {
        let profile: Self = serde_json::from_str(note.content.as_str())?;
        Ok(profile)
    }
}
impl TryFrom<serde_json::Value> for ResinUserProfile {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
impl From<ResinUserProfile> for serde_json::Value {
    fn from(profile: ResinUserProfile) -> Self {
        serde_json::to_value(profile).expect("Failed to serialize ResinUserProfile.")
    }
}
impl From<&ResinUserProfile> for serde_json::Value {
    fn from(profile: &ResinUserProfile) -> Self {
        serde_json::to_value(profile).expect("Failed to serialize ResinUserProfile.")
    }
}
impl TryFrom<&str> for ResinUserProfile {
    type Error = serde_json::Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(s)
    }
}
impl std::fmt::Display for ResinUserProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap_or_default())
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct ResinFavoriteListings {
    pub favorite_listings: Vec<String>,
}
impl From<ResinFavoriteListings> for nostro2::note::NostrNote {
    fn from(favorites: ResinFavoriteListings) -> Self {
        let content = serde_json::to_string(&favorites).unwrap_or_default();
        
        Self {
            content,
            kind: 10401,
            ..Default::default()
        }
    }
}
impl TryFrom<nostro2::note::NostrNote> for ResinFavoriteListings {
    type Error = serde_json::Error;
    fn try_from(note: nostro2::note::NostrNote) -> Result<Self, Self::Error> {
        let favorites: Self = serde_json::from_str(note.content.as_str())?;
        Ok(favorites)
    }
}
impl TryFrom<&nostro2::note::NostrNote> for ResinFavoriteListings {
    type Error = serde_json::Error;
    fn try_from(note: &nostro2::note::NostrNote) -> Result<Self, Self::Error> {
        let favorites: Self = serde_json::from_str(note.content.as_str())?;
        Ok(favorites)
    }
}
impl TryFrom<serde_json::Value> for ResinFavoriteListings {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
impl From<ResinFavoriteListings> for serde_json::Value {
    fn from(favorites: ResinFavoriteListings) -> Self {
        serde_json::to_value(favorites).expect("Failed to serialize ResinFavoriteListings.")
    }
}
impl From<&ResinFavoriteListings> for serde_json::Value {
    fn from(favorites: &ResinFavoriteListings) -> Self {
        serde_json::to_value(favorites).expect("Failed to serialize ResinFavoriteListings.")
    }
}
impl TryFrom<&str> for ResinFavoriteListings {
    type Error = serde_json::Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(s)
    }
}
impl std::fmt::Display for ResinFavoriteListings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test::wasm_bindgen_test]
    fn sign_nostr_listing() {
        let new_listing = NostrListing {
            title: "New Listing".to_string(),
            position: ListingPosition {
                geopoint: Geopoint { lat: 5.0, lon: 5.0 },
                geocode: "1234".to_string(),
                full_address: "1234 Main St".to_string(),
            },
            price: ListingPrice {
                price: 100000.0,
                currency: Currency::Dollar,
            },
            description: "This is a new listing".to_string(),
            property_type: PropertyType::House,
            availability: ListingAvailability::LongTerm,
            bedrooms: 3,
            bathrooms: 2,
            size: ListingSize {
                size: 100.0,
                unit: PropertySizeUnit::SqM,
            },
            lot_size: Some(ListingSize {
                size: 1000.0,
                unit: PropertySizeUnit::SqM,
            }),
            contract_type: ListingContractType::Sale,
            location: Some(ListingLocation::City),
            amenities: vec![
                ListingAmenity::Pool,
                ListingAmenity::Gym,
                ListingAmenity::Parking,
            ],
            images: vec![],
        };
        let mut nostr_note: nostro2::note::NostrNote = new_listing.into();
        let signer = nostro2_signer::keypair::NostrKeypair::generate(false);
        nostr_note.pubkey = signer.public_key();
        signer.sign_nostr_event(&mut nostr_note);
        wasm_bindgen_test::console_log!("{}", nostr_note);
        assert_eq!(nostr_note.verify(), true);
    }
    const NOTE_TEST: &str = r#"
    {"pubkey":"1d0817a2e98476df0aa6038da514f8b961dfa03e26a92e9384091705759a9851","created_at":1742432261,"kind":30403,"tags":[["d","4d6cac4ca2d60bc562691c9aa4c4dad55ea8aa9bc5a22bd9a3cf07460ee5f843"]],"content":"{\"title\":\"New Listing\",\"position\":{\"geopoint\":{\"lat\":5.0,\"lon\":5.0},\"geocode\":\"1234\",\"full_address\":\"1234 Main St\"},\"price\":{\"price\":100000.0,\"currency\":\"Dollar\"},\"description\":\"This is a new listing\",\"property_type\":\"House\",\"availability\":\"LongTerm\",\"bedrooms\":3,\"bathrooms\":2,\"size\":{\"size\":100.0,\"unit\":\"SqM\"},\"lot_size\":{\"size\":1000.0,\"unit\":\"SqM\"},\"contract_type\":\"Sale\",\"location\":\"City\",\"amenities\":[\"Pool\",\"Gym\",\"Parking\"],\"images\":[]}","id":"75363c56cedc59fbb76b740b79abf4f70d5f8d06377b46904abf17d7fd4d27b6","sig":"76350db29beaf9e159dd6ab7a8630e41f7a2d9b6760fc011047c087f9ee8bd613985fba62a49d925211346d1706aee92020426d7249b02b624e1e67ba592ffd8"}
    "#;
    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn verify_nostr_listing() {
        let nostr_note: nostro2::note::NostrNote = serde_json::from_str(NOTE_TEST).unwrap();
        assert_eq!(nostr_note.verify(), true);
        let listing: NostrListing = nostr_note.try_into().unwrap();
        println!("{}", listing);
        assert_eq!(listing.title, "New Listing");
        assert_eq!(listing.position.geopoint.lat, 5.0);
        assert_eq!(listing.position.geopoint.lon, 5.0);
        assert_eq!(listing.price.price, 100000.0);
    }
    #[test]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn test_hash_is_always_same() {
        let new_listing = NostrListing {
            title: "New Listing".to_string(),
            position: ListingPosition {
                geopoint: Geopoint { lat: 5.0, lon: 5.0 },
                geocode: "1234".to_string(),
                full_address: "1234 Main St".to_string(),
            },
            price: ListingPrice {
                price: 100000.0,
                currency: Currency::Dollar,
            },
            description: "This is a new listing".to_string(),
            property_type: PropertyType::House,
            availability: ListingAvailability::LongTerm,
            bedrooms: 3,
            bathrooms: 2,
            size: ListingSize {
                size: 100.0,
                unit: PropertySizeUnit::SqM,
            },
            lot_size: Some(ListingSize {
                size: 1000.0,
                unit: PropertySizeUnit::SqM,
            }),
            contract_type: ListingContractType::Sale,
            location: Some(ListingLocation::City),
            amenities: vec![
                ListingAmenity::Pool,
                ListingAmenity::Gym,
                ListingAmenity::Parking,
            ],
            images: vec![],
        };
        let hash1 = new_listing.position.sha_hash();
        let hash2 = new_listing.position.sha_hash();
        assert_eq!(hash1, hash2);
    }
}
