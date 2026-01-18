<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { Line } from 'vue-chartjs'
import type { ChartData, ChartOptions } from 'chart.js'
import type { ApiError, Measurement } from '../assets.ts'

import ErrorPanel from '../ErrorPanel.vue'
import config from '../config.ts'
import { getErrorMessage, toMeasurement } from '../utils.ts'

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

onMounted(async () => {
  const fetchMeasurements = async () => {
    try {
      const response = await fetch(`${config.apiHost}/api/measurements`)
      if (response.ok) {
        const data = await response.json()
        const measurementsAsChartData = toChartData(data.map(toMeasurement))
        chartData.value = measurementsAsChartData
        apiError.value = null
      } else {
        apiError.value = await response.json()
      }
    } catch (error) {
      apiError.value = { message: getErrorMessage(error) }
      console.error('Fetch failed:', error)
    }
  }
  await fetchMeasurements()
})
</script>

<template>
  <ErrorPanel :error="apiError" v-if="apiError" />
  <Line :options="chartOptions" :data="chartData" v-else />
</template>
