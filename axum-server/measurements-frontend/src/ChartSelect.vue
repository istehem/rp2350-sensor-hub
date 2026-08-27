<script setup lang="ts">
import { ChartSelectMode } from './assets'

const properties = defineProps<{
  name: string
}>()

const emit = defineEmits<{
  chartSelectionChanged: [ChartSelectMode]
}>()

const onMedianBandSelected = () => {
  emit('chartSelectionChanged', ChartSelectMode.MedianAndBand)
}
const onDecimationSelected = () => {
  emit('chartSelectionChanged', ChartSelectMode.Decimation)
}
</script>
<template>
  <fieldset>
    <nav>
      <label class="radio">
        <input type="radio" :name="properties.name" checked @change="onMedianBandSelected" />
        <span>Median And Band</span>
      </label>
      <label class="radio">
        <input type="radio" :name="properties.name" @change="onDecimationSelected" />
        <span>Decimation</span>
      </label>
    </nav>
  </fieldset>
</template>
