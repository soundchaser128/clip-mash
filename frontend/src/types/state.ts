import {nanoid} from "nanoid"
import {
  Clip,
  Marker,
  SelectMode,
  SelectedMarker,
  VideoWithMarkers,
} from "./types"

type TaggedState<T extends string> = {tag: T}

export type VideoSource = "localFiles" | "stash"

export interface InitialState {
  id: string
}

export type Initial = TaggedState<"Initial"> & InitialState

export interface SelectSourceState extends InitialState {
  source: VideoSource
}

export type SelectSource = TaggedState<"SelectMode"> & SelectSourceState

export interface SelectStashModeState extends SelectSourceState {
  mode: SelectMode
}

export type SelectStashMode = TaggedState<"SelectStashMode"> &
  SelectStashModeState

export interface SelectStashCriteriaState extends SelectStashModeState {
  selectedIds: string[]
  fileName: string
  includeAll: boolean
}

export type SelectStashCriteria = TaggedState<"SelectStashCriteria"> &
  SelectStashCriteriaState

export interface SelectStashMarkersState extends SelectStashCriteriaState {
  selectedMarkers: SelectedMarker[]
  markers: Marker[]
  interactive: boolean
}

export type ClipOrder = "random" | "scene-order"

export interface VideoOptionsState
  extends Omit<SelectStashMarkersState, "markers"> {
  clipDuration: number
  clipOrder: ClipOrder
  outputFps: number
  outputResolution: "720" | "1080" | "4K"
  splitClips: boolean
}

export type VideoOptions = TaggedState<"VideoOptions"> & VideoOptionsState

export interface PreviewClipsState extends VideoOptions {
  clips: Clip[]
}

export type PreviewClips = TaggedState<"PreviewClips"> & PreviewClipsState

export type CreateVideo = TaggedState<"CreateVideo"> & PreviewClipsState

export interface SelectPathState extends SelectSourceState {
  localVideoPath: string
  recurse: boolean
}

export type SelectPath = TaggedState<"SelectPath"> & SelectPathState

export interface ListVideosState extends SelectPathState {
  videos: VideoWithMarkers[]
}

export type ListVideos = TaggedState<"ListVideos"> & ListVideosState

export type State =
  | Initial
  | SelectSource
  | SelectStashMode
  | SelectStashCriteria
  | VideoOptions
  | PreviewClips
  | CreateVideo
  | SelectPath
  | ListVideos
