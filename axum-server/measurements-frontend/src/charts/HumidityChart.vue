<script setup lang="ts">
import { computed, ref } from 'vue'
import { Line } from 'vue-chartjs'
import type { ChartData, ChartOptions } from 'chart.js'
import type { ApiError, Measurement } from '../assets.ts'

import ErrorPanel from '../ErrorPanel.vue'
import config from '../config.ts'
import { tension, timeAxis } from './chartOptions.ts'

const properties = defineProps<{
  measurements: Measurement[] | null
  apiError: ApiError | null
  color: string
}>()

function toChartData(measurements: Measurement[]): ChartData<'line'> {
  const data = measurements.map((measurement) => ({
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

const chartData = computed<ChartData<'line'>>(() => {
  return toChartData(properties.measurements || [])
})
</script>

<template>
  <ErrorPanel :error="apiError" v-if="apiError" />
  <Line :options="chartOptions" :data="chartData" v-else />
</template>
