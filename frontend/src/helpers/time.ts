import {formatDuration} from "date-fns"
import {SelectedMarker} from "../api"

function padNumber(n: number, padding = 2): string {
  return n.toString().padStart(padding, "0")
}

type DurationFormat = "long" | "short" | "short-with-ms"

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

export const dateTimeFormat = new Intl.DateTimeFormat("en-US", {
  dateStyle: "medium",
  timeStyle: "short",
})
