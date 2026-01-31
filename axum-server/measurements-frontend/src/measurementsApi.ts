import type { ApiError, Measurement } from './assets.ts'
import config from './config.ts'
import * as E from 'fp-ts/Either'

function getErrorMessage(error: unknown): string {
  if (error instanceof Error) return error.message
  return String(error)
}

/* eslint-disable '@typescript-eslint/no-explicit-any' */
function toMeasurement(data: any): Measurement {
  return {
    ...data,
    date: new Date(data.date),
  }
}
/* eslint-enable @typescript-eslint/no-explicit-any */

export async function fetchMeasurements(): Promise<E.Either<ApiError, Measurement[]>> {
  try {
    const response = await fetch(
      `${config.apiHost}/api/measurements?downsample=${config.downsample}`,
    )
    if (response.ok) {
      const data = await response.json()
      return E.right(data.map(toMeasurement))
    } else {
      const apiError = await response.json()
      return E.left({ ...apiError })
    }
  } catch (error) {
    const apiError: ApiError = { message: getErrorMessage(error) }
    console.error('Fetch failed:', error)
    return E.left(apiError)
  }
}

export async function fetchLatestMeasurement(): Promise<E.Either<ApiError, Measurement>> {
  try {
    const response = await fetch(`${config.apiHost}/api/measurements/latest`)
    if (response.ok) {
      const data = await response.json()
      return E.right(toMeasurement(data))
    } else {
      const apiError = await response.json()
      return E.left({ ...apiError })
    }
  } catch (error) {
    const apiError: ApiError = { message: getErrorMessage(error) }
    console.error('Fetch failed:', error)
    return E.left(apiError)
  }
}
