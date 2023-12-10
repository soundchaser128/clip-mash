export function isBetween(
  value: number,
  lower: number,
  upper: number,
): boolean {
  return value >= lower && value <= upper
}

export function clamp(value: number, lower: number, upper: number): number {
  return Math.min(Math.max(value, lower), upper)
}
