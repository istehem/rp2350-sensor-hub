<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { Line } from 'vue-chartjs'
import type { ChartData, ChartOptions } from 'chart.js'
import type { ApiError, Measurement } from '../assets.ts'
import config from '../config.ts'

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

function toMeasurement(data: any): Measurement {
  return {
    ...data,
    date: new Date(data.date),
  }
}

function getErrorMessage(error: unknown): string {
  if (error instanceof Error) return error.message
  return String(error)
}

onMounted(async () => {
  const fetchMeasurements = async () => {
    try {
      const response = await fetch(`${config.apiHost}/api/measurements`)
      if (response.ok) {
        const data = await response.json()
        const charData = toChartData(data.map(toMeasurement))
        chartData.value = charData
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
  <div class="center-align error-container" v-if="apiError">
    <h6>{{ apiError.message }}</h6>
  </div>
  <Line :options="chartOptions" :data="chartData" v-else />
</template>
