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
  Music = 4,
  VideoOptions = 5,
  PreviewClips = 6,
  Wait = 7,
}

export enum LocalFilesFormStage {
  SelectPath = 1,
  ListVideos = 2,
  Music = 3,
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
}

export type SelectMode = "tags" | "performers" | "scenes"

export type VideoSource = "stash" | "localFile" | undefined

export type FormState = LocalVideosFormState | StashFormState | InitialFormState

export interface InitialFormState {
  source: undefined
  id: string
}

interface CommonFormState {
  id: string
  videos?: VideoWithMarkers[]
  localVideoPath?: string
  recurse?: boolean
  clipOrder?: "random" | "scene-order"
  clipDuration?: number
  outputResolution?: "720" | "1080" | "4K"
  outputFps?: number
  selectedMarkers?: SelectedMarker[]
  splitClips?: boolean
  fileName?: string
  clips?: Clip[]
  interactive?: boolean
  seed?: string
  songs?: SongDto[]
  musicVolume?: number
  trimVideoForSongs?: boolean
}

export interface LocalVideosFormState extends CommonFormState {
  source: "localFile"
  stage: LocalFilesFormStage
}

export interface StashFormState extends CommonFormState {
  source: "stash"
  selectMode?: SelectMode
  selectedIds?: string[]
  includeAll?: boolean
  markers?: Marker[]
  stage: FormStage
}

export const StateHelpers = {
  isStash(state: FormState): state is StashFormState {
    return state.source === "stash"
  },

  isLocalFiles(state: FormState): state is LocalVideosFormState {
    return state.source === "localFile"
  },

  isNotInitial(
    state: FormState
  ): state is StashFormState | LocalVideosFormState {
    return state.source === "stash" || state.source === "localFile"
  },

  isInitial(state: FormState): state is InitialFormState {
    return state.source === undefined
  },
}

export interface Clip {
  source: "stash" | "localFile"
  videoId: VideoId
  markerId: MarkerId
  range: [number, number]
  indexWithinVideo: number
  indexWithinMarker: number
}

export interface VideoDto {
  id: VideoId
  title: string
  performers: string[]
  fileName: string
  interactive: boolean
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

export interface VideoWithMarkers {
  video: VideoDto
  markers: Marker[]
}

export interface Scene {
  id: string
  performers: string[]
  imageUrl: string
  title: string
  studio: string
  tags: string[]
  rating?: number
  interactive: boolean
  markerCount: number
}

export interface SongDto {
  songId: number
  duration: number
  fileName: string
  url: string
}
