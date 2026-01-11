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
  <header class="transparent"></header>
  <main class="responsive">
    <article class="small center-align">
      <div class="center-align" v-if="apiError">
        <h6>{{ apiError.message }}</h6>
      </div>
      <div v-else-if="measurement">
        <h6>Measured at: {{ new Date(measurement.date).toLocaleString() }}</h6>
        <h6>Temperature: {{ measurement.temperature }}°C</h6>
        <h6>Humidity: {{ measurement.humidity }}%</h6>
      </div>
      <div v-else>
        <progress class="circle small indeterminate" value="50" max="100"></progress>
      </div>
    </article>
  </main>
  <footer class="transparent"></footer>
</template>

<style scoped></style>
