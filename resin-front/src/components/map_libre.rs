use crate::router::ResinRoute;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::components::Link;

use crate::{components::drawer::BottomDrawer, contexts::provider::ListingDataStore};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ListingMapPreviewProps {
    pub geolocation: resin_core::Geopoint,
}

#[function_component(ListingMapPreview)]
pub fn map_libre_component(props: &ListingMapPreviewProps) -> Html {
    let location = props.geolocation.clone();
    let map_state = use_state(|| None);
    let map_setter = map_state.setter();
    use_effect_with((), move |()| {
        let options = MapLibreOptions {
            container: "map",
            center: [location.lon, location.lat],
            zoom: 14,
            ..Default::default()
        };
        map_setter.set(Some(MapLibre::new(options)));
        || {}
    });

    let location = props.geolocation.clone();
    use_effect_with(map_state, move |map| {
        if let Some(map) = map.as_ref() {
            let _marker = map.add_marker(location.lon, location.lat);
        }
        || {}
    });
    html! {
        <div id="map" class="relative size-full"></div>
    }
}

#[function_component(MapLibreComponent)]
pub fn map_libre_component() -> Html {
    let map_state = use_state(|| None);
    let is_open = use_state(|| false);
    let clicked_marker = use_state(|| None);
    let listing_ctx = use_context::<ListingDataStore>().expect("No context found");
    let map_setter = map_state.setter();
    use_effect_with((), move |()| {
        yew::platform::spawn_local(async move {
            let position = nostr_minions::browser_api::GeolocationPosition::locate()
                .await
                .unwrap_or(nostr_minions::browser_api::GeolocationPosition {
                    coords: nostr_minions::browser_api::GeolocationCoordinates {
                        latitude: 55.29499,
                        longitude: -5.71777,
                        accuracy: 0.0,
                        altitude: None,
                        altitude_accuracy: None,
                        speed: None,
                    },
                    timestamp: 0.0,
                });
            let options = MapLibreOptions {
                container: "map",
                center: [position.coords.longitude, position.coords.latitude],
                zoom: 10,
                ..Default::default()
            };
            map_setter.set(Some(MapLibre::new(options)));
        });
        || {}
    });

    let is_open_setter = is_open.setter();
    let listing_setter = clicked_marker.setter();
    use_effect_with(
        (map_state, listing_ctx.listings()),
        move |(map, listings)| {
            if let Some(map) = map.as_ref() {
                for (listing_id, listing) in listings.iter().cloned() {
                    let is_open_setter = is_open_setter.clone();
                    let marker = map
                        .add_marker(listing.position.geopoint.lat, listing.position.geopoint.lon);
                    let click_callback = Closure::wrap(Box::new(move || {
                        is_open_setter.set(true);
                    }) as Box<dyn FnMut()>);
                    marker
                        .get_element()
                        .add_event_listener_with_callback(
                            "click",
                            click_callback.into_js_value().as_ref().unchecked_ref(),
                        )
                        .expect("Failed to add event listener");
                    listing_setter.set(Some((listing_id, listing)));
                }
            }
            || {}
        },
    );
    html! {
        <>
        <div id="map" class="relative h-screen w-screen"></div>
        <BottomDrawer {is_open}>
            <MapPopup
                listing_id={(*clicked_marker).clone().map(|(id, _)| id).unwrap_or_default()}
                listing={(*clicked_marker).clone().map(|(_, listing)| listing).unwrap_or_default()}
                />
        </BottomDrawer>
        </>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct MapPopupProps {
    pub listing_id: String,
    pub listing: resin_core::NostrListing,
}

#[function_component(MapPopup)]
pub fn map_popup(props: &MapPopupProps) -> Html {
    let MapPopupProps {
        listing_id,
        listing,
    } = props;
    html! {
        <div class="flex flex-col gap-8 p-4 bg-white items-center h-full justify-center">
            <img src="/public/BG_Hexagon_White.svg" class="fixed inset-0 opacity-10" />
            <div class="flex flex-row gap-4 px-4 w-full items-center">
                <div class="aspect-square w-24 h-24 bg-gray-300 rounded-xl overflow-clip">
                    <img class="object-cover w-full h-full" src={listing.images.first().cloned().unwrap_or_default()} />
                </div>
                <div class="flex flex-col gap-1 overflow-clip w-full">
                    <Link<ResinRoute> to={ResinRoute::Listing { listing_id: listing_id.clone() }}>
                    <h2 class="text-lg font-bold text-bitcoin-orange text-nowrap truncate">{&listing.title}</h2>
                    </Link<ResinRoute>>
                    <p class="text-sm leading-tight text-wrap line-clamp-2">{&listing.position.full_address}</p>
                    <p class="font-bold text-nowrap truncate">{
                        format!("{} {}",
                        &listing.price.to_string(),
                        match listing.contract_type {
                            resin_core::ListingContractType::Rent => "per month",
                            _ => "",
                        })
                    }</p>
                </div>
            </div>
            <div class="flex flex-col overflow-clip w-full px-4">
                <p class="text-sm leading-tight text-wrap line-clamp-2">{&listing.description}</p>
            </div>
            <div class="flex flex-col overflow-clip w-full px-4">
                <p class="text-sm leading-tight text-wrap line-clamp-2">{&listing.position.full_address}</p>
            </div>

        </div>
    }
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone, PartialEq)]
    type Map;

    #[wasm_bindgen(constructor, js_namespace = maplibregl)]
    fn new(options: &JsValue) -> Map;

    #[derive(Debug, Clone)]
    type NavigationControl;

    #[wasm_bindgen(constructor, js_namespace = maplibregl)]
    fn new(options: &JsValue) -> NavigationControl;

    #[wasm_bindgen(method, js_name = addControl)]
    fn add_control(this: &Map, event: &NavigationControl);

    #[derive(Debug, Clone)]
    pub type Marker;

    #[wasm_bindgen(constructor, js_namespace = maplibregl)]
    fn new_marker() -> Marker;

    #[wasm_bindgen(method, js_name = setLngLat, catch)]
    fn set_lng_lat(this: &Marker, lng_lat: &JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, js_name = addTo)]
    fn add_to(this: &Marker, map: &Map);

    #[wasm_bindgen(method, js_name = getElement)]
    pub fn get_element(this: &Marker) -> web_sys::Element;

}

#[allow(clippy::struct_excessive_bools)]
pub struct NavigationControlOptions {
    show_compass: bool,
    show_zoom: bool,
    visualize_pitch: bool,
    visualize_roll: bool,
}
impl Default for NavigationControlOptions {
    fn default() -> Self {
        Self {
            show_compass: true,
            show_zoom: true,
            visualize_pitch: false,
            visualize_roll: false,
        }
    }
}
impl From<NavigationControlOptions> for JsValue {
    fn from(options: NavigationControlOptions) -> Self {
        let obj = web_sys::js_sys::Object::new();
        let _ = web_sys::js_sys::Reflect::set(&obj, &"showCompass".into(), &options.show_compass.into());
        let _ = web_sys::js_sys::Reflect::set(&obj, &"showZoom".into(), &options.show_zoom.into());
        let _ = web_sys::js_sys::Reflect::set(
            &obj,
            &"visualizePitch".into(),
            &options.visualize_pitch.into(),
        );
        let _ = web_sys::js_sys::Reflect::set(
            &obj,
            &"visualizeRoll".into(),
            &options.visualize_roll.into(),
        );
        obj.into()
    }
}

pub struct MapLibreOptions {
    attribution_control: bool,
    container: &'static str,
    center: [f64; 2],
    zoom: u8,
    hash: bool,
    style: &'static str,
    zoom_control: bool,
}
impl Default for MapLibreOptions {
    fn default() -> Self {
        Self {
            attribution_control: false,
            container: "",
            center: [-5.71777, 55.29499],
            zoom: 1,
            hash: false,
            style: "https://api.maptiler.com/maps/basic-v2/style.json?key=5ckghkvT18rt9xXc4HxA",
            zoom_control: true,
        }
    }
}
impl From<MapLibreOptions> for JsValue {
    fn from(options: MapLibreOptions) -> Self {
        let obj = web_sys::js_sys::Object::new();
        let _ = web_sys::js_sys::Reflect::set(
            &obj,
            &"attributionControl".into(),
            &options.attribution_control.into(),
        );
        let _ = web_sys::js_sys::Reflect::set(&obj, &"container".into(), &options.container.into());
        let js_array = web_sys::js_sys::Array::new_with_length(2);
        js_array.set(0, options.center[0].into());
        js_array.set(1, options.center[1].into());
        gloo::console::log!("center: {:?}", &js_array);
        let _ = web_sys::js_sys::Reflect::set(&obj, &"center".into(), &js_array.into());
        let _ = web_sys::js_sys::Reflect::set(&obj, &"zoom".into(), &options.zoom.into());
        let _ = web_sys::js_sys::Reflect::set(&obj, &"hash".into(), &options.hash.into());
        let _ = web_sys::js_sys::Reflect::set(&obj, &"style".into(), &options.style.into());
        let _ = web_sys::js_sys::Reflect::set(
            &obj,
            &"zoomControl".into(),
            &options.zoom_control.into(),
        );
        obj.into()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MapLibre {
    map: Map,
}
impl MapLibre {
    #[must_use]
    pub fn new(options: MapLibreOptions) -> Self {
        let map = Map::new(&options.into());
        map.add_control(&NavigationControl::new(
            &NavigationControlOptions::default().into(),
        ));
        Self { map }
    }
    #[must_use]
    pub fn add_marker(&self, lng: f64, lat: f64) -> Marker {
        let marker = Marker::new_marker();
        let array = web_sys::js_sys::Array::new_with_length(2);
        array.set(0, lng.into());
        array.set(1, lat.into());
        if marker.set_lng_lat(&array.into()).is_ok() {
            marker.add_to(&self.map);
        }
        marker
    }
}
