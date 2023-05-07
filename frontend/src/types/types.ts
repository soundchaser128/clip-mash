export interface Tag {
  name: string
  id: string
  markerCount: number
}

export interface Performer {
  name: string
  id: string
  sceneCount: number
  imageUrl?: string
  tags: string[]
  rating?: number
  favorite: boolean
}

export enum FormStage {
  SelectMode = 1,
  SelectCriteria = 2,
  SelectMarkers = 3,
  VideoOptions = 4,
  PreviewClips = 5,
  Wait = 6,
}

export type IdSource = "stash" | "localFile"

export interface MarkerId {
  type: IdSource
  id: number
}

export interface VideoId {
  type: IdSource
  id: string
}

export interface SelectedMarker {
  id: MarkerId
  videoId: VideoId
  selectedRange: [number, number]
  indexWithinVideo: number
  selected: boolean
  duration: number
}

export type SelectMode = "tags" | "performers" | "scenes"

export type VideoSource = "stash" | "local-files" | undefined

export type FormState = LocalVideosFormState | StashFormState | InitialFormState

export interface InitialFormState {
  source: undefined
  id: string
}

export interface LocalVideosFormState {
  source: "local-files"
  id: string
  videos?: LocalVideoDto[]
  localVideoPath?: string
  recurse?: boolean
}

export interface Marker {
  id: MarkerId
  primaryTag: string
  streamUrl: string
  screenshotUrl: string
  start: number
  end: number
  sceneTitle?: string
  performers: string[]
  fileName?: string
  sceneInteractive: boolean
  tags: string[]
  indexWithinVideo: number
  videoId: VideoId
}

export interface StashFormState {
  source: "stash"
  selectMode?: SelectMode
  selectedIds?: string[]
  clipOrder?: "random" | "scene-order"
  clipDuration?: number
  outputResolution?: "720" | "1080" | "4K"
  outputFps?: number
  selectedMarkers?: SelectedMarker[]
  markers?: Marker[]
  fileName?: string
  clips?: Clip[]
  splitClips?: boolean
  includeAll?: boolean
  interactive?: boolean
  stage: FormStage
  id: string
}

export const StateHelpers = {
  isStash(state: FormState): state is StashFormState {
    return state.source === "stash"
  },

  isLocalFiles(state: FormState): state is LocalVideosFormState {
    return state.source === "local-files"
  },

  isInitial(state: FormState): state is InitialFormState {
    return state.source === undefined
  },
}

export interface Clip {
  source: "stash" | "localFiles"
  videoId: VideoId
  markerId: MarkerId
  range: [number, number]
  indexWithinVideo: number
  indexWithinMarker: number
}

export interface VideoDto {
  id: VideoId
  title: string
  // studio?: string
  // imageUrl: string
  performers: string[]
  // tags: string[]
  // markerCount: number
  // interactive: boolean
  // rating?: number
}

export interface LocalVideoDto {
  id: string
  fileName: string
  interactive: boolean
  markers: Marker[]
}
