import {formatDuration} from "date-fns"
import {SelectedMarker} from "./api"
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

type DurationFormat = "long" | "short" | "short-with-ms"

function padNumber(n: number, padding = 2): string {
  return n.toString().padStart(padding, "0")
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
    } else if (durationFormat === "short") {
      return "00:00"
    } else {
      return "00:00.0000"
    }
  }

  const hours = Math.floor(duration / 3600)
  const minutes = Math.floor((duration % 3600) / 60)
  const seconds = Math.floor(duration % 60)
  const milliseconds = Math.floor((duration % 1) * 1000)

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
    let result = `${padNumber(minutes)}:${padNumber(seconds)}`
    if (hours > 0) {
      result = `${padNumber(hours)}:${result}`
    }

    if (durationFormat === "short-with-ms") {
      result = `${result}.${padNumber(milliseconds, 3)}`
    }

    return result
  }
}

const durationRegex =
  /^(?<hours>(\d+):)?(?<minutes>\d+):(?<seconds>\d+)\.?(?<millis>\d*)$/

export function parseTimestamp(input: string | number): number {
  if (typeof input === "string") {
    const match = input.match(durationRegex)
    if (match?.groups) {
      const {hours, minutes, seconds, millis} = match.groups
      const hoursInt = parseInt(hours || "0", 10)
      const minutesInt = parseInt(minutes, 10)
      const secondsInt = parseInt(seconds, 10)
      const millisInt = parseInt(millis || "0", 10)

      return hoursInt * 3600 + minutesInt * 60 + secondsInt + millisInt / 1000
    } else {
      return 0
    }
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
  dateStyle: "medium",
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

export function saveJsonToDisk<T>(fileName: string, data: T) {
  const json = JSON.stringify(data)
  const blob = new Blob([json], {type: "application/json"})
  saveBlobToDisk(fileName, blob)
}

export function saveBlobToDisk(fileName: string, blob: Blob) {
  const href = URL.createObjectURL(blob)
  const link = document.createElement("a")
  link.href = href
  link.download = fileName
  document.body.appendChild(link)
  link.click()
  document.body.removeChild(link)
  URL.revokeObjectURL(href)
}
