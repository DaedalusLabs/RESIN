<template>
   <FlowbiteDrawer :is-open="showDrawer" class="p-4" @close="handleCloseDrawer">
      <template #title>
         <h3 v-show="!verificationStarted" class="text-xl font-semibold text-pirate-950">
            Let's verify your identity
         </h3>
      </template>
      <template #content>
        <div id="sumsub-websdk-container"></div>

        <section v-show="!verificationStarted" class="text-center">
        <p>You will be redirected to our identity provider to verify your identity.</p>
        <FlowbiteButton
               text="Continue"
               class="mt-3 px-5 py-3"
               @click="launchIdentityProvider"
            />
        </section>

        </template>
   </FlowbiteDrawer>
</template>

<script setup lang="ts">
import snsWebSdk from '@sumsub/websdk';

const showDrawer = ref(true);
const verificationStarted = ref(false);
const runtimeConfig = useRuntimeConfig();
const nostrStore = useNostrStore();

const emit = defineEmits(["close", "next"]);

const props = defineProps({
   show: {
      type: Boolean,
      default: false,
   },
});


watchEffect(() => {
   showDrawer.value = props.show;
});

const handleCloseDrawer = () => {
   emit("close");
};

const handleNext = () => {
   emit("next");
};

const getAccessToken = async() => {
   const message = "verify-id";
   const signedMessage = await nostrStore.signMessage(message);

   const response = await fetch(`${runtimeConfig.public.BACKEND_ENDPOINT}/verification/token`, {
      method: "POST",
      headers: {
         "Content-Type": "application/json",
      },
      body: JSON.stringify({
         signedMessage: signedMessage
      }),
   });

   if (!response.ok) {
      throw new Error("Failed to get access token");
   } 

   const accessToken = (await response.json()).accessToken;
   return accessToken;
};
         
const accessToken = await getAccessToken();


const launchIdentityProvider = () => {
    verificationStarted.value = true;
    
    const snsWebSdkInstance = snsWebSdk.init(
        accessToken,
        () => getAccessToken(),
    ).withConf({
        lang: 'en'
    })
    .on("idCheck.onApplicantStatusChanged", (payload) => {
      if (payload.reviewStatus === 'completed') {
         if (payload.reviewResult.reviewAnswer === 'GREEN') {
            emit("next");
         }
      }
    })
    .build();

    snsWebSdkInstance.launch("#sumsub-websdk-container");
};


</script>
