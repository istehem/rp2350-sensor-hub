<script setup lang="ts">
import { computed } from 'vue'
import { Line } from 'vue-chartjs'
import type { ChartData, ChartOptions } from 'chart.js'
import { Chart } from 'chart.js'
import type { ApiError, Measurement } from '../assets.ts'

import ErrorPanel from '../ErrorPanel.vue'
import { tension, timeAxis } from './chartOptions.ts'

const properties = defineProps<{
  measurements: Measurement[] | null
  apiError: ApiError | null
  color: string
  textColor: string
  gridColor: string
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

const chartData = computed<ChartData<'line'>>(() => {
  return toChartData(properties.measurements || [])
})

const chartOptions = computed<ChartOptions<'line'>>(() => {
  return {
    responsive: true,
    maintainAspectRatio: false,
    scales: {
      x: timeAxis(properties.textColor, properties.gridColor),
      y: {
        title: {
          color: properties.textColor,
          display: true,
          text: 'Humidity (%)',
        },
        min: 0,
        max: 100,
        ticks: { color: properties.textColor },
        grid: {
          color: properties.gridColor,
        },
      },
    },
    plugins: {
      legend: { labels: { color: properties.textColor } },
    },
  }
})
</script>

<template>
  <ErrorPanel :error="apiError" v-if="apiError" />
  <Line :options="chartOptions" :data="chartData" v-else />
</template>
