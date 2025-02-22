<template>
   <div
      class="fixed inset-x-0 bottom-0 z-50 border-t border-pirate-200 bg-white shadow-[0_-8px_15px_-3px_rgba(0,0,0,0.05)]"
   >
      <div
         class="mx-auto flex max-w-md items-center justify-between px-6 py-2 xl:py-4"
      >
         <NuxtLinkLocale
            to="home"
            data-testid="nav-home"
            :title="$t('navigation.home')"
            class="flex flex-col items-center text-pirate-500 hover:text-pirate-900"
            :class="{ 'text-resin-500 hover:text-resin-500': isHomeView }"
         >
            <PhHouseSimple :size="24" weight="regular" class="mb-1" />
            <span class="sr-only">{{ $t("navigation.home") }}</span>
         </NuxtLinkLocale>
         <NuxtLinkLocale
            to="properties"
            data-testid="nav-properties"
            :title="$t('navigation.properties')"
            class="flex flex-col items-center text-pirate-500 hover:text-pirate-900"
            :class="{ 'text-resin-500 hover:text-resin-500': isPropertiesView }"
         >
            <PhMagnifyingGlass :size="24" weight="regular" class="mb-1" />
            <span class="sr-only">{{ $t("navigation.properties") }}</span>
         </NuxtLinkLocale>
         <NuxtLinkLocale
            to="my-resin"
            data-testid="nav-resin"
            :title="$t('navigation.myResin')"
            class="flex flex-col items-center text-pirate-500 hover:text-pirate-900"
            :class="{ 'text-resin-500 hover:text-resin-500': isResinView }"
         >
            <NuxtImg
               :src="resinImageSrc"
               width="32"
               height="32"
               class="mb-1 hover:brightness-50 active:brightness-100"
               :class="{ 'hover:brightness-100': isResinView }"
               :alt="$t('navigation.myResin')"
            />
            <span class="sr-only">{{ $t("navigation.myResin") }}</span>
         </NuxtLinkLocale>
         <NuxtLinkLocale
            to="favorites"
            data-testid="nav-favorites"
            :title="$t('navigation.favorites')"
            class="flex flex-col items-center text-pirate-500 hover:text-pirate-900"
            :class="{ 'text-resin-500 hover:text-resin-500': isFavoritesView }"
         >
            <PhHeartStraight
               :size="24"
               :weight="hasFavorites ? 'fill' : 'regular'"
               class="mb-1"
            />
            <span class="sr-only">{{ $t("navigation.favorites") }}</span>
         </NuxtLinkLocale>
         <NuxtLinkLocale
            to="profile"
            data-testid="nav-profile"
            :title="$t('navigation.profile')"
            class="relative flex cursor-pointer flex-col items-center text-pirate-500 hover:text-pirate-900"
            :class="{ 'text-resin-500 hover:text-resin-500': isProfileView }"
            @click="handleProfileClick"
         >
            <PhUser :size="24" weight="regular" class="mb-1" />
            <span class="sr-only">{{ $t("navigation.profile") }}</span>
            <span
               v-if="hasUnreadMessages"
               class="absolute right-0 top-0 h-2 w-2 rounded-full bg-resin-500 text-sm font-medium"
            ></span>
         </NuxtLinkLocale>
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

const nostrStore = useNostrStore();
const localePath = useLocalePath();
const router = useRouter();
const emit = defineEmits(["toggleMenuBar"]);

const propertiesStore = usePropertiesStore();
const hasFavorites = computed(() => {
   return propertiesStore.favorites.length > 0;
});

const handleProfileClick = () => {
   if (!nostrStore.isAuthenticated) {
      router.push(localePath("introduction"));
      return;
   }
   emit("toggleMenuBar");
};

const route = useRoute();

const isPropertiesView = ref(false);
const isHomeView = ref(false);
const isResinView = ref(false);
const isFavoritesView = ref(false);
const isProfileView = ref(false);

watchEffect(() => {
   const path = route.path;
   isPropertiesView.value =
      path === localePath("properties") || path === localePath("map");
   isHomeView.value = path === localePath("home");
   isResinView.value = path === localePath("my-resin");
   isFavoritesView.value = path === localePath("favorites");
   isProfileView.value = path === localePath("profile");
});

const hasUnreadMessages = computed(() => {
   const count = nostrStore.unreadMessagesCount;
   return count > 0;
});

const resinImageSrc = computed(() => {
   return isResinView.value
      ? "/images/logos/Resin_Hexagon_Orange_Fill.svg"
      : "/images/logos/Resin_Hexagon_Gray_Fill.svg";
});
</script>
