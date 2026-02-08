import * as O from 'fp-ts/Option'
import * as S from 'fp-ts/State'

import type { ApiError, Measurement } from './assets.ts'

export interface Colors {
  primary: string
  secondary: string
  surfaceVariant: string
}

export type Mode = 'light' | 'dark'

export interface AppState {
  latestMeasurement: O.Option<Measurement>
  latestMeasurementApiError: O.Option<ApiError>
  measurements: Measurement[]
  measurementsApiError: O.Option<ApiError>
  colors: Colors
  mode: Mode
}

export const initialState: AppState = {
  latestMeasurement: O.none,
  latestMeasurementApiError: O.none,
  measurements: [],
  measurementsApiError: O.none,
  colors: {
    primary: '#cfbcff',
    secondary: '#cbc2db',
    surfaceVariant: '#49454e',
  },
  mode: 'dark',
}

export const setLatestMeasurement = (measurement: O.Option<Measurement>) =>
  S.modify((s: AppState) => ({ ...s, latestMeasurement: measurement }))

export const setLatestMeasurementApiError = (error: O.Option<ApiError>) =>
  S.modify((s: AppState) => ({ ...s, latestMeasurementApiError: error }))

export const setMeasurements = (measurements: Measurement[]) =>
  S.modify((s: AppState) => ({ ...s, measurements: measurements }))

export const setMeasurementsApiError = (error: O.Option<ApiError>) =>
  S.modify((s: AppState) => ({ ...s, measurementsApiError: error }))

export const setColors = (colors: Colors) => S.modify((s: AppState) => ({ ...s, colors: colors }))

export const setMode = (mode: Mode) => S.modify((s: AppState) => ({ ...s, mode: mode }))
