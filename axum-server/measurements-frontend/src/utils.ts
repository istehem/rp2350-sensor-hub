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

export function downsampleMeasurements(
  measurements: Measurement[],
  wantedCount: number,
): Measurement[] {
  if (wantedCount >= measurements.length) {
    return measurements
  }
  const picked: Measurement[] = []
  const interval = measurements.length / wantedCount

  for (let i = 0; i < wantedCount; i++) {
    const evenIndex = Math.floor(i * interval + interval / 2)
    picked.push(measurements[evenIndex]!)
  }
  return picked
}
