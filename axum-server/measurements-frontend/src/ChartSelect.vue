<script setup lang="ts">
import { ChartSelectMode } from './assets'

const properties = defineProps<{
  name: string
}>()

const emit = defineEmits<{
  chartSelectionChanged: [ChartSelectMode]
}>()

const onMedianBandSelected = async () => {
  console.log('Median And Band')
  emit('chartSelectionChanged', ChartSelectMode.MedianAndBand)
}
const onDecimationSelected = async () => {
  console.log('Decimation')
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
