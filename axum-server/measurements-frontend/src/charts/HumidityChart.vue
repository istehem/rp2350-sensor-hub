<script setup lang="ts">
import type { Option } from 'fp-ts/Option'
import * as O from 'fp-ts/Option'
import { pipe } from 'fp-ts/function'
import { computed } from 'vue'
import { Line } from 'vue-chartjs'
import type { ChartData, ChartOptions } from 'chart.js'
import type { ApiError, MeasurementSnapshot, Measurement, Plottable } from '../assets.ts'
import { MeasurementsCodec, MeasurementSnapshotsCodec } from '../assets.ts'

import ErrorPanel from '../ErrorPanel.vue'
import {
  calculateMeasurementAxisMinMax,
  calculateMeasurementAxisMinMax2,
  generateChartOptions,
  tension,
} from './chartOptions.ts'

const properties = defineProps<{
  measurements: Plottable
  apiError: Option<ApiError>
  medianDatasetColor: string
  bandDatasetColor: string
  textColor: string
  gridColor: string
}>()

const title = 'Humidity (%)'

function measurementSnapshotsToChartData(measurements: MeasurementSnapshot[]): ChartData<'line'> {
  const medianData = measurements.map((measurement) => ({
    x: measurement.humidity.date.getTime(),
    y: measurement.humidity.median,
  }))
  const maximumData = measurements.map((measurement) => ({
    x: measurement.humidity.date.getTime(),
    y: measurement.humidity.band.maximum,
  }))
  const minimumData = measurements.map((measurement) => ({
    x: measurement.humidity.date.getTime(),
    y: measurement.humidity.band.minimum,
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

function measurementsToChartData(measurements: Measurement[]): ChartData<'line'> {
  const medianData = measurements.map((measurement) => ({
    x: measurement.date.getTime(),
    y: measurement.humidity,
  }))
  return {
    datasets: [
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

function toChartData(measurements: Plottable): ChartData<'line'> {
  if (MeasurementSnapshotsCodec.is(measurements)) {
    return measurementSnapshotsToChartData(measurements)
  } else if (MeasurementsCodec.is(measurements)) {
    return measurementsToChartData(measurements)
  }
  throw new Error('Not somethig we can plot.')
}

function toMeasurementsChartOptions(measurements: Measurement[]): ChartOptions<'line'> {
  const minMax = calculateMeasurementAxisMinMax2(
    measurements,
    { min: 22, max: 25 },
    (measurement: Measurement) => measurement.humidity,
  )
  return generateChartOptions(title, minMax, 1, {
    textColor: properties.textColor,
    gridColor: properties.gridColor,
  })
}

function toMeasurementSnapshatsChartOptions(
  measurements: MeasurementSnapshot[],
): ChartOptions<'line'> {
  const minMax = calculateMeasurementAxisMinMax(
    measurements,
    { min: 29, max: 32 },
    (measurement: MeasurementSnapshot) => measurement.humidity.median,
  )
  return generateChartOptions(title, minMax, 1, {
    textColor: properties.textColor,
    gridColor: properties.gridColor,
  })
}

const chartData = computed<ChartData<'line'>>(() => toChartData(properties.measurements))

const chartOptions = computed<ChartOptions<'line'>>(() => {
  if (MeasurementSnapshotsCodec.is(properties.measurements)) {
    return toMeasurementSnapshatsChartOptions(properties.measurements)
  }
  if (MeasurementsCodec.is(properties.measurements)) {
    return toMeasurementsChartOptions(properties.measurements)
  }
  throw new Error('Not somethig we can plot.')
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
