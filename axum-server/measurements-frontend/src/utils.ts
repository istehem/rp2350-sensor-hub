import type { Measurement } from './assets.ts'

export function getErrorMessage(error: unknown): string {
  if (error instanceof Error) return error.message
  return String(error)
}

export function toMeasurement(data: any): Measurement {
  return {
    _kind: 'Measurement',
    ...data,
    date: new Date(data.date),
  }
}
