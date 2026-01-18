export interface Measurement {
  _kind: 'Measurement'
  temperature: number
  humidity: number
  date: Date
}

export interface Measurements {
  _kind: 'Measurements'
  measurements: Measurement[]
}

export interface ApiError {
  _kind: 'ApiError'
  message: string
}
