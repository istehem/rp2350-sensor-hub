import * as t from 'io-ts'
import { DateFromISOString } from 'io-ts-types'

export const MeasurementCodec = t.type({
  temperature: t.number,
  humidity: t.number,
  date: DateFromISOString,
})

export type Measurement = t.TypeOf<typeof MeasurementCodec>

export const VersionCodec = t.type({
  version: t.string,
})

export type Version = t.TypeOf<typeof VersionCodec>

export const ApiErrorCodec = t.type({
  message: t.string,
})

export type ApiError = t.TypeOf<typeof ApiErrorCodec>
