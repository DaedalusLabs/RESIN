<template>
   <div
      class="fixed inset-x-0 bottom-0 z-50 border-t border-gray-200 bg-white shadow-[0_-8px_15px_-3px_rgba(0,0,0,0.05)]"
   >
      <div class="mx-auto flex max-w-md items-center justify-between px-6 py-2">
         <NuxtLink
            :to="localePath('home')"
            class="flex flex-col items-center text-gray-500 hover:text-gray-900"
            :class="{ 'text-resin-500': isHomeView }"
         >
            <PhHouseSimple :size="24" weight="regular" class="mb-1" />
         </NuxtLink>
         <NuxtLink
            :to="localePath('properties')"
            class="flex flex-col items-center text-gray-500 hover:text-gray-900"
            :class="{ 'text-resin-500': isListView }"
         >
            <PhMagnifyingGlass :size="24" weight="regular" class="mb-1" />
         </NuxtLink>
         <NuxtLink
            :to="localePath('my-resin')"
            class="flex flex-col items-center text-gray-500 hover:text-gray-900"
            :class="{ 'text-resin-500': isResinView }"
         >
            <NuxtImg :src="resinImageSrc" width="32" height="32" class="mb-1" />
         </NuxtLink>
         <NuxtLink
            :to="localePath('favorites')"
            class="flex flex-col items-center text-gray-500 hover:text-gray-900"
            :class="{ 'text-resin-500': isFavoritesView }"
         >
            <PhHeartStraight :size="24" weight="regular" class="mb-1" />
         </NuxtLink>
         <NuxtLink
            :to="localePath('profile')"
            class="relative flex flex-col items-center text-gray-500 hover:text-gray-900"
            :class="{ 'text-resin-500': isProfileView }"
            @click="handleProfileClick"
         >
            <PhUser :size="24" weight="regular" class="mb-1" />
         </NuxtLink>
      </div>
   </div>
</template>

<script setup>
import {
   PhHouseSimple,
   PhMagnifyingGlass,
   PhHeartStraight,
   PhUser,
} from "@phosphor-icons/vue";

const emit = defineEmits(["toggleMenuBar"]);

const handleProfileClick = () => {
   emit("toggleMenuBar");
};

const route = useRoute();

const isListView = ref(false);
const isHomeView = ref(false);
const isResinView = ref(false);
const isFavoritesView = ref(false);
const isProfileView = ref(false);
watchEffect(() => {
   isListView.value = route.path.includes("properties");
   isHomeView.value = route.path.includes("home");
   isResinView.value = route.path.includes("my-resin");
   isFavoritesView.value = route.path.includes("favorites");
   isProfileView.value = route.path.includes("profile");
});

const resinImageSrc = computed(() => {
   return isResinView.value
      ? "/images/logos/Resin_Hexagon_Orange_Fill.svg"
      : "/images/logos/Resin_Hexagon_Gray_Fill.svg";
});
</script>
