<script setup lang="ts">
import { ref, onMounted } from 'vue'

interface Measurement {
  temperature: number
  humidity: number
  date: string
}

const measurement = ref<Measurement | null>(null)

onMounted(async () => {
  try {
    const res = await fetch('/api/measurements/latest')
    measurement.value = await res.json()
  } catch (err) {
    console.error('Fetch failed:', err)
  }
})
</script>

<template>
  <div v-if="measurement">
    <div>
      <h1>Date: {{ new Date(measurement.date).toLocaleString() }}</h1>
    </div>
    <div>
      <h1>Temperature: {{ measurement.temperature }}°C</h1>
    </div>
    <div>
      <h1>Humidity: {{ measurement.humidity }}%</h1>
    </div>
  </div>
  <div v-else>Loading...</div>
</template>

<style scoped></style>
