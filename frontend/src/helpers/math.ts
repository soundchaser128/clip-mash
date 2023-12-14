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

export function lerp(a: number, b: number, t: number): number {
  return a + (b - a) * t
}

export function lerpArrays(a: number[], b: number[], t: number): number[] {
  return a.map((v, i) => lerp(v, b[i], t))
}
