import {ClipFormInputs} from "@/routes/clips/settings/ClipSettingsForm"
import {
  Clip,
  ClipOrder,
  EncodingEffort,
  SelectedMarker,
  VideoCodec,
  VideoQuality,
  SongDto,
  MarkerDto,
  PaddingType,
} from "../api"

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
  outputResolution?: [number, number]
  outputFps?: number
  selectedMarkers?: SelectedMarker[]
  fileName?: string
  clips?: Clip[]
  interactive?: boolean
  songs?: SongDto[]
  musicVolume?: number
  videoCodec?: VideoCodec
  videoQuality?: VideoQuality
  encodingEffort?: EncodingEffort
  clipWeights?: [string, number][]
  clipOptions?: ClipFormInputs
  padding?: PaddingType
  includeOriginalFileName?: boolean
}

export interface SerializedFormState extends FormState {
  clipMashVersion: string
}
