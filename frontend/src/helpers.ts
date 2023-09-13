import {format, formatDuration, getTime, parse} from "date-fns"
import {SelectedMarker} from "./api"
import {FormState} from "./types/form-state"
import {scaleSequential} from "d3-scale"
import {interpolatePlasma} from "d3-scale-chromatic"

export function getFormState(): FormState | null {
  const json = sessionStorage.getItem("form-state")
  if (json) {
    const state: {data: FormState} = JSON.parse(json)
    return state.data
  } else {
    return null
  }
}

export function getSegmentColor(index: number, count: number): string {
  const colorScale = scaleSequential()
    .domain([0, count - 1])
    .interpolator(interpolatePlasma)

  return colorScale(index)
}

export function getSegmentTextColor(color: string): string {
  const [r, g, b] = color
    .slice(1)
    .match(/.{1,2}/g)!
    .map((x) => parseInt(x, 16))

  const luma = 0.2126 * r + 0.7152 * g + 0.0722 * b

  if (luma < 140) {
    return "#ffffff"
  } else {
    return "#000000"
  }
}

type DurationFormat = "long" | "short"

export function formatSeconds(
  input: number | [number, number] | number[] | undefined,
  durationFormat: DurationFormat = "long",
): string {
  let duration = 0
  if (typeof input === "number") {
    duration = input
  } else if (Array.isArray(input)) {
    duration = input[1] - input[0]
  }

  if (duration === 0) {
    if (durationFormat === "long") {
      return "0 seconds"
    } else {
      return "00:00"
    }
  }

  const date = new Date(duration * 1000)
  if (durationFormat === "long") {
    return formatDuration(
      {
        hours: date.getUTCHours(),
        minutes: date.getUTCMinutes(),
        seconds: date.getUTCSeconds(),
      },
      {format: ["hours", "minutes", "seconds"]},
    )
  } else {
    if (date.getUTCHours() > 0) {
      return format(date, "HH:mm:ss")
    } else {
      return format(date, "mm:ss")
    }
  }
}

export function parseTimestamp(input: string | number): number {
  if (typeof input === "string") {
    const date = parse(input, "mm:ss", 0)
    const millis = getTime(date)
    return millis / 1000.0
  } else {
    return input
  }
}

export type HasDuration = Pick<
  SelectedMarker,
  "selected" | "selectedRange" | "loops"
>

export function sumDurations(markers?: HasDuration[]): number {
  if (!markers) {
    return 0
  } else {
    return markers
      .filter((m) => m.selected)
      .reduce(
        (sum, {selectedRange: [start, end], loops}) =>
          sum + (end - start) * loops,
        0,
      )
  }
}

export function pluralize(
  word: string,
  count: number | undefined | null,
): string {
  return count === 1 ? word : `${word}s`
}

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
