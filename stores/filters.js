import { defineStore } from "pinia";

export const useFiltersStore = defineStore("filters", {
   state: () => ({
      // Range filters
      price: { from: 0, to: 0 },
      livingArea: { min: 0, max: 0 },
      plotArea: { min: 0, max: 0 },
      gardenArea: { min: 0, max: 0 },

      // Number filters
      rooms: 0,
      bedrooms: 0,

      // Single select filters
      rentalPeriod: [],
      availability: [],
      availableSince: [],
      constructionType: [],
      purpose: [],
      sort: [],

      // Multi-select filters
      gardenOrientation: [],
      outdoorSpace: [],
      features: [],
      locations: [],
      garages: [],

      // Filter options (for dropdowns)
      filterOptions: {
         gardenOrientations: ["North", "South", "East", "West"],
         outdoorSpaces: ["Garden", "Balcony", "Terrace"],
         availability: ["Available", "Under negotiation", "Sold"],
         availableSince: [
            "No preference",
            "Today",
            "3 days",
            "5 days",
            "10 days",
            "30 days",
         ],
         sort: [
            "House",
            "Apartment",
            "Parking availability",
            "Construction land",
            "Commercial property",
         ],
         rentalPeriod: ["Short to medium term", "Long term"],
         constructionType: ["Existing construction", "New construction"],
         purpose: ["Vacation home", "Permanent construction"],
         locations: [
            "On the edge of the forest",
            "On a busy road",
            "By the park",
            "In a residential area",
         ],
         garages: [
            "Attached garage",
            "Detached garage",
            "Garage + Carport",
            "Built-in garage",
         ],
         features: [
            "Sustainable energy",
            "Central heating boiler",
            "Swimming pool",
            "Bathtub",
            "Terrace",
            "Garden",
            "Balcony",
         ],
      },
   }),

   getters: {
      activeFilterCount: (state) => {
         let count = 0;

         // Count range filters
         if (state.price.from > 0 || state.price.to > 0) count++;
         if (state.livingArea.min > 0 || state.livingArea.max > 0) count++;
         if (state.plotArea.min > 0 || state.plotArea.max > 0) count++;
         if (state.gardenArea.min > 0 || state.gardenArea.max > 0) count++;

         // Count number filters
         if (state.rooms > 0) count++;
         if (state.bedrooms > 0) count++;

         // Count single select filters
         if (state.rentalPeriod) count++;
         if (state.availability) count++;
         if (state.availableSince) count++;
         if (state.constructionType) count++;
         if (state.purpose) count++;
         if (state.sort) count++;

         // Count multi-select filters
         if (state.gardenOrientation.length > 0) count++;
         if (state.outdoorSpace.length > 0) count++;
         if (state.features.length > 0) count++;
         if (state.locations.length > 0) count++;
         if (state.garages.length > 0) count++;

         return count;
      },
      currentFilters() {
         const { filterOptions, ...filterState } = this.$state;
         return filterState;
      },
   },

   actions: {
      clearAllFilters() {
         // Reset range filters
         this.price = { from: 0, to: 0 };
         this.livingArea = { min: 0, max: 0 };
         this.plotArea = { min: 0, max: 0 };
         this.gardenArea = { min: 0, max: 0 };

         // Reset number filters
         this.rooms = 0;
         this.bedrooms = 0;

         // Reset single select filters
         this.rentalPeriod = [];
         this.availability = [];
         this.availableSince = [];
         this.constructionType = [];
         this.purpose = [];
         this.sort = [];

         // Reset multi-select filters
         this.gardenOrientation = [];
         this.outdoorSpace = [];
         this.features = [];
         this.locations = [];
         this.garages = [];

         console.log("Filters cleared");
      },

      // Specific filter update actions
      updateRangeFilter(filterName, field, value) {
         this[filterName][field] = Number(value);
      },

      updateNumberFilter(filterName, value) {
         this[filterName] = Number(value);
      },

      updateSelectFilter(filterName, value) {
         this[filterName] = value;
      },

      updateCheckboxFilter(filterName, value) {
         this[filterName].push(value);
      },

      // Helper action to get current filter state
      getFilterState() {
         const { filterOptions, ...filterState } = this.$state;
         return filterState;
      },
   },
});
