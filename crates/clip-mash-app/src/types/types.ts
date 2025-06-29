import {ClipOptions} from "../api"

export interface JsonError {
  name: "JsonError"
  message: "error"
  error: string | Record<string, string>
}

export type ClipStrategy = ClipOptions["clipPicker"]["type"]

export type DeepPartial<T> = {
  [P in keyof T]?: T[P] extends object ? DeepPartial<T[P]> : T[P]
}
