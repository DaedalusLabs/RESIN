import { defineStore } from "pinia";

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
   favorites: (string | number)[];
   searches: string[];
   viewedProperties: (string | number)[];
   trendingAreas: string[];
   hasSeenMapToast: boolean;
   recoveryPhrase: string[];
   ownedProperties: Property[];
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
   }),

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
      addOwnedProperty(property: Property): void {
         this.ownedProperties.push(property);
      },

      addViewedProperty(id: string): void {
         if (!this.viewedProperties.includes(id)) {
            this.viewedProperties.push(id);
         }
      },

      addSearch(searchTerm: string): void {
         this.searches.push(searchTerm);
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

      toggleFavorite(locationId: string | number): void {
         if (this.favorites.includes(locationId)) {
            this.favorites = this.favorites.filter((id) => id !== locationId);
         } else {
            this.favorites.push(locationId);
         }
      },

      isFavorite(locationId: string | number): boolean {
         return this.favorites.includes(locationId);
      },

      findPropertyBySearchQuery(searchTerm: string): Property | undefined {
         const [street, city, country] = searchTerm.split(", ");
         return this.properties.find((property) => {
            return (
               property.location.address.street.toLowerCase() ===
                  street.toLowerCase() &&
               property.location.address.city.toLowerCase() ===
                  city.toLowerCase() &&
               property.location.address.country.toLowerCase() ===
                  country.toLowerCase()
            );
         });
      },
   },
});
