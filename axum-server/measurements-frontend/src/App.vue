<script setup lang="ts">
import * as E from 'fp-ts/Either'
import { pipe } from 'fp-ts/function'
import { ref, onMounted, onUnmounted } from 'vue'

import type { ApiError, Measurement } from './assets.ts'
import TemperatureChart from './charts/TemperatureChart.vue'
import HumidityChart from './charts/HumidityChart.vue'
import HumidityD3Chart from './charts/HumidityD3Chart.vue'
import ErrorPanel from './ErrorPanel.vue'
import { fetchLatestMeasurement, fetchMeasurements } from './measurementsApi.ts'

const primaryFallbackColor = '#cfbcff'
const secondaryFallbackColor = '#cbc2db'
const surfaceVariantFallbackColor = '#49454e'

const latestMeasurement = ref<Measurement | null>(null)
const measurements = ref<Measurement[] | null>(null)
const measurementsApiError = ref<ApiError | null>(null)
const latestMeasurementApiError = ref<ApiError | null>(null)
const switchModeIcon = ref<string>('dark_mode')
const primaryColor = ref<string>(primaryFallbackColor)
const secondaryColor = ref<string>(primaryFallbackColor)
const surfaceVariantColor = ref<string>(primaryFallbackColor)

let latestMeasurementTimeoutId: number | null = null
let measurementsTimeoutId: number | null = null

async function getCssColor(color: string, fallbackColor: string): Promise<string> {
  try {
    const theme = await ui('theme')
    const mode = (await ui('mode')) as 'light' | 'dark' | undefined

    if (!theme || typeof theme === 'string') return fallbackColor
    const themeCss = theme[(mode || 'light') as 'light' | 'dark']
    if (!themeCss || typeof themeCss !== 'string') return fallbackColor

    const varRe = new RegExp(`--${color}\\s*:\\s*([^;]+);?`)
    const m = themeCss.match(varRe)
    const raw = m?.[1]?.trim()
    return raw || fallbackColor
  } catch {
    return fallbackColor
  }
}

async function toggleSwitchModeIcon() {
  const mode = await ui('mode')
  if (mode === 'light') {
    switchModeIcon.value = 'dark_mode'
  } else {
    switchModeIcon.value = 'light_mode'
  }
  primaryColor.value = await getCssColor('primary', primaryFallbackColor)
  secondaryColor.value = await getCssColor('secondary', secondaryFallbackColor)
  surfaceVariantColor.value = await getCssColor('surface-variant', surfaceVariantFallbackColor)
}

async function flipMode() {
  const mode = await ui('mode')
  if (mode === 'light') {
    ui('mode', 'dark')
  } else {
    ui('mode', 'light')
  }
  await toggleSwitchModeIcon()
}

async function pollLatestMeasurement() {
  const measurementResponse = await fetchLatestMeasurement()
  pipe(
    measurementResponse,
    E.match(
      (error) => {
        latestMeasurementApiError.value = error
      },
      (success) => {
        latestMeasurementApiError.value = null
        latestMeasurement.value = success
      },
    ),
  )
  latestMeasurementTimeoutId = setTimeout(pollLatestMeasurement, 10000)
}

async function pollMeasurements() {
  const measurementsResponse = await fetchMeasurements()
  pipe(
    measurementsResponse,
    E.match(
      (error) => {
        measurementsApiError.value = error
      },
      (success) => {
        measurementsApiError.value = null
        measurements.value = success
      },
    ),
  )
  measurementsTimeoutId = setTimeout(pollMeasurements, 60000)
}

onMounted(async () => {
  toggleSwitchModeIcon()
  pollLatestMeasurement()
  pollMeasurements()
})

onUnmounted(() => {
  if (latestMeasurementTimeoutId) clearTimeout(latestMeasurementTimeoutId)
  if (measurementsTimeoutId) clearTimeout(measurementsTimeoutId)
})
</script>

<template>
  <header class="fixed">
    <nav>
      <button class="circle transparent primary-text" @click="flipMode">
        <i>{{ switchModeIcon }}</i>
      </button>
      <h6 class="max">Measurements</h6>
    </nav>
  </header>
  <main class="responsive">
    <ErrorPanel v-if="latestMeasurementApiError" :error="latestMeasurementApiError" />
    <div v-else-if="latestMeasurement">
      <article>
        <div class="grid shrink-center">
          <div class="s6 m6 l6">
            <h6>Date:</h6>
          </div>
          <div class="s6 m6 l6">
            <h6>{{ latestMeasurement.date.toLocaleDateString() }}</h6>
          </div>
          <div class="s6 m6 l6">
            <h6>Time:</h6>
          </div>
          <div class="s6 m6 l6">
            <h6>{{ latestMeasurement.date.toLocaleTimeString() }}</h6>
          </div>
          <div class="s6 m6 l6">
            <h6>Temperature:</h6>
          </div>
          <div class="s6 m6 l6">
            <h6>{{ latestMeasurement.temperature.toFixed(1) }}°C</h6>
          </div>
          <div class="s6 m6 l6">
            <h6>Humidity:</h6>
          </div>
          <div class="s6 m6 l6">
            <h6>{{ latestMeasurement.humidity.toFixed(1) }}%</h6>
          </div>
        </div>
      </article>
      <article class="medium">
        <TemperatureChart
          :measurements="measurements"
          :api-error="measurementsApiError"
          :dataset-color="primaryColor"
          :text-color="secondaryColor"
          :grid-color="surfaceVariantColor"
        />
      </article>
      <article class="medium">
        <HumidityChart
          :measurements="measurements"
          :api-error="measurementsApiError"
          :dataset-color="primaryColor"
          :text-color="secondaryColor"
          :grid-color="surfaceVariantColor"
        />
      </article>
      <article class="medium">
        <HumidityD3Chart
          :measurements="measurements"
          :api-error="measurementsApiError"
          :dataset-color="primaryColor"
          :text-color="secondaryColor"
          :grid-color="surfaceVariantColor"
        />
      </article>
    </div>
    <article v-else class="center-align">
      <progress class="circle small indeterminate" value="50" max="100"></progress>
    </article>
  </main>
</template>

<style scoped></style>
