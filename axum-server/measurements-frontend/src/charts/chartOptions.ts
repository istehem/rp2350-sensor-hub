export const timeAxis = {
  type: 'time' as const,
  time: {
    unit: 'hour' as const,
    tooltipFormat: 'M/d/yyyy, h:mm:ss a',
    displayFormats: {
      hour: 'M/d/yyyy, h:mm a',
      minute: 'M/d/yyyy, h:mm a',
    },
  },
  title: {
    display: true,
    text: 'Time',
  },
}

export const tension = 0.4
