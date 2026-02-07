import type { ApiError, Measurement } from './assets.ts'
import { ApiErrorCodec, MeasurementCodec } from './assets.ts'
import config from './config.ts'

import * as t from 'io-ts'
import * as E from 'fp-ts/Either'
import * as TE from 'fp-ts/TaskEither'
import { PathReporter } from 'io-ts/lib/PathReporter'
import { pipe } from 'fp-ts/function'

function getErrorMessage(error: unknown): string {
  if (error instanceof Error) return error.message
  return String(error)
}

const toApiError = (errors: t.Errors): ApiError => ({
  message: 'Measurements API validations failed: ' + PathReporter.report(E.left(errors)).join(', '),
})

const handleResponse = (response: Response): TE.TaskEither<ApiError, unknown> =>
  TE.tryCatch(
    () => {
      if (response.ok) {
        return response.json()
      } else {
        return response.json().then((json) =>
          pipe(
            ApiErrorCodec.decode(json),
            E.fold(
              (errors) => Promise.reject<ApiError>(toApiError(errors)),
              (validError) => Promise.reject<ApiError>(validError),
            ),
          ),
        )
      }
    },
    (error): ApiError => error as ApiError,
  )

const MeasurementsCodec = t.array(MeasurementCodec)

export const fetchMeasurements = (): TE.TaskEither<ApiError, Measurement[]> =>
  pipe(
    TE.tryCatch(
      () =>
        fetch(`${config.apiHost}/api/measurements?downsample=${config.measurements.downsample}`),
      (reason): ApiError => ({ message: getErrorMessage(reason) }),
    ),
    TE.chain(handleResponse),
    TE.chain((data) => pipe(MeasurementsCodec.decode(data), E.mapLeft(toApiError), TE.fromEither)),
  )

export const fetchLatestMeasurement = (): TE.TaskEither<ApiError, Measurement> =>
  pipe(
    TE.tryCatch(
      () => fetch(`${config.apiHost}/api/measurements/latest`),
      (reason): ApiError => ({ message: getErrorMessage(reason) }),
    ),
    TE.chain(handleResponse),
    TE.chain((data) => pipe(MeasurementCodec.decode(data), E.mapLeft(toApiError), TE.fromEither)),
  )
