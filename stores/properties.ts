import { defineStore } from "pinia";
import TypesenseInstantSearchAdapter, {
   type SearchClient,
} from "typesense-instantsearch-adapter";
import { useNostrStore } from "./nostr";
import { watch } from "vue";

interface Address {
   street: string;
   city: string;
   country: string;
}

interface Location {
   address: Address;
}

interface Property {
   id: string;
   name: string;
   location: Location;
}

interface PropertiesState {
   properties: Property[];
   filteredProperties: Property[];
   favorites: string[];
   searches: string[];
   viewedProperties: string[];
   trendingAreas: string[];
   hasSeenMapToast: boolean;
   recoveryPhrase: string[];
   ownedProperties: Property[];
   searchClient: SearchClient;
   imagesBaseUrl: string;
   apiEndpoint: string;
   typesenseHost: string;
   typesensePort: string;
   typesenseApiKey: string;
   typesenseProtocol: string;
   isInitialized: boolean;
   isLoadingNostr: boolean;
}

export const usePropertiesStore = defineStore("properties", {
   state: (): PropertiesState => ({
      properties: [],
      filteredProperties: [],
      favorites: [],
      searches: [],
      viewedProperties: [],
      trendingAreas: [],
      hasSeenMapToast: false,
      recoveryPhrase: [],
      ownedProperties: [],
      searchClient: undefined!,
      imagesBaseUrl: "",
      apiEndpoint: "",
      typesenseHost: "",
      typesensePort: "",
      typesenseApiKey: "",
      typesenseProtocol: "",
      isInitialized: false,
      isLoadingNostr: false,
   }),
   persist: {
      key: "properties",
      storage: piniaPluginPersistedstate.localStorage(),
      pick: ["hasSeenMapToast", "viewedProperties", "searches"],
   },
   getters: {
      getLocations(): Property[] {
         return this.properties;
      },

      getFilteredLocations(): Property[] {
         return this.filteredProperties;
      },

      areFiltersActive(): boolean {
         return this.properties.length !== this.filteredProperties.length;
      },

      favoriteLocations(): Property[] {
         return this.properties.filter((location) =>
            this.favorites.includes(location.id),
         );
      },

      viewedLocations(): Property[] {
         if (!this.isInitialized) return [];
         // // Perform a multi-ID search using Typesense InstantSearch
         // const viewedIds = this.viewedProperties;
         // if (!viewedIds.length) return [];
         
         // // Create a filter string for multiple IDs using OR conditions
         // const filterString = this.viewedProperties.map(id => `id:=${id}`).join(' || ');
         
         // // Use searchClient to query properties matching the IDs
         // const searchResults = this.searchClient.search([{
         //    indexName: 'nostr_listing',
         //    params: {
         //       filter_by: filterString,
         //       per_page: viewedIds.length
         //    }
         // }]);
         // console.log("searchResults", await searchResults);
         // // Return the filtered properties in order of viewing
         // return [];

         console.log("viewedProperties", this.viewedProperties, this.properties);
         return this.properties.filter((location) =>
            this.viewedProperties.includes(location.id),
         );
      },

      getRecoveredPhrase(): string[] {
         return this.recoveryPhrase;
      },
   },

   actions: {
      init() {
         if (this.isInitialized) return;

         const config = useRuntimeConfig();
         this.imagesBaseUrl = config.public.IMAGES_BASE_URL;
         this.apiEndpoint = config.public.BACKEND_ENDPOINT;
         this.typesenseHost = config.public.TYPESENSE_HOST;
         this.typesensePort = config.public.TYPESENSE_PORT;
         this.typesenseApiKey = config.public.TYPESENSE_API_KEY;
         this.typesenseProtocol = config.public.TYPESENSE_PROTOCOL || "https";
         this.initializeSearch();

         this.isInitialized = true;

      // Listen for initial search results from searchClient
      this.searchClient.search([{
        indexName: 'nostr_listing',
        params: {
          per_page: 1000
        }
      }]).then((results) => {
        console.log("Initial search results loaded:", results.results[0].hits.length);
        this.properties = results.results[0].hits;
      }).catch((err) => {
        console.error("Error loading initial search results:", err);
      });
      },

      async loadNostrPreferences() {
         const nostrStore = useNostrStore();
         if (nostrStore.authenticated) {
            try {
               this.isLoadingNostr = true;
               const event = await nostrStore.loadPreferences("favorites");
               if (event) {
                  this.favorites = event;
               }

               console.log("loading chat store");
               const chatStore = useChatStore();
               await chatStore.init();
            } catch (error) {
               console.error("Failed to load favorites from Nostr:", error);
            } finally {
               this.isLoadingNostr = false;
            }
         }
      },

      watchNostrAuth() {
         const nostrStore = useNostrStore();
         watch(
            () => nostrStore.authenticated,
            async (isAuthenticated) => {
               if (isAuthenticated) {
                  await new Promise((resolve) => setTimeout(resolve, 3000));
                  await this.loadNostrPreferences();
               } else {
                  this.favorites = [];
               }
            },
         );
      },

      async get(id: string) {
         try {
            const response = await fetch(`${this.apiEndpoint}/listings/${id}`);
            if (!response.ok) {
               if (response.status === 404) {
                  throw new Error("Property not found");
               } else if (response.status >= 500) {
                  throw new Error("Server error");
               }
            }
            const data = await response.json();
            return data;
         } catch (error) {
            console.error(error);
            throw error;
         }
      },

      async getBulk(ids: string[]) {
         if (!ids.length) return [];
         
         try {
            // Create a filter string for multiple IDs using OR conditions
            const filterString = ids.map(id => `id:=${id}`).join(' || ');
            
            // Use searchClient to query properties matching the IDs
            const results = await this.searchClient.search([{
               indexName: 'nostr_listing',
               params: {
                  filter_by: filterString,
                  per_page: ids.length
               }
            }]);

            // Get the hits from the results
            const properties = results.results[0].hits;

            // Sort the properties to match the order of the input ids
            return ids.map(id => properties.find(prop => prop.id === id)).filter(Boolean);
         } catch (error) {
            console.error('Error fetching bulk properties:', error);
            return [];
         }
      },

      initializeSearch() {
         console.log("initializing search");
         const typesenseAdapter = new TypesenseInstantSearchAdapter({
            server: {
               apiKey: this.typesenseApiKey,
               nodes: [
                  {
                     host: this.typesenseHost,
                     port: parseInt(this.typesensePort),
                     protocol: this.typesenseProtocol,
                  },
               ],
            },
            geoLocationField: "location.coordinates",
            additionalSearchParameters: {
               query_by: "title,location.street,location.city,location.country",
            },
         });

         this.searchClient = typesenseAdapter.searchClient;
      },
      addOwnedProperty(property: Property): void {
         this.ownedProperties.push(property);
      },

      addViewedProperty(id: string): void {
         if (!this.viewedProperties.includes(id)) {
            this.viewedProperties.push(id);
         }
         if (this.viewedProperties.length > 10) {
            this.viewedProperties = this.viewedProperties.slice(1);
         }
      },

      addSearch(searchTerm: string): void {
         if (!this.searches.includes(searchTerm)) {
            this.searches.push(searchTerm);
         }
         if (this.searches.length > 10) {
            this.searches = this.searches.slice(1);
         }
      },

      setFilteredLocations(filteredProperties: Property[]): void {
         this.filteredProperties = filteredProperties;
      },
      filterLocations(searchTerm: string): void {
         const lowerCaseSearchTerm = searchTerm.toLowerCase();

         this.filteredProperties = this.properties.filter((property) => {
            const combinedFields = [
               property.name,
               property.location.street,
               property.location.city,
               property.location.country,
            ]
               .join(" ")
               .toLowerCase();

            return combinedFields.includes(lowerCaseSearchTerm);
         });
      },

      resetLocations(): void {
         this.filteredProperties = this.properties;
      },

      async toggleFavorite(locationId: string): Promise<void> {
         if (this.favorites.includes(locationId)) {
            this.favorites = this.favorites.filter((id) => id !== locationId);
         } else {
            this.favorites.push(locationId);
         }

         // Save to Nostr if authenticated
         const nostrStore = useNostrStore();
         if (nostrStore.authenticated) {
            try {
               await nostrStore.savePreferences("favorites", this.favorites);
            } catch (error) {
               console.error("Failed to save favorites to Nostr:", error);
            }
         }
      },

      isFavorite(locationId: string): boolean {
         return this.favorites.includes(locationId);
      },

      findPropertyById(id: string): Property | undefined {
         return this.properties.find((property) => property.id === id);
      },
   },
});
