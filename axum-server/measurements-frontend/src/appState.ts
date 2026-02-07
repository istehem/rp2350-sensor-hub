import * as O from 'fp-ts/Option'

import type { ApiError, Measurement } from './assets.ts'

export interface Colors {
  primary: string
  secondary: string
  surfaceVariant: string
}

export interface AppState {
  latestMeasurement: O.Option<Measurement>
  latestMeasurementApiError: O.Option<ApiError>
  measurements: Measurement[]
  measurementsApiError: O.Option<ApiError>
  colors: Colors
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
}
