<template>
   <div>
      <FlowbiteErrorToast
         :text="$t('introduction.nsec.invalidKey')"
         :show-toast="wrongNsec"
         @close-toast="wrongNsec = false"
      />
      <FlowbiteDrawer :is-open="showDrawer" @close="handleCloseNsecDrawer">
         <template #title>{{ $t("introduction.nsec.title") }}</template>
         <template #content>
            <form action="#" method="POST" @submit.prevent="validateKey">
               <FlowbiteTextInput
                  v-model="nsecKey"
                  :placeholder="$t('introduction.nsec.placeholder')"
                  :error-messages="errorMessages"
               />
               <p v-if="showError" class="mt-1 text-sm text-red-600">
                  {{ errorMessages[0] }}
               </p>
               <FlowbiteButton
                  :text="$t('logIn')"
                  class="mt-4 flex w-full items-center justify-center px-5 py-3"
                  :is-login="true"
                  :show-icon="false"
                  @click="validateKey"
               />
            </form>
         </template>
      </FlowbiteDrawer>
   </div>
</template>

<script setup>
const { t } = useI18n();
const nsecKey = ref("");

const isNsecFilled = computed(() => nsecKey.value.trim() !== "");
const wrongNsec = ref(false);

const errorMessages = ref([]);

const emit = defineEmits(["close"]);

const validateKey = () => {
   if (!isNsecFilled.value) {
      errorMessages.value.push(t("introduction.nsec.error"));
   } else if (nsecKey.value != "123456") {
      wrongNsec.value = true;
      errorMessages.value.push(t("introduction.nsec.invalidKey"));
   } else {
      const localeRoute = useLocaleRoute();
      const route = localeRoute({
         name: "properties",
      });
      if (route) {
         return navigateTo(route.fullPath);
      }
   }
};

const handleCloseNsecDrawer = () => {
   showDrawer.value = false;
   emit("close");
};

const props = defineProps({
   showNsecDrawer: Boolean,
});

watchEffect(() => {
   if (isNsecFilled.value) {
      errorMessages.value = [];
   }
});

const showDrawer = ref(false);

watchEffect(() => {
   showDrawer.value = props.showNsecDrawer;
});
</script>
