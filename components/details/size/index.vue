<template>
   <div class="mt-6 flex space-x-8">
      <p class="flex items-center gap-1">
         <PhRuler :size="20" class="inline" />
         {{ property.property?.size }}
         {{ $t("property.details.size.squareMeters") }}
      </p>
      <p class="flex items-center gap-1">
         <PhHouseSimple :size="20" class="inline" />
         {{ property.property?.size }}
         {{ $t("property.details.size.squareMeters") }}
      </p>
      <p class="flex items-center gap-1">
         <PhBed :size="20" class="inline" />
         {{ property.property?.bedrooms }}
         {{
            property.property?.bedrooms === 1
               ? $t("property.details.size.bed")
               : $t("property.details.size.beds")
         }}
      </p>
   </div>
</template>

<script setup>
import { PhRuler, PhHouseSimple, PhBed } from "@phosphor-icons/vue";

defineProps({
   property: {
      type: Object,
      required: true,
   },
});
</script>
