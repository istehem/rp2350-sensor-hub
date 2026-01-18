import type { ApiError, Measurement, Measurements } from './assets.ts'
import config from './config.ts'
import { getErrorMessage, toMeasurement } from './utils.ts'

export async function fetchMeasurements(): Promise<Measurements | ApiError> {
  try {
    const response = await fetch(`${config.apiHost}/api/measurements`)
    if (response.ok) {
      const data = await response.json()
      return { _kind: 'Measurements', measurements: data.map(toMeasurement) }
    } else {
      const apiError: ApiError = await response.json()
      return apiError
    }
  } catch (error) {
    const apiError: ApiError = { _kind: 'ApiError', message: getErrorMessage(error) }
    console.error('Fetch failed:', error)
    return apiError
  }
}

export async function fetchLatestMeasurement(): Promise<Measurement | ApiError> {
  try {
    const response = await fetch(`${config.apiHost}/api/measurements/latest`)
    if (response.ok) {
      const data = await response.json()
      return toMeasurement(data)
    } else {
      const apiError: ApiError = await response.json()
      return apiError
    }
  } catch (error) {
    const apiError: ApiError = { _kind: 'ApiError', message: getErrorMessage(error) }
    console.error('Fetch failed:', error)
    return apiError
  }
}
