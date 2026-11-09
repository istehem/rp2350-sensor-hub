export type Comparator<T> = (a: T, b: T) => number

export function max<T>(items: readonly T[], compare: Comparator<T>, fallback: T): T {
  if (items.length === 0) return fallback

  return items.reduce((best, item) => (compare(item, best) > 0 ? item : best))
}

export function min<T>(items: readonly T[], compare: Comparator<T>, fallback: T): T {
  if (items.length === 0) return fallback

  return items.reduce((best, item) => (compare(item, best) < 0 ? item : best))
}
