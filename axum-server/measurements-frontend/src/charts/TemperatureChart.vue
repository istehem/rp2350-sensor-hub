<script setup lang="ts">
import { ref } from 'vue'
import { Line } from 'vue-chartjs'
import type { ChartData, ChartOptions } from 'chart.js'

const chartData = ref<ChartData<'line'>>({
  datasets: [
    {
      label: 'Temperature (°C)',
      data: [
        { x: new Date('2023-01-01T06:00:00').getTime(), y: 12 },
        { x: new Date('2023-01-01T12:00:00').getTime(), y: 18 },
        { x: new Date('2023-01-01T18:00:00').getTime(), y: 16 },
      ],
      borderColor: '#42a5f5',
      backgroundColor: 'rgba(66, 165, 245, 0.1)',
    },
  ],
})

const chartOptions = ref<ChartOptions<'line'>>({
  responsive: true,
  scales: {
    x: {
      type: 'time' as const,
      time: {
        unit: 'hour' as const,
        tooltipFormat: 'MMM d, HH:mm',
      },
      title: {
        display: true,
        text: 'Time',
      },
    },
    y: {
      title: {
        display: true,
        text: 'Temperature (°C)',
      },
      min: 0,
      max: 30,
    },
  },
})
</script>

<template>
  <Line :options="chartOptions" :data="chartData" />
</template>
