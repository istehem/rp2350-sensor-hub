import type { ApiError, Measurement } from './assets.ts'
import { MeasurementCodec } from './assets.ts'
import config from './config.ts'

import * as t from 'io-ts'
import * as E from 'fp-ts/Either'
import * as TE from 'fp-ts/TaskEither'
import { pipe } from 'fp-ts/function'

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

const toApiError = (errors: t.Errors): ApiError => ({
  message: `the request returned an invalid measurement: ${errors}`,
})

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

export const fetchLatestMeasurement = (): TE.TaskEither<ApiError, Measurement> =>
  pipe(
    TE.tryCatch(
      () => fetch(`${config.apiHost}/api/measurements/latest`),
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
    TE.chain((data) => pipe(MeasurementCodec.decode(data), E.mapLeft(toApiError), TE.fromEither)),
  )
