import {
  Clip,
  Marker,
  SelectMode,
  SelectedMarker,
  VideoWithMarkers,
} from "./types"

type TaggedState<T extends string, U extends number> = {tag: T; index: U}

export type VideoSource = "localFiles" | "stash"

export interface InitialState {
  id: string
}

export type Initial = TaggedState<"Initial", 0> & InitialState

export interface SelectSourceState extends InitialState {
  source: VideoSource
}

export type SelectSource = TaggedState<"SelectSource", 1> & SelectSourceState

export interface SelectStashModeState extends SelectSourceState {
  mode: SelectMode
}

export type SelectStashMode = TaggedState<"SelectStashMode", 2> &
  SelectStashModeState

export interface SelectStashCriteriaState extends SelectStashModeState {
  selectedIds: string[]
  fileName: string
  includeAll: boolean
}

export type SelectStashCriteria = TaggedState<"SelectStashCriteria", 3> &
  SelectStashCriteriaState

export interface SelectStashMarkersState extends SelectStashCriteriaState {
  selectedMarkers: SelectedMarker[]
  markers: Marker[]
  interactive: boolean
}

export type SelectStashMarkers = TaggedState<"SelectStashMarkers", 4> &
  SelectStashMarkersState

export type ClipOrder = "random" | "scene-order"

export interface VideoOptionsState
  extends Omit<SelectStashMarkersState, "markers"> {
  clipDuration: number
  clipOrder: ClipOrder
  outputFps: number
  outputResolution: "720" | "1080" | "4K"
  splitClips: boolean
}

export type VideoOptions = TaggedState<"VideoOptions", 5> & VideoOptionsState

export interface PreviewClipsState extends VideoOptions {
  clips: Clip[]
}

export type PreviewClips = TaggedState<"PreviewClips", 6> & PreviewClipsState

export type CreateVideo = TaggedState<"CreateVideo", 7> & PreviewClipsState

export interface SelectPathState extends SelectSourceState {
  localVideoPath: string
  recurse: boolean
}

export type SelectPath = TaggedState<"SelectPath", 1> & SelectPathState

export interface ListVideosState extends SelectPathState {
  videos: VideoWithMarkers[]
}

export type ListVideos = TaggedState<"ListVideos", 2> & ListVideosState

export type State =
  | Initial
  | SelectSource
  | SelectStashMode
  | SelectStashCriteria
  | SelectStashMarkers
  | VideoOptions
  | PreviewClips
  | CreateVideo
  | SelectPath
  | ListVideos

export const StateHelper = {
  isInitial(state: State): state is Initial {
    return state.tag == "Initial"
  },

  isSelectSource(state: State): state is SelectSource {
    return state.tag == "SelectSource"
  },

  isSelectStashMode(state: State): state is SelectStashMode {
    return state.tag == "SelectStashMode"
  },

  isSelectPath(state: State): state is SelectPath {
    return state.tag == "SelectPath"
  },

  isSelectStashCriteria(state: State): state is SelectStashCriteria {
    return state.tag == "SelectStashCriteria"
  },

  isSelectStashMarkers(state: State): state is SelectStashMarkers {
    return state.tag === "SelectStashMarkers"
  },

  isListVideos(state: State): state is ListVideos {
    return state.tag == "ListVideos"
  },

  isVideoOptions(state: State): state is VideoOptions {
    return state.tag == "VideoOptions"
  },

  isPreviewClips(state: State): state is PreviewClips {
    // @ts-expect-error not sure why this is failing
    return state.tag === "PreviewClips"
  },

  isCreateVideo(state: State): state is CreateVideo {
    // @ts-expect-error not sure why this is failing
    return state.tag == "CreateVideo"
  },
}
