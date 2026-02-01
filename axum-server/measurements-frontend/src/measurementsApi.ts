import type { ApiError, Measurement } from './assets.ts'
import { MeasurementCodec } from './assets.ts'
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
  message:
    'The request returned an invalid measurement: ' +
    PathReporter.report(E.left(errors)).join(', '),
})

const MeasurementsCodec = t.array(MeasurementCodec)

export const fetchMeasurements = (): TE.TaskEither<ApiError, Measurement[]> =>
  pipe(
    TE.tryCatch(
      () => fetch(`${config.apiHost}/api/measurements?downsample=${config.downsample}`),
      (reason): ApiError => ({ message: getErrorMessage(reason) }),
    ),
    TE.chain((response) =>
      TE.tryCatch(
        () => {
          if (!response.ok) throw response
          return response.json()
        },
        (reason): ApiError => ({ message: getErrorMessage(reason) }),
      ),
    ),
    TE.chain((data) => pipe(MeasurementsCodec.decode(data), E.mapLeft(toApiError), TE.fromEither)),
  )

export const fetchLatestMeasurement = (): TE.TaskEither<ApiError, Measurement> =>
  pipe(
    TE.tryCatch(
      () => fetch(`${config.apiHost}/api/measurements/latest`),
      (reason): ApiError => ({ message: getErrorMessage(reason) }),
    ),
    TE.chain((response) =>
      TE.tryCatch(
        () => {
          if (response.ok) {
            return response.json()
          } else {
            return response.json().then((json) => Promise.reject<ApiError>(json))
          }
        },
        (error) => error,
      ),
    ),
    TE.chain((data) => pipe(MeasurementCodec.decode(data), E.mapLeft(toApiError), TE.fromEither)),
  )
