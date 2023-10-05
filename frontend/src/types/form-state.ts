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
  SelectMarkers = 2,
  Music = 3,
  VideoOptions = 4,
  PreviewClips = 5,
  Wait = 6,
}

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
