<script setup lang="ts">
import { ref, watch } from 'vue'
import { Line } from 'vue-chartjs'
import type { ChartData, ChartOptions } from 'chart.js'
import type { ApiError, Measurement } from '../assets.ts'

import ErrorPanel from '../ErrorPanel.vue'
import config from '../config.ts'
import { downsampleMeasurements } from '../utils.ts'
import { tension, timeAxis } from './chartOptions.ts'

const properties = defineProps<{
  measurements: Measurement[] | null
  apiError: ApiError | null
  color: string
}>()

function toChartData(measurements: Measurement[]): ChartData<'line'> {
  const data = downsampleMeasurements(measurements, 50).map((measurement) => ({
    x: measurement.date.getTime(),
    y: measurement.humidity,
  }))
  return {
    datasets: [
      {
        label: 'Humidity (%)',
        data: data,
        borderColor: properties.color,
        backgroundColor: properties.color,
        tension: tension,
      },
    ],
  }
}

const apiError = ref<ApiError | null>(null)
const chartData = ref<ChartData<'line'>>(toChartData([]))

const chartOptions = ref<ChartOptions<'line'>>({
  responsive: true,
  scales: {
    x: timeAxis,
    y: {
      title: {
        display: true,
        text: 'Humidity (%)',
      },
      min: 0,
      max: 100,
    },
  },
})

watch(
  () => properties.measurements,
  async (newMeasurements) => {
    chartData.value = toChartData(properties.measurements || [])
  },
  { deep: true },
)

watch(
  () => properties.apiError,
  async (newMeasurements) => {
    apiError.value = properties.apiError
  },
  { deep: true },
)
</script>

<template>
  <ErrorPanel :error="apiError" v-if="apiError" />
  <Line :options="chartOptions" :data="chartData" v-else />
</template>
