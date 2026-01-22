import type { ApiError, Measurement, Measurements } from './assets.ts'
import config from './config.ts'

function getErrorMessage(error: unknown): string {
  if (error instanceof Error) return error.message
  return String(error)
}

/* eslint-disable '@typescript-eslint/no-explicit-any' */
function toMeasurement(data: any): Measurement {
  return {
    _kind: 'Measurement',
    ...data,
    date: new Date(data.date),
  }
}
/* eslint-enable @typescript-eslint/no-explicit-any */

export async function fetchMeasurements(): Promise<Measurements | ApiError> {
  try {
    const response = await fetch(
      `${config.apiHost}/api/measurements?downsample=${config.downsample}`,
    )
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
