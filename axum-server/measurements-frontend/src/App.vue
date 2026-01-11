<script setup lang="ts">
import { ref, onMounted } from 'vue'

interface Measurement {
  temperature: number
  humidity: number
  date: string
}

interface ApiError {
  message: string
}

const apiHost = import.meta.env.VITE_MEASUREMENTS_API_HOST || ''
const measurement = ref<Measurement | null>(null)
const apiError = ref<ApiError | null>(null)

function getErrorMessage(error: unknown): string {
  if (error instanceof Error) return error.message
  return String(error)
}

onMounted(async () => {
  try {
    const response = await fetch(`${apiHost}/api/measurements/latest`)
    if (response.ok) {
      measurement.value = await response.json()
    } else {
      apiError.value = await response.json()
    }
  } catch (error) {
    apiError.value = { message: getErrorMessage(error) }
    console.error('Fetch failed:', error)
  }
})
</script>

<template>
  <main class="responsive round">
    <div class="fill center-align" v-if="apiError">
      <h1>{{ apiError.message }}</h1>
    </div>
    <div class="fill center-align" v-else-if="measurement">
      <div class="container center-align">
        <h2>Measured at: {{ new Date(measurement.date).toLocaleString() }}</h2>
        <h2>Temperature: {{ measurement.temperature }}°C</h2>
        <h2>Humidity: {{ measurement.humidity }}%</h2>
      </div>
    </div>
    <div v-else>Loading...</div>
  </main>
</template>

<style scoped></style>
