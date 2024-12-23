import { defineStore } from "pinia";

// Interface definitions
interface RangeFilter {
   from: number;
   to: number;
}

interface FilterOptions {
   gardenOrientations: string[];
   outdoorSpaces: string[];
   availability: string[];
   availableSince: string[];
   sort: string[];
   rentalPeriod: string[];
   constructionType: string[];
   purpose: string[];
   constructionPeriod: string[];
   locations: string[];
   garages: string[];
   features: string[];
   display: string[];
}

interface FiltersState {
   // Range filters
   price: RangeFilter;
   livingArea: RangeFilter;
   plotArea: RangeFilter;
   gardenArea: RangeFilter;

   // Number filters
   rooms: number;
   bedrooms: number;

   // Single select filters
   availableSince: string;

   // Multi-select filters
   rentalPeriod: string[];
   availability: string[];
   constructionType: string[];
   purpose: string[];
   constructionPeriod: string[];
   sort: string[];
   gardenOrientation: string[];
   outdoorSpace: string[];
   features: string[];
   locations: string[];
   garages: string[];
   display: string[];

   // Filter options
   filterOptions: FilterOptions;
}

export const useFiltersStore = defineStore("filters", {
   state: (): FiltersState => ({
      // Range filters
      price: { from: 0, to: Infinity },
      livingArea: { from: 0, to: Infinity },
      plotArea: { from: 0, to: Infinity },
      gardenArea: { from: 0, to: Infinity },

      // Number filters
      rooms: 0,
      bedrooms: 0,

      // Single select filters
      availableSince: "",

      // Multi-select filters
      rentalPeriod: [],
      availability: [],
      constructionType: [],
      purpose: [],
      constructionPeriod: [],
      sort: [],
      gardenOrientation: [],
      outdoorSpace: [],
      features: [],
      locations: [],
      garages: [],
      display: [],

      // Filter options
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
         constructionPeriod: [
            "Before 1906",
            "1906-1930",
            "1931-1944",
            "1945-1959",
            "1960-1970",
            "1971-1980",
            "1981-1990",
            "1991-2000",
            "2001-2010",
            "2011-2020",
         ],
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
         display: ["Houses", "Projects"],
      },
   }),

   getters: {
      activeFilterCount: (state: FiltersState): number => {
         let count = 0;

         // Count range filters
         if (state.price.from > 0 || state.price.to !== Infinity) count++;
         if (state.livingArea.from > 0 || state.livingArea.to !== Infinity)
            count++;
         if (state.plotArea.from > 0 || state.plotArea.to !== Infinity) count++;
         if (state.gardenArea.from > 0 || state.gardenArea.to !== Infinity)
            count++;

         // Count number filters
         if (state.rooms > 0) count++;
         if (state.bedrooms > 0) count++;

         // Count single select filters
         if (state.availableSince) count++;

         // Count multi-select filters
         if (state.gardenOrientation.length > 0) count++;
         if (state.sort.length > 0) count++;
         if (state.constructionType.length > 0) count++;
         if (state.constructionPeriod.length > 0) count++;
         if (state.purpose.length > 0) count++;
         if (state.outdoorSpace.length > 0) count++;
         if (state.features.length > 0) count++;
         if (state.locations.length > 0) count++;
         if (state.garages.length > 0) count++;
         if (state.display.length > 0) count++;
         if (state.rentalPeriod.length > 0) count++;
         if (state.availability.length > 0) count++;

         return count;
      },
      currentFilters(this: FiltersState): Omit<FiltersState, "filterOptions"> {
         const { filterOptions, ...filterState } = this;
         return filterState;
      },
   },

   actions: {
      clearAllFilters(): void {
         // Reset range filters
         this.price = { from: 0, to: Infinity };
         this.livingArea = { from: 0, to: Infinity };
         this.plotArea = { from: 0, to: Infinity };
         this.gardenArea = { from: 0, to: Infinity };

         // Reset number filters
         this.rooms = 0;
         this.bedrooms = 0;

         // Reset single select filters
         this.availableSince = "";

         // Reset multi-select filters
         this.rentalPeriod = [];
         this.availability = [];
         this.constructionType = [];
         this.purpose = [];
         this.constructionPeriod = [];
         this.sort = [];
         this.gardenOrientation = [];
         this.outdoorSpace = [];
         this.features = [];
         this.locations = [];
         this.garages = [];
         this.display = [];
      },

      // Specific filter update actions
      updateRangeFilter(
         filterName: keyof Pick<
            FiltersState,
            "price" | "livingArea" | "plotArea" | "gardenArea"
         >,
         field: keyof RangeFilter,
         value: number,
      ): void {
         this[filterName][field] = Number(value);
      },

      updateNumberFilter(
         filterName: keyof Pick<FiltersState, "rooms" | "bedrooms">,
         value: number,
      ): void {
         this[filterName] = Number(value);
      },

      updateSelectFilter(
         filterName: keyof Pick<FiltersState, "availableSince">,
         value: string,
      ): void {
         this[filterName] = value;
      },

      updateCheckboxFilter(
         filterName: keyof Omit<
            FiltersState,
            | "price"
            | "livingArea"
            | "plotArea"
            | "gardenArea"
            | "rooms"
            | "bedrooms"
            | "availableSince"
            | "filterOptions"
         >,
         value: string,
      ): void {
         if (this[filterName].includes(value)) {
            this[filterName] = this[filterName].filter(
               (item) => item !== value,
            );
         } else {
            this[filterName].push(value);
         }
      },

      // Helper action to get current filter state
      getFilterState(): Omit<FiltersState, "filterOptions"> {
         const { filterOptions, ...filterState } = this;
         return filterState;
      },
   },
});
