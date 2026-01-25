import type { ChartOptions } from 'chart.js'
import type { Measurement } from '../assets.js'

interface Colors {
  textColor: string
  gridColor: string
}

interface MeasurementAxisMinMax {
  min: number
  max: number
}

/* eslint-disable '@typescript-eslint/no-explicit-any' */
function timeAxis(textColor: string, gridColor: string): any {
  return {
    type: 'time' as const,
    time: {
      unit: 'hour' as const,
      stepSize: 1,
      tooltipFormat: 'M/d/yyyy, h:mm:ss a',
    },
    ticks: {
      source: 'auto' as const,
      autoSkip: false,
      minRotation: 0,
      maxRotation: 0,
      color: textColor,
      callback(this: any, value: string | number | Date, index: number, ticks: any[]) {
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
/* eslint-enable @typescript-eslint/no-explicit-any */

export function calculateMeasurementAxisMinMax(
  measurements: Measurement[],
  minMaxThresholds: MeasurementAxisMinMax,
  callback: (measurement: Measurement) => number,
): [number, number] {
  const measurementsForType = measurements.map(callback)
  measurementsForType.push(minMaxThresholds.min, minMaxThresholds.max)
  return [Math.ceil(Math.min(...measurementsForType)), Math.ceil(Math.max(...measurementsForType))]
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
