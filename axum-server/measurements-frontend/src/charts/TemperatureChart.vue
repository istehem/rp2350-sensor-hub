<script setup lang="ts">
import type { Option } from 'fp-ts/Option'
import * as O from 'fp-ts/Option'
import { pipe } from 'fp-ts/function'
import { computed } from 'vue'
import { Line } from 'vue-chartjs'
import type { ChartData, ChartOptions } from 'chart.js'
import type { ApiError, MeasurementSnapshot } from '../assets.ts'

import ErrorPanel from '../ErrorPanel.vue'
import { calculateMeasurementAxisMinMax, generateChartOptions, tension } from './chartOptions.ts'

const properties = defineProps<{
  measurements: MeasurementSnapshot[]
  apiError: Option<ApiError>
  datasetColor: string
  textColor: string
  gridColor: string
}>()

const title = 'Temperature (°C)'

function toChartData(measurements: MeasurementSnapshot[]): ChartData<'line'> {
  const data = measurements.map((measurement) => ({
    x: measurement.temperature.date.getTime(),
    y: measurement.temperature.median,
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

const chartData = computed<ChartData<'line'>>(() => toChartData(properties.measurements))

const chartOptions = computed<ChartOptions<'line'>>(() => {
  const minMax = calculateMeasurementAxisMinMax(
    properties.measurements,
    { min: 22, max: 25 },
    (measurement: MeasurementSnapshot) => measurement.temperature.median,
  )
  return generateChartOptions(title, minMax, 0.5, {
    textColor: properties.textColor,
    gridColor: properties.gridColor,
  })
})

const error = computed(() =>
  pipe(
    properties.apiError,
    O.match(
      () => null,
      (error) => error,
    ),
  ),
)
</script>

<template>
  <ErrorPanel v-if="error" :error="error" />
  <Line v-else :options="chartOptions" :data="chartData" />
</template>
