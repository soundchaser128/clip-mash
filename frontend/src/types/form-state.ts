import {
  Clip,
  ClipOrder,
  EncodingEffort,
  MeasureCount,
  SelectedMarker,
  VideoCodec,
  VideoQuality,
  SongDto,
} from "../api"
import {ClipStrategy, VideoWithMarkers} from "./types"

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
  videos?: VideoWithMarkers[]
  recurse?: boolean
  clipOrder?: ClipOrder
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
  beatsPerMeasure?: number
  cutAfterMeasures?: MeasureCount
  clipStrategy?: ClipStrategy
  clipWeights?: Array<[string, number]>
  videoCodec?: VideoCodec
  videoQuality?: VideoQuality
  encodingEffort?: EncodingEffort
  finalFileName?: string
}
