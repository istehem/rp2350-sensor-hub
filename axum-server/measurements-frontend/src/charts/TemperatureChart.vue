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
  medianDatasetColor: string
  bandDatasetColor: string
  textColor: string
  gridColor: string
}>()

const title = 'Temperature (°C)'

function toChartData(measurements: MeasurementSnapshot[]): ChartData<'line'> {
  const medianData = measurements.map((measurement) => ({
    x: measurement.temperature.date.getTime(),
    y: measurement.temperature.median,
  }))
  const maximumData = measurements.map((measurement) => ({
    x: measurement.temperature.date.getTime(),
    y: measurement.temperature.band.maximum,
  }))
  const minimumData = measurements.map((measurement) => ({
    x: measurement.temperature.date.getTime(),
    y: measurement.temperature.band.minimum,
  }))
  return {
    datasets: [
      {
        label: 'maximum',
        data: maximumData,
        borderColor: 'transparent',
        backgroundColor: properties.bandDatasetColor,
        tension: tension,
        fill: false,
        pointRadius: 0,
        order: 1,
      },
      {
        label: 'minimum',
        data: minimumData,
        borderColor: 'transparent',
        backgroundColor: properties.bandDatasetColor,
        tension: tension,
        fill: 0,
        pointRadius: 0,
        order: 1,
      },
      {
        label: title,
        data: medianData,
        borderColor: properties.medianDatasetColor,
        backgroundColor: properties.medianDatasetColor,
        tension: tension,
        fill: false,
        order: 0,
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
