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
  "bg-purple-400",
  "bg-green-400",
  "bg-yellow-400",
  "bg-red-400",
  "bg-teal-400",
  "bg-orange-600",
  "bg-rose-400",
]

export function getSegmentColor(index: number): string {
  return segmentColors[index % segmentColors.length]
}
