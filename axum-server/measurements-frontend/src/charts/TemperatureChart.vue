<script setup lang="ts">
import { computed } from 'vue'
import { Line } from 'vue-chartjs'
import type { ChartData, ChartOptions } from 'chart.js'
import type { ApiError, Measurement } from '../assets.ts'

import ErrorPanel from '../ErrorPanel.vue'
import { tension, timeAxis } from './chartOptions.ts'

const properties = defineProps<{
  measurements: Measurement[] | null
  apiError: ApiError | null
  color: string
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
        borderColor: properties.color,
        backgroundColor: properties.color,
        tension: tension,
      },
    ],
  }
}

const chartOptions: ChartOptions<'line'> = {
  responsive: true,
  maintainAspectRatio: false,
  scales: {
    x: timeAxis,
    y: {
      title: {
        display: true,
        text: 'Temperature (°C)',
      },
      min: 20,
      max: 25,
    },
  },
}

const chartData = computed<ChartData<'line'>>(() => {
  return toChartData(properties.measurements || [])
})
</script>

<template>
  <ErrorPanel :error="apiError" v-if="apiError" />
  <Line :options="chartOptions" :data="chartData" v-else />
</template>
