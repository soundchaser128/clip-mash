import {ClipOptions} from "../api"

export interface JsonError {
  name: "JsonError"
  message: "error"
  error: string | Record<string, string>
}

export type ClipStrategy = ClipOptions["clipPicker"]["type"]

export interface MarkerCount {
  title: string
  count: number
}

export interface MarkerGroup {
  markers: MarkerCount[]
  name: string
}
