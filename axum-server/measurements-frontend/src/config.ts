export default {
  apiHost: import.meta.env.VITE_MEASUREMENTS_API_HOST || '',
  measurements: {
    downsample: import.meta.env.VITE_DOWNSAMPLE_MEASUREMENTS || 50,
    pollEvery: import.meta.env.VITE_POLL_MEASUREMENTS_EVERY || 60000,
  },
  latestMeasurement: {
    pollEvery: import.meta.env.VITE_POLL_LATEST_MEASUREMENT_EVERY || 10000,
  },
}
