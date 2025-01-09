<template>
  <div class="property-sort">
    <label>{{ $t('property.overview.sort.label') }}</label>
    <select v-model="selectedSort" @change="updateSort">
      <option value="newest">{{ $t('property.overview.sort.newest') }}</option>
      <option value="priceAsc">{{ $t('property.overview.sort.priceAsc') }}</option>
      <option value="priceDesc">{{ $t('property.overview.sort.priceDesc') }}</option>
      <option value="sizeAsc">{{ $t('property.overview.sort.sizeAsc') }}</option>
      <option value="sizeDesc">{{ $t('property.overview.sort.sizeDesc') }}</option>
    </select>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const selectedSort = ref('newest')
const emit = defineEmits(['update:sort'])

function updateSort() {
  emit('update:sort', selectedSort.value)
}
</script>

<style scoped>
.property-sort {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin: 1rem 0;
}

label {
  color: white;
}

select {
  padding: 0.5rem;
  border: 1px solid #ccc;
  border-radius: 4px;
  background: white;
}
</style> 