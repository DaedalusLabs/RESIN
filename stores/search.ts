// stores/searchStore.ts
import { defineStore } from "pinia";

export const useSearchStore = defineStore("search", {
   state: () => ({
      refinements: {},
      searchState: {
         "nostr_listing": {}
      },
   }),
   actions: {
      updateSearchState(newState) {
         this.searchState = newState;
      },
      updateRefinements(newRefinements) {
         this.refinements = newRefinements;
      },
   },
   persist: {
      storage: piniaPluginPersistedstate.localStorage(),
      paths: ["refinements", "searchState"],
   },
});
