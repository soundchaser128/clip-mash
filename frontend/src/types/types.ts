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

export interface InitialFormState {
  source: undefined
  id: string
}

export interface LocalVideosFormState {
  source: "local-files"
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
  performers: string[]
  fileName: string
  interactive: boolean
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
