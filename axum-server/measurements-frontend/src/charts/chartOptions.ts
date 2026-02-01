import type { ChartOptions, TooltipItem } from 'chart.js'
import type { Measurement } from '../assets.js'

interface Colors {
  textColor: string
  gridColor: string
}

interface MeasurementAxisMinMax {
  min: number
  max: number
}

function timeAxis(
  textColor: string,
  gridColor: string,
): any /* eslint-disable-line @typescript-eslint/no-explicit-any */ {
  return {
    type: 'time' as const,
    time: {
      unit: 'hour' as const,
      tooltipFormat: 'M/d/yyyy, h:mm:ss a',
    },
    ticks: {
      source: 'auto' as const,
      autoSkip: false,
      minRotation: 0,
      maxRotation: 0,
      color: textColor,
      callback(
        this: unknown,
        value: string | number | Date,
        index: number,
        ticks: TooltipItem<'line'>[],
      ) {
        let t: number
        if (typeof value === 'number') t = value
        else if (value instanceof Date) t = value.getTime()
        else {
          const n = Number(value)
          t = Number.isFinite(n) ? n : Date.parse(String(value))
        }
        let middleTimestampAt = Math.trunc(ticks.length / 2)
        middleTimestampAt =
          middleTimestampAt % 2 == 1 ? Math.max(middleTimestampAt - 1, 0) : middleTimestampAt
        if (index == 0 || index === middleTimestampAt || index == ticks.length - 1) {
          return new Date(t).toLocaleTimeString('en-US', {
            hour: '2-digit',
            minute: '2-digit',
            hour12: true,
          })
        }
        return ''
      },
    },
    grid: {
      color: gridColor,
    },
  }
}

export function calculateMeasurementAxisMinMax(
  measurements: Measurement[],
  defaultMinMax: MeasurementAxisMinMax,
  callback: (measurement: Measurement) => number,
): MeasurementAxisMinMax {
  if (measurements.length === 0) {
    return defaultMinMax
  }
  const measurementsForType = measurements.map(callback)
  const minMeasured = Math.floor(Math.min(...measurementsForType))
  const maxMeasured = Math.ceil(Math.max(...measurementsForType))

  if (maxMeasured - minMeasured <= 1) {
    return {
      min: minMeasured - 1,
      max: maxMeasured + 1,
    }
  }

  return {
    min: minMeasured,
    max: maxMeasured,
  }
}

export function generateChartOptions(
  title: string,
  yAxisMinMax: MeasurementAxisMinMax,
  stepSize: number,
  colors: Colors,
): ChartOptions<'line'> {
  return {
    responsive: true,
    maintainAspectRatio: false,
    scales: {
      x: timeAxis(colors.textColor, colors.gridColor),
      y: {
        title: {
          color: colors.textColor,
          display: true,
          text: title,
        },
        min: yAxisMinMax.min,
        max: yAxisMinMax.max,
        ticks: { color: colors.textColor, stepSize: stepSize },
        grid: {
          color: colors.gridColor,
        },
      },
    },
    plugins: {
      legend: { labels: { color: colors.textColor } },
    },
  }
}

export const tension = 0.4
