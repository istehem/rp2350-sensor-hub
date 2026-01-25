<script setup lang="ts">
import { computed } from 'vue'
import { Line } from 'vue-chartjs'
import type { ChartData, ChartOptions } from 'chart.js'
import type { ApiError, Measurement } from '../assets.ts'

import ErrorPanel from '../ErrorPanel.vue'
import { calculateMeasurementAxisMinMax, generateChartOptions, tension } from './chartOptions.ts'

const properties = defineProps<{
  measurements: Measurement[] | null
  apiError: ApiError | null
  datasetColor: string
  textColor: string
  gridColor: string
}>()

const title = 'Temperature (°C)'

function toChartData(measurements: Measurement[]): ChartData<'line'> {
  const data = measurements.map((measurement) => ({
    x: measurement.date.getTime(),
    y: measurement.temperature,
  }))
  return {
    datasets: [
      {
        label: title,
        data: data,
        borderColor: properties.datasetColor,
        backgroundColor: properties.datasetColor,
        tension: tension,
      },
    ],
  }
}

const chartData = computed<ChartData<'line'>>(() => toChartData(properties.measurements || []))

const chartOptions = computed<ChartOptions<'line'>>(() => {
  const minMax = calculateMeasurementAxisMinMax(
    properties.measurements || [],
    { min: 22, max: 25 },
    (measurement: Measurement) => measurement.temperature,
  )
  return generateChartOptions(title, minMax, 0.5, {
    textColor: properties.textColor,
    gridColor: properties.gridColor,
  })
})
</script>

<template>
  <ErrorPanel v-if="apiError" :error="apiError" />
  <Line v-else :options="chartOptions" :data="chartData" />
</template>
