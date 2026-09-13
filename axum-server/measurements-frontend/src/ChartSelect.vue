<script setup lang="ts">
import { ChartSelectMode } from './assets'
import { ref } from 'vue'

const properties = defineProps<{
  name: string
  initState: ChartSelectMode
}>()

const emit = defineEmits<{
  chartSelectionChanged: [ChartSelectMode]
}>()

const selected = ref<ChartSelectMode>(properties.initState)
</script>
<template>
  <fieldset>
    <nav @change="emit('chartSelectionChanged', selected)">
      <label class="radio">
        <input
          v-model="selected"
          type="radio"
          :name="properties.name"
          :value="ChartSelectMode.MedianAndBand"
        />
        <span>Median And Band</span>
      </label>
      <label class="radio">
        <input
          v-model="selected"
          type="radio"
          :name="properties.name"
          :value="ChartSelectMode.Decimation"
        />
        <span>Decimation</span>
      </label>
    </nav>
  </fieldset>
</template>
