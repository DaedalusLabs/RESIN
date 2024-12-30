<template>
    <ais-autocomplete>
        <template v-slot="{ currentRefinement, indices, refine }">
            <div class="relative" ref="searchContainer">
            <div class="flex items-center justify-between gap-2">

                <form @submit.prevent class="flex-grow">
                    <label for="default-search" class="sr-only mb-2 text-sm font-medium text-gray-900 dark:text-white">
                        Search
                    </label>
                    <div class="relative">
                        <div class="pointer-events-none absolute inset-y-0 start-0 flex items-center ps-3">
                            <svg class="h-4 w-4 text-gray-500 dark:text-gray-400" aria-hidden="true"
                                xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 20 20">
                                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round"
                                    stroke-width="2" d="m19 19-4-4m0-7A7 7 0 1 1 1 8a7 7 0 0 1 14 0Z" />
                            </svg>
                        </div>
                        <input id="default-search" :value="currentRefinement" type="search"
                            class="focus:outline-border-pirate-700 block w-full rounded-2xl border border-pirate-700 border-transparent bg-black p-4 pr-7 ps-10 text-sm text-gray-900 text-pirate-50 outline-none focus:border-pirate-700 focus:ring-0"
                            placeholder="City, region, country..." required
                            autocomplete="off"
                            @input="(e) => { refine(e.currentTarget.value); isDropdownOpen = true; }" />
                        <button v-if="query" type="button" class="absolute inset-y-0 end-0 flex items-center pe-3"
                            @click="refine('')">
                            <PhXCircle class="text-pirate-400" :size="18" weight="fill" />
                        </button>
                    </div>

                </form>
            </div>
            <TopBarInstantDropdown
                v-if="isDropdownOpen"
                class="absolute z-10"
                :filtered-suggestions="indices"
                :query="currentRefinement"
                @update:query="updateQuery"
                @close="isDropdownOpen = false"
            />
            </div>
        </template>

    </ais-autocomplete>

</template>

<script setup>
import { ref, watch } from 'vue';
import { PhXCircle } from "@phosphor-icons/vue";
import { useSearchStore } from '~/stores/search';

const searchStore = useSearchStore();
const isDropdownOpen = ref(true);

const props = defineProps({
    query: {
        type: String,
        default: ''
    }
});

// Watch for changes in the searchStore's refinements
watch(() => searchStore.refinements.query, (newQuery) => {
    if (newQuery !== undefined) {
        searchStore.updateRefinements({ query: newQuery });
    }
}, { immediate: true });

const emit = defineEmits(["update:query"]);

function onInput(event) {
    localQuery.value = event.target.value;
    emit("update:query", event);
}

function clearQuery(event) {
    refine('');
}

function updateQuery(newQuery, latitude, longitude) {
    emit('update:query', newQuery, latitude, longitude);
}
</script>
