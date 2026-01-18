<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'

import config from './config.ts'
import type { ApiError, Measurement } from './assets.ts'
import TemperatureChart from './charts/TemperatureChart.vue'
import { getErrorMessage, toMeasurement } from './utils.ts'

const measurement = ref<Measurement | null>(null)
const apiError = ref<ApiError | null>(null)
const switchModeIcon = ref<string>('dark_mode')
var intervalId: number | null = null

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

  const fetchMeasurement = async () => {
    try {
      const response = await fetch(`${config.apiHost}/api/measurements/latest`)
      if (response.ok) {
        const data = await response.json()
        measurement.value = toMeasurement(data)
        apiError.value = null
      } else {
        apiError.value = await response.json()
      }
    } catch (error) {
      apiError.value = { message: getErrorMessage(error) }
      console.error('Fetch failed:', error)
    }
  }
  fetchMeasurement()
  intervalId = setInterval(fetchMeasurement, 10000)
})

onUnmounted(() => {
  if (intervalId) clearInterval(intervalId)
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
    <article class="center-align error-container" v-if="apiError">
      <h6>{{ apiError.message }}</h6>
    </article>
    <div v-else-if="measurement">
      <article>
        <div class="grid shrink-center">
          <div class="s6 m6 l6">
            <h6>Date:</h6>
          </div>
          <div class="s6 m6 l6">
            <h6>{{ measurement.date.toLocaleDateString() }}</h6>
          </div>
          <div class="s6 m6 l6">
            <h6>Time:</h6>
          </div>
          <div class="s6 m6 l6">
            <h6>{{ measurement.date.toLocaleTimeString() }}</h6>
          </div>
          <div class="s6 m6 l6">
            <h6>Temperature:</h6>
          </div>
          <div class="s6 m6 l6">
            <h6>{{ measurement.temperature.toFixed(1) }}°C</h6>
          </div>
          <div class="s6 m6 l6">
            <h6>Humidity:</h6>
          </div>
          <div class="s6 m6 l6">
            <h6>{{ measurement.humidity.toFixed(1) }}%</h6>
          </div>
        </div>
      </article>
      <article>
        <TemperatureChart />
      </article>
    </div>
    <article class="center-align" v-else>
      <progress class="circle small indeterminate" value="50" max="100"></progress>
    </article>
  </main>
  <footer class="transparent"></footer>
</template>

<style scoped></style>
