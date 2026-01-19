export function timeAxis(textColor: string, gridColor: string): any {
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
        var middleTimestampAt = Math.trunc(ticks.length / 2)
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

export const tension = 0.4
