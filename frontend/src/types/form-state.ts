import {
  Clip,
  ClipOrder,
  EncodingEffort,
  MarkerDto,
  MeasureCount,
  SelectedMarker,
  VideoCodec,
  VideoQuality,
} from "../api"
import {ClipStrategy, SelectMode, VideoWithMarkers} from "./types"
// FIXME
import {SongDto} from "./types.generated"

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
  ListVideos = 1,
  SelectMarkers = 2,
  Music = 3,
  VideoOptions = 4,
  PreviewClips = 5,
  Wait = 6,
}

export type FormState = LocalVideosFormState | StashFormState | InitialFormState

interface CommonFormState {
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

export interface InitialFormState extends CommonFormState {
  source: undefined
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
  markers?: MarkerDto[]
  stage: FormStage
}

export const StateHelpers = {
  isStash(state: FormState): state is StashFormState {
    return state.source === "stash"
  },

  isLocalFiles(state: FormState): state is LocalVideosFormState {
    return state.source === "localFile"
  },
}
