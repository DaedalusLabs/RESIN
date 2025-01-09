<template>
   <section class="mx-auto flex w-10/12 flex-col py-20">
      <h1 class="mb-10 text-2xl font-extrabold leading-tight">{{ $t('myResin.title') }}</h1>

      <div class="space-y-10">
         <section class="flex flex-col gap-4">
            <MyResinNavbar :title="$t('myResin.favorites.title')" link="favorites" />

            <div v-if="favorites.length === 0">
               <p class="mt-20 gap-2 font-semibold text-pirate-950">
                  {{ $t('myResin.favorites.noProperties') }}
               </p>
            </div>
            <div class="space-y-4">
               <FavoritesCard v-for="favorite in favorites" :key="favorite.id" :property="favorite"
                  @remove="removeFavorite(favorite.id)" />
            </div>
         </section>
      </div>
   </section>
</template>

<script setup>
import { usePropertiesStore } from "~/stores/properties";

const propertiesStore = usePropertiesStore();
const favorites = computed(() => propertiesStore.favoriteLocations);


onMounted(() => {
});

const removeFavorite = (id) => {
   propertiesStore.toggleFavorite(id);
};

definePageMeta({
   layout: "white",
});
</script>
