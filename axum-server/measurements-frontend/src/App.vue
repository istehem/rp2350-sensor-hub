<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'

import type { ApiError, Measurement } from './assets.ts'
import TemperatureChart from './charts/TemperatureChart.vue'
import HumidityChart from './charts/HumidityChart.vue'
import ErrorPanel from './ErrorPanel.vue'
import { fetchLatestMeasurement, fetchMeasurements } from './measurementsApi.ts'

const latestMeasurement = ref<Measurement | null>(null)
const measurements = ref<Measurement[] | null>(null)
const measurementsApiError = ref<ApiError | null>(null)
const latestMeasurementApiError = ref<ApiError | null>(null)
const switchModeIcon = ref<string>('dark_mode')
var latestMeasurementIntervalId: number | null = null
var measurementsIntervalId: number | null = null

async function toggleSwitchModeIcon() {
  const mode = await ui('mode')
  if (mode === 'light') {
    switchModeIcon.value = 'dark_mode'
  } else {
    switchModeIcon.value = 'light_mode'
  }
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

onMounted(async () => {
  toggleSwitchModeIcon()

  const updateMeasurements = async () => {
    const measurementsResponse = await fetchMeasurements()
    if (measurementsResponse._kind === 'ApiError') {
      measurementsApiError.value = measurementsResponse
    } else {
      measurements.value = measurementsResponse.measurements
    }
  }

  const updateLatestMeasurement = async () => {
    const measurementResponse = await fetchLatestMeasurement()
    if (measurementResponse._kind === 'ApiError') {
      latestMeasurementApiError.value = measurementResponse
    } else {
      latestMeasurement.value = measurementResponse
    }
  }

  await updateLatestMeasurement()
  latestMeasurementIntervalId = setInterval(updateLatestMeasurement, 10000)

  await updateMeasurements()
  measurementsIntervalId = setInterval(updateMeasurements, 60000)
})

onUnmounted(() => {
  if (latestMeasurementIntervalId) clearInterval(latestMeasurementIntervalId)
  if (measurementsIntervalId) clearInterval(measurementsIntervalId)
})
</script>

<template>
  <header class="transparent">
    <nav>
      <button @click="flipMode" class="circle transparent primary-text">
        <i>{{ switchModeIcon }}</i>
      </button>
      <h6 class="max">Measurements</h6>
    </nav>
  </header>
  <main class="responsive">
    <ErrorPanel :error="latestMeasurementApiError" v-if="latestMeasurementApiError" />
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
      <article>
        <TemperatureChart :measurements="measurements" :apiError="measurementsApiError" />
      </article>
      <article>
        <HumidityChart :measurements="measurements" :apiError="measurementsApiError" />
      </article>
    </div>
    <article class="center-align" v-else>
      <progress class="circle small indeterminate" value="50" max="100"></progress>
    </article>
  </main>
  <footer class="transparent"></footer>
</template>

<style scoped></style>
