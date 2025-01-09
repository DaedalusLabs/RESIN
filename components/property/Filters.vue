<template>
  <div class="property-filters">
    <button @click="toggleFilters" class="toggle-filters">
      {{ $t('property.overview.filters.toggle') }}
    </button>

    <div v-if="showFilters" class="filters-panel">
      <div class="filter-group">
        <label>{{ $t('property.overview.filters.price.label') }}</label>
        <div class="range-inputs">
          <input 
            type="number" 
            :placeholder="$t('property.overview.filters.price.min')"
            v-model="filters.minPrice"
          />
          <input 
            type="number" 
            :placeholder="$t('property.overview.filters.price.max')"
            v-model="filters.maxPrice"
          />
        </div>
      </div>

      <div class="filter-group">
        <label>{{ $t('property.overview.filters.size.label') }}</label>
        <div class="range-inputs">
          <input 
            type="number" 
            :placeholder="$t('property.overview.filters.size.min')"
            v-model="filters.minSize"
          />
          <input 
            type="number" 
            :placeholder="$t('property.overview.filters.size.max')"
            v-model="filters.maxSize"
          />
        </div>
      </div>

      <div class="filter-group">
        <label>{{ $t('property.overview.filters.bedrooms.label') }}</label>
        <div class="range-inputs">
          <input 
            type="number" 
            :placeholder="$t('property.overview.filters.bedrooms.min')"
            v-model="filters.minBedrooms"
          />
          <input 
            type="number" 
            :placeholder="$t('property.overview.filters.bedrooms.max')"
            v-model="filters.maxBedrooms"
          />
        </div>
      </div>

      <div class="filter-group">
        <label>{{ $t('property.overview.filters.bathrooms.label') }}</label>
        <div class="range-inputs">
          <input 
            type="number" 
            :placeholder="$t('property.overview.filters.bathrooms.min')"
            v-model="filters.minBathrooms"
          />
          <input 
            type="number" 
            :placeholder="$t('property.overview.filters.bathrooms.max')"
            v-model="filters.maxBathrooms"
          />
        </div>
      </div>

      <div class="filter-group">
        <label>{{ $t('property.overview.filters.propertyType.label') }}</label>
        <select v-model="filters.propertyType">
          <option value="house">{{ $t('property.overview.filters.propertyType.house') }}</option>
          <option value="apartment">{{ $t('property.overview.filters.propertyType.apartment') }}</option>
          <option value="commercial">{{ $t('property.overview.filters.propertyType.commercial') }}</option>
        </select>
      </div>

      <div class="filter-actions">
        <button @click="applyFilters" class="apply-filters">
          {{ $t('property.overview.filters.apply') }}
        </button>
        <button @click="clearFilters" class="clear-filters">
          {{ $t('property.overview.filters.clear') }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const showFilters = ref(false)
const filters = ref({
  minPrice: '',
  maxPrice: '',
  minSize: '',
  maxSize: '',
  minBedrooms: '',
  maxBedrooms: '',
  minBathrooms: '',
  maxBathrooms: '',
  propertyType: ''
})

const emit = defineEmits(['update:filters'])

function toggleFilters() {
  showFilters.value = !showFilters.value
}

function applyFilters() {
  emit('update:filters', filters.value)
}

function clearFilters() {
  filters.value = {
    minPrice: '',
    maxPrice: '',
    minSize: '',
    maxSize: '',
    minBedrooms: '',
    maxBedrooms: '',
    minBathrooms: '',
    maxBathrooms: '',
    propertyType: ''
  }
  emit('update:filters', filters.value)
}
</script>

<style scoped>
.property-filters {
  margin: 1rem 0;
}

.toggle-filters {
  background: #2c3e50;
  color: white;
  border: none;
  padding: 0.5rem 1rem;
  border-radius: 4px;
  cursor: pointer;
}

.filters-panel {
  background: #34495e;
  padding: 1rem;
  border-radius: 4px;
  margin-top: 1rem;
}

.filter-group {
  margin-bottom: 1rem;
}

.filter-group label {
  display: block;
  color: white;
  margin-bottom: 0.5rem;
}

.range-inputs {
  display: flex;
  gap: 1rem;
}

.range-inputs input {
  flex: 1;
  padding: 0.5rem;
  border: 1px solid #ccc;
  border-radius: 4px;
}

select {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid #ccc;
  border-radius: 4px;
}

.filter-actions {
  display: flex;
  gap: 1rem;
  margin-top: 1rem;
}

.apply-filters,
.clear-filters {
  flex: 1;
  padding: 0.5rem;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.apply-filters {
  background: #42b983;
  color: white;
}

.clear-filters {
  background: #e74c3c;
  color: white;
}
</style> 