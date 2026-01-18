<script setup lang="ts">
import { ref, watch } from 'vue'
import { Line } from 'vue-chartjs'
import type { ChartData, ChartOptions } from 'chart.js'
import type { ApiError, Measurement } from '../assets.ts'

import ErrorPanel from '../ErrorPanel.vue'
import config from '../config.ts'
import { getErrorMessage, toMeasurement } from '../utils.ts'

const properties = defineProps<{
  measurements: Measurement[] | null
  apiError: ApiError | null
}>()

function toChartData(measurements: Measurement[]): ChartData<'line'> {
  const data = measurements.map((measurement) => ({
    x: measurement.date.getTime(),
    y: measurement.temperature,
  }))
  return {
    datasets: [
      {
        label: 'Temperature (°C)',
        data: data,
        borderColor: '#42a5f5',
        backgroundColor: 'rgba(66, 165, 245, 0.1)',
        tension: 0.4,
      },
    ],
  }
}

const apiError = ref<ApiError | null>(null)
const chartData = ref<ChartData<'line'>>(toChartData([]))

const chartOptions = ref<ChartOptions<'line'>>({
  responsive: true,
  scales: {
    x: {
      type: 'time' as const,
      time: {
        unit: 'hour' as const,
        tooltipFormat: 'MMM d, HH:mm',
      },
      title: {
        display: true,
        text: 'Time',
      },
    },
    y: {
      title: {
        display: true,
        text: 'Temperature (°C)',
      },
      min: 20,
      max: 25,
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
