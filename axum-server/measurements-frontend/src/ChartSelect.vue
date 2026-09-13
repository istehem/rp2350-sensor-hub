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

const onMedianBandSelected = () => {
  emit('chartSelectionChanged', ChartSelectMode.MedianAndBand)
}
const onDecimationSelected = () => {
  emit('chartSelectionChanged', ChartSelectMode.Decimation)
}

const selected = ref<ChartSelectMode>(properties.initState)
</script>
<template>
  <fieldset>
    <nav>
      <label class="radio">
        <input
          v-model="selected"
          type="radio"
          :name="properties.name"
          :value="ChartSelectMode.MedianAndBand"
          @change="onMedianBandSelected"
        />
        <span>Median And Band</span>
      </label>
      <label class="radio">
        <input
          v-model="selected"
          type="radio"
          :name="properties.name"
          :value="ChartSelectMode.Decimation"
          @change="onDecimationSelected"
        />
        <span>Decimation</span>
      </label>
    </nav>
  </fieldset>
</template>
