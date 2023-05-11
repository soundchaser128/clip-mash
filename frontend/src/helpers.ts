import {format, formatDuration} from "date-fns"
import {FormState} from "./types/types"

export function getFormState(): FormState | null {
  const json = sessionStorage.getItem("form-state")
  if (json) {
    const state: {data: FormState} = JSON.parse(json)
    return state.data
  } else {
    return null
  }
}

export const segmentColors = [
  "bg-purple-400 hover:bg-purple-500 text-white",
  "bg-green-400 hover:bg-green-500 text-white",
  "bg-yellow-400 hover:bg-yellow-500 text-white",
  "bg-red-400 hover:bg-red-500 text-white",
  "bg-teal-400 hover:bg-teal-500 text-white",
  "bg-orange-600 hover:bg-orange-500 text-white",
  "bg-rose-400 hover:bg-rose-500 text-white",
]

export function getSegmentColor(index: number): string {
  return segmentColors[index % segmentColors.length]
}

type DurationFormat = "long" | "short"

export function formatSeconds(
  input: number | [number, number] | undefined,
  durationFormat: DurationFormat = "long"
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
      {format: ["hours", "minutes", "seconds"]}
    )
  } else {
    return format(duration * 1000, "mm:ss")
  }
}
