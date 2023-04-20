export interface Tag {
  name: string
  id: string
  count: number
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

export interface SelectedMarker {
  id: string
  duration: number
  selected: boolean
}

export type SelectMode = "tags" | "performers" | "scenes"

export interface FormState {
  selectMode?: SelectMode
  selectedIds?: string[]
  clipOrder?: "random" | "scene-order"
  clipDuration?: number
  outputResolution?: "720" | "1080" | "4K"
  outputFps?: number
  selectedMarkers?: SelectedMarker[]
  markers?: unknown[]
  fileName?: string
  clips?: Clip[]
  splitClips?: boolean
  includeAll?: boolean
  interactive?: boolean
  stage: FormStage
  id: string
}

export interface Clip {
  markerId: string
  sceneId: string
  range: [number, number]
  markerIndex: number
}

export interface Scene {
  id: string
  title: string
  studio?: string
  imageUrl: string
  performers: string[]
  tags: string[]
  markerCount: number
  interactive: boolean
  rating?: number
}
