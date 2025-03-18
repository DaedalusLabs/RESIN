#![warn(
    clippy::all,
    clippy::missing_errors_doc,
    clippy::style,
    clippy::unseparated_literal_suffix,
    clippy::pedantic,
    clippy::nursery
)]

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
        write!(f, "{}", currency)
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
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, PartialOrd, Default)]
pub struct ListingPrice {
    pub price: f64,
    pub currency: Currency,
}
impl std::fmt::Display for ListingPrice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.currency, self.price)
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
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, PartialOrd, Default)]
pub struct ListingSize {
    pub size: f64,
    pub unit: PropertySizeUnit,
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
#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, PartialOrd, Default,
)]
pub enum ListingContractType {
    #[default]
    Sale,
    Rent,
    RentToOwn,
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
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash, PartialOrd)]
pub enum ListingAmenities {
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
    pub amenities: Vec<ListingAmenities>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
                ListingAmenities::Pool,
                ListingAmenities::Gym,
                ListingAmenities::Parking,
            ],
        };
        let mut nostr_note: nostro2::note::NostrNote = new_listing.into();
        let signer = nostro2_signer::keypair::NostrKeypair::generate(false);
        nostr_note.pubkey = signer.public_key();
        signer.sign_nostr_event(&mut nostr_note);
        assert_eq!(nostr_note.verify(), true);
    }
    const NOTE_TEST: &str = r#"
    {"pubkey":"50aad38230446d5c51c450e81462fe1e0f652bb28bf74788208c662141467233","created_at":1742247253,"kind":1,"tags":[["d","4d6cac4ca2d60bc562691c9aa4c4dad55ea8aa9bc5a22bd9a3cf07460ee5f843"]],"content":"{\"title\":\"New Listing\",\"position\":{\"geopoint\":{\"lat\":5.0,\"lon\":5.0},\"geocode\":\"1234\",\"full_address\":\"1234 Main St\"},\"price\":{\"price\":100000.0,\"currency\":\"Dollar\"},\"description\":\"This is a new listing\",\"property_type\":\"House\",\"availability\":\"LongTerm\",\"bedrooms\":3,\"bathrooms\":2,\"size\":{\"size\":100.0,\"unit\":\"SqM\"},\"lot_size\":{\"size\":1000.0,\"unit\":\"SqM\"},\"contract_type\":\"Sale\",\"location\":\"City\",\"amenities\":[\"Pool\",\"Gym\",\"Parking\"]}","id":"6c46135546ae8e8bdb76b8a35128cea58135490afe9b5eb053a4291a6845cc07","sig":"7226b7ff80abbf575869dc9db32843151d41d9490999386bff1ade8388f64adfadfe38c08dbd02a833bdbca98039b27be920052c76577b99b3651c967594825d"}
    "#;
    #[test]
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
}
