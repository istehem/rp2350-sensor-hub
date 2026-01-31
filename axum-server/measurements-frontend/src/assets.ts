export interface Measurement {
  temperature: number
  humidity: number
  date: Date
}

export interface ApiError {
  message: string
}

export const unknownError: ApiError = {
  message: 'unknown error',
}
