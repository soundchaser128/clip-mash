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
