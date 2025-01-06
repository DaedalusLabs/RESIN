import { defineStore } from "pinia";
import TypesenseInstantSearchAdapter, { type SearchClient } from 'typesense-instantsearch-adapter';
import { useNostrStore } from './nostr';
import { watch } from 'vue';

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
   isInitialized: boolean;
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
      imagesBaseUrl: '',
      apiEndpoint: '',
      typesenseHost: '',
      typesensePort: '',
      typesenseApiKey: '',
      isInitialized: false,
   }),
   persist: true,
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
         return this.properties.filter((location) =>
            this.viewedProperties.includes(location.id),
         );
      },

      getRecoveredPhrase(): string[] {
         return this.recoveryPhrase;
      },
   },

   actions: {
      async init() {
         if (this.isInitialized) return;

         const config = useRuntimeConfig();
         this.imagesBaseUrl = config.public.IMAGES_BASE_URL;
         this.apiEndpoint = config.public.BACKEND_ENDPOINT;
         this.typesenseHost = config.public.TYPESENSE_HOST;
         this.typesensePort = config.public.TYPESENSE_PORT;
         this.typesenseApiKey = config.public.TYPESENSE_API_KEY;

         this.isInitialized = true;
      },

      async loadNostrPreferences() {
         const nostrStore = useNostrStore();
         if (nostrStore.authenticated) {
            try {
               const event = await nostrStore.loadPreferences('favorites');
               if (event) {
                  this.favorites = event;
               } 
            } catch (error) {
               console.error('Failed to load favorites from Nostr:', error);
            }
         }
      },

      watchNostrAuth() {
         const nostrStore = useNostrStore();
         watch(() => nostrStore.authenticated, async (isAuthenticated) => {
            if (isAuthenticated) {
               await new Promise(resolve => setTimeout(resolve, 3000));
               await this.loadNostrPreferences();
            } else {
               this.favorites = [];
            }
         });
      },

      async get(id: string) {
         try {
            const response = await fetch(`${this.apiEndpoint}/listings/${id}`);
            if (!response.ok) {
               if (response.status === 404) {
                  throw new Error('Property not found');
               } else if (response.status >= 500) {
                  throw new Error('Server error');
               }
            }
            const data = await response.json();
            return data;
         } catch (error) {
            console.error(error);
            throw error;
         }
      },
      initializeSearch() {
         const typesenseAdapter = new TypesenseInstantSearchAdapter({
            server: {
               apiKey: this.typesenseApiKey,
               nodes: [
                  {
                     host: this.typesenseHost,
                     port: parseInt(this.typesensePort),
                     protocol: this.typesenseHost.includes('localhost') ? 'http' : 'https'
                  }
               ]
            },
            geoLocationField: 'location.coordinates',
            additionalSearchParameters: {
               query_by: 'title,location.street,location.city,location.country'
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
               property.location.address.street,
               property.location.address.city,
               property.location.address.country,
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
               await nostrStore.savePreferences('favorites', this.favorites);
            } catch (error) {
               console.error('Failed to save favorites to Nostr:', error);
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
