import {ClipOptions} from "../api"

export interface JsonError {
  name: "JsonError"
  message: "error"
  error: string | Record<string, string>
}

export type ClipStrategy = ClipOptions["clipPicker"]["type"]
