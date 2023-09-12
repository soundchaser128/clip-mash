import {MarkerDto, VideoDto} from "../api"

export type SelectMode = "tags" | "performers" | "scenes"

export type VideoSource = "stash" | "localFile" | undefined

export type ClipStrategy =
  | "roundRobin"
  | "weightedRandom"
  | "equalLength"
  | "noSplit"

export interface VideoWithMarkers {
  video: VideoDto
  markers: MarkerDto[]
}

export interface JsonError {
  name: "JsonError"
  message: "error"
  error: string | Record<string, string>
}

export interface Page<T> {
  content: T[]
  totalItems: number
  pageNumber: number
  pageSize: number
  totalPages: number
}
