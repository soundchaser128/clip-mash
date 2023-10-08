import {
  Clip,
  ClipOrder,
  EncodingEffort,
  MeasureCount,
  SelectedMarker,
  VideoCodec,
  VideoQuality,
  SongDto,
  MarkerDto,
} from "../api"
import {ClipStrategy} from "./types"

export enum FormStage {
  Start = 0,
  ListVideos = 1,
  SelectVideos = 2,
  SelectMarkers = 3,
  Music = 4,
  VideoOptions = 5,
  PreviewClips = 6,
  CreateVideo = 7,
}

export type ClipOrderType = ClipOrder["type"]

export interface FormState {
  stage: FormStage
  videoId?: string
  markers?: MarkerDto[]
  videoIds?: string[]
  recurse?: boolean
  clipOrder?: ClipOrder
  clipDuration?: number
  outputResolution?: [number, number]
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
  beatsPerMeasure?: number
  cutAfterMeasures?: MeasureCount
  clipStrategy?: ClipStrategy
  clipWeights?: Array<[string, number]>
  videoCodec?: VideoCodec
  videoQuality?: VideoQuality
  encodingEffort?: EncodingEffort
}
