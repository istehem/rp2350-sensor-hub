import * as t from 'io-ts'
import { DateFromISOString } from 'io-ts-types'

export const MeasurementCodec = t.type({
  temperature: t.number,
  humidity: t.number,
  date: DateFromISOString,
})

export type Measurement = t.TypeOf<typeof MeasurementCodec>

const MedianAndBand = t.type({
  median: t.number,
  band: t.type({
    minimum: t.number,
    maximum: t.number,
  }),
  date: DateFromISOString,
})

export const MeasurementSnapshotCodec = t.type({
  temperature: MedianAndBand,
  humidity: MedianAndBand,
})
export type MeasurementSnapshot = t.TypeOf<typeof MeasurementSnapshotCodec>

export const VersionCodec = t.type({
  version: t.string,
})

export type Version = t.TypeOf<typeof VersionCodec>

export const ApiErrorCodec = t.type({
  message: t.string,
})

export type ApiError = t.TypeOf<typeof ApiErrorCodec>

export const MeasurementsCodec = t.array(MeasurementCodec)
export const MeasurementSnapshotsCodec = t.array(MeasurementSnapshotCodec)

export const PlottableCodec = t.union([MeasurementsCodec, MeasurementSnapshotsCodec])
export type Plottable = t.TypeOf<typeof PlottableCodec>

export enum ChartSelectMode {
  MedianAndBand,
  Decimation,
}
