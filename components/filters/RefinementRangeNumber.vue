<template>
    <ais-range-input :attribute="attribute">
      <template v-slot="{ currentRefinement, range, refine }">
        <FiltersRangeInputNumeric 
          :id="`${attribute}Filter`"
          :label="label"
          :range-min="range.min"
          :range-max="range.max"
          :value-min="formatMinValue(currentRefinement.min, range.min)"
          :value-max="formatMaxValue(currentRefinement.max, range.max)"
          :placeholder="placeholder"
          :unit="unit"
          @update:model-start="handleStartUpdate(refine, currentRefinement, range, $event)"
          @update:model-end="handleEndUpdate(refine, currentRefinement, range, $event)"
        />
      </template>
    </ais-range-input>
  </template>
  
  <script setup>
  const props = defineProps({
    attribute: {
      type: String,
      required: true
    },
    label: {
      type: String,
      required: true
    },
    placeholder: {
      type: String,
      default: '0'
    },
    unit: {
      type: String,
      default: ''
    }
  })
  
function formatMinValue(minValue, minRange) {
    return minValue !== null && minValue !== minRange ? minRange : undefined;
};
function formatMaxValue(maxValue, maxRange) {
    return maxValue !== null && maxValue !== maxRange ? maxRange : undefined;
};
  
  const handleStartUpdate = (refine, currentRefinement, range, event) => {
    refine({
      min: event,
      max: formatMaxValue(currentRefinement.max, range.max)
    })
  }
  
  const handleEndUpdate = (refine, currentRefinement, range, event) => {
    refine({
      min: formatMinValue(currentRefinement.min, range.min),
      max: event
    })
  }
  </script>
  