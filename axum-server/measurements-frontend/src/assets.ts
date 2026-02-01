import * as t from 'io-ts'
import { DateFromISOString } from 'io-ts-types'

export const MeasurementCodec = t.type({
  temperature: t.number,
  humidity: t.number,
  date: DateFromISOString,
})

export type Measurement = t.TypeOf<typeof MeasurementCodec>

export interface ApiError {
  message: string
}

export const unknownError: ApiError = {
  message: 'unknown error',
}
