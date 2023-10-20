import {formatDuration} from "date-fns"
import {ListVideoDto, SelectedMarker} from "./api"
import {FormState} from "./types/form-state"
import {scaleSequential} from "d3-scale"
import {interpolatePlasma} from "d3-scale-chromatic"
import React from "react"

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

export function getSegmentStyle(
  index: number,
  count: number,
): React.CSSProperties {
  const color = getSegmentColor(index, count)
  const textColor = getSegmentTextColor(color)

  return {
    backgroundColor: color,
    color: textColor,
  }
}

type DurationFormat = "long" | "short"

function padNumber(n: number): string {
  return n.toString().padStart(2, "0")
}

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

  const hours = Math.floor(duration / 3600)
  const minutes = Math.floor((duration % 3600) / 60)
  const seconds = Math.floor(duration % 60)

  if (durationFormat === "long") {
    return formatDuration(
      {
        hours,
        minutes,
        seconds,
      },
      {format: ["hours", "minutes", "seconds"]},
    )
  } else {
    if (hours > 0) {
      return `${padNumber(hours)}:${padNumber(minutes)}:${padNumber(seconds)}`
    } else {
      return `${padNumber(minutes)}:${padNumber(seconds)}`
    }
  }
}

export function parseTimestamp(input: string | number): number {
  if (typeof input === "string") {
    let seconds: number
    if (input.match(/^\d+:\d+:\d+$/)) {
      const [hh, mm, ss] = input.split(":").map((x) => parseInt(x, 10))
      seconds = ss + mm * 60 + hh * 60 * 60
    } else {
      const [mm, ss] = input.split(":").map((x) => parseInt(x, 10))
      seconds = ss + mm * 60
    }
    return seconds
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

export const dateTimeFormat = new Intl.DateTimeFormat("en-US", {
  dateStyle: "long",
  timeStyle: "short",
})

// format number of bytes as human readable string
export function formatBytes(bytes: number): string {
  const sizes = ["Bytes", "KB", "MB", "GB", "TB"]
  if (bytes === 0) {
    return "0 Bytes"
  }
  const i = Math.floor(Math.log(bytes) / Math.log(1024))
  return `${parseFloat((bytes / Math.pow(1024, i)).toFixed(2))} ${sizes[i]}`
}
