/**
 * Generated by orval v6.17.0 🍺
 * Do not edit manually.
 * clip-mash
 * OpenAPI spec version: 0.16.1
 */
import {customInstance} from "./custom-client"
export type DownloadVideoParams = {
  fileName: string
}

export type DetectMarkersParams = {
  /**
   * The threshold for the marker detection (from 0.0 to 1.0)
   */
  threshold?: number | null
}

export type SplitMarkerParams = {
  /**
   * The time to split the marker at
   */
  time: number
}

export type DeleteMarker200 = unknown | null

export type WeightedRandomClipOptionsWeightsItemItem = string & number

export interface WeightedRandomClipOptions {
  clipLengths: PmvClipOptions
  length: number
  weights: WeightedRandomClipOptionsWeightsItemItem[][]
}

export type VideoSource = (typeof VideoSource)[keyof typeof VideoSource]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const VideoSource = {
  Folder: "Folder",
  Download: "Download",
  Stash: "Stash",
} as const

export type VideoResolution =
  (typeof VideoResolution)[keyof typeof VideoResolution]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const VideoResolution = {
  NUMBER_720: "720",
  NUMBER_1080: "1080",
  "4K": "4K",
} as const

export type VideoQuality = (typeof VideoQuality)[keyof typeof VideoQuality]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const VideoQuality = {
  low: "low",
  medium: "medium",
  high: "high",
  lossless: "lossless",
} as const

export type VideoIdOneOfThreeType =
  (typeof VideoIdOneOfThreeType)[keyof typeof VideoIdOneOfThreeType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const VideoIdOneOfThreeType = {
  stash: "stash",
} as const

export type VideoIdOneOfThree = {
  id: string
  type: VideoIdOneOfThreeType
}

export type VideoId = VideoIdOneOf | VideoIdOneOfThree

export type VideoIdOneOfType =
  (typeof VideoIdOneOfType)[keyof typeof VideoIdOneOfType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const VideoIdOneOfType = {
  localFile: "localFile",
} as const

export type VideoIdOneOf = {
  id: string
  type: VideoIdOneOfType
}

export interface VideoDto {
  duration: number
  fileName: string
  id: VideoId
  interactive: boolean
  performers: string[]
  source: VideoSource
  title: string
}

export type VideoCodec = (typeof VideoCodec)[keyof typeof VideoCodec]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const VideoCodec = {
  av1: "av1",
  h264: "h264",
  h265: "h265",
} as const

export interface UpdateMarker {
  end: number
  rowid: number
  start: number
  title: string
}

/**
 * Steadily accelerates the strokes from `start_strokes_per_beat` to `end_strokes_per_beat`
 */
export type StrokeTypeOneOfThreeAccelerate = {
  end_strokes_per_beat: number
  start_strokes_per_beat: number
}

export type StrokeTypeOneOfThree = {
  /** Steadily accelerates the strokes from `start_strokes_per_beat` to `end_strokes_per_beat` */
  accelerate: StrokeTypeOneOfThreeAccelerate
}

/**
 * Creates a stroke every `n` beats
 */
export type StrokeTypeOneOfEveryNth = {
  n: number
}

export type StrokeTypeOneOf = {
  /** Creates a stroke every `n` beats */
  everyNth: StrokeTypeOneOfEveryNth
}

export type StrokeType = StrokeTypeOneOf | StrokeTypeOneOfThree

export type SortDirection = (typeof SortDirection)[keyof typeof SortDirection]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const SortDirection = {
  asc: "asc",
  desc: "desc",
} as const

export type ListVideosParams = {
  query?: string | null
  page?: number | null
  size?: number | null
  sort?: string | null
  dir?: SortDirection | null
}

export interface SongDto {
  beats: number[]
  duration: number
  fileName: string
  songId: number
  url: string
}

export interface SongClipOptions {
  beatsPerMeasure: number
  cutAfterMeasures: MeasureCount
  songs: Beats[]
}

export type SelectedMarkerSelectedRangeItem = number & number

export interface SelectedMarker {
  id: MarkerId
  indexWithinVideo: number
  loops: number
  selected?: boolean | null
  selectedRange: SelectedMarkerSelectedRangeItem[]
  title: string
  videoId: VideoId
}

export interface RoundRobinClipOptions {
  clipLengths: PmvClipOptions
  length: number
}

export interface RandomizedClipOptions {
  baseDuration: number
  divisors: number[]
}

export interface Progress {
  done: boolean
  etaSeconds?: number | null
  itemsFinished: number
  itemsTotal: number
  message: string
  timestamp: string
  videoId: string
}

export type PmvClipOptionsOneOfFourAllOfType =
  (typeof PmvClipOptionsOneOfFourAllOfType)[keyof typeof PmvClipOptionsOneOfFourAllOfType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const PmvClipOptionsOneOfFourAllOfType = {
  songs: "songs",
} as const

export type PmvClipOptionsOneOfFourAllOf = {
  type: PmvClipOptionsOneOfFourAllOfType
}

export type PmvClipOptionsOneOfFour = SongClipOptions &
  PmvClipOptionsOneOfFourAllOf

export type PmvClipOptionsOneOfAllOfType =
  (typeof PmvClipOptionsOneOfAllOfType)[keyof typeof PmvClipOptionsOneOfAllOfType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const PmvClipOptionsOneOfAllOfType = {
  randomized: "randomized",
} as const

export type PmvClipOptionsOneOfAllOf = {
  type: PmvClipOptionsOneOfAllOfType
}

export type PmvClipOptionsOneOf = RandomizedClipOptions &
  PmvClipOptionsOneOfAllOf

export type PmvClipOptions = PmvClipOptionsOneOf | PmvClipOptionsOneOfFour

export interface NewId {
  id: string
}

export type MeasureCountOneOfThreeType =
  (typeof MeasureCountOneOfThreeType)[keyof typeof MeasureCountOneOfThreeType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const MeasureCountOneOfThreeType = {
  random: "random",
} as const

export type MeasureCountOneOfThree = {
  max: number
  min: number
  type: MeasureCountOneOfThreeType
}

export type MeasureCountOneOf = {
  count: number
  type: MeasureCountOneOfType
}

export type MeasureCount = MeasureCountOneOf | MeasureCountOneOfThree

export type MeasureCountOneOfType =
  (typeof MeasureCountOneOfType)[keyof typeof MeasureCountOneOfType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const MeasureCountOneOfType = {
  fixed: "fixed",
} as const

export type MarkerIdOneOfThreeType =
  (typeof MarkerIdOneOfThreeType)[keyof typeof MarkerIdOneOfThreeType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const MarkerIdOneOfThreeType = {
  stash: "stash",
} as const

export type MarkerIdOneOfThree = {
  id: number
  type: MarkerIdOneOfThreeType
}

export type MarkerId = MarkerIdOneOf | MarkerIdOneOfThree

export type MarkerIdOneOfType =
  (typeof MarkerIdOneOfType)[keyof typeof MarkerIdOneOfType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const MarkerIdOneOfType = {
  localFile: "localFile",
} as const

export type MarkerIdOneOf = {
  id: number
  type: MarkerIdOneOfType
}

export interface MarkerDto {
  end: number
  fileName?: string | null
  id: MarkerId
  indexWithinVideo: number
  performers: string[]
  primaryTag: string
  sceneInteractive: boolean
  sceneTitle?: string | null
  screenshotUrl?: string | null
  start: number
  streamUrl: string
  tags: string[]
  videoId: VideoId
}

export interface ListVideoDto {
  markers: MarkerDto[]
  video: VideoDto
}

export interface ListVideoDtoPage {
  content: ListVideoDto[]
  pageNumber: number
  pageSize: number
  totalItems: number
  totalPages: number
}

export interface EqualLengthClipOptions {
  clipDuration: number
  divisors: number[]
}

export type EncodingEffort =
  (typeof EncodingEffort)[keyof typeof EncodingEffort]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const EncodingEffort = {
  low: "low",
  medium: "medium",
  high: "high",
} as const

export interface CreateVideoBody {
  clips: Clip[]
  encodingEffort: EncodingEffort
  fileName: string
  musicVolume?: number | null
  outputFps: number
  outputResolution: VideoResolution
  selectedMarkers: SelectedMarker[]
  songIds: number[]
  videoCodec: VideoCodec
  videoId: string
  videoQuality: VideoQuality
}

export interface CreateMarker {
  end: number
  indexWithinVideo: number
  previewImagePath?: string | null
  start: number
  title: string
  videoId: string
  videoInteractive: boolean
}

export interface CreateFunscriptBody {
  clips: Clip[]
  source: VideoSource
}

export interface CreateClipsBody {
  clipOrder: ClipOrder
  clips: ClipOptions
  markers: SelectedMarker[]
  seed?: string | null
}

export interface CreateBeatFunscriptBody {
  songIds: number[]
  strokeType: StrokeType
}

export interface Config {
  apiKey: string
  stashUrl: string
}

export type ClipsResponseStreams = {[key: string]: string}

export interface ClipsResponse {
  beatOffsets?: number[] | null
  clips: Clip[]
  streams: ClipsResponseStreams
  videos: VideoDto[]
}

export type ClipPickerOptionsOneOfOnezeroType =
  (typeof ClipPickerOptionsOneOfOnezeroType)[keyof typeof ClipPickerOptionsOneOfOnezeroType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const ClipPickerOptionsOneOfOnezeroType = {
  noSplit: "noSplit",
} as const

export type ClipPickerOptionsOneOfOnezero = {
  type: ClipPickerOptionsOneOfOnezeroType
}

export type ClipPickerOptions =
  | ClipPickerOptionsOneOf
  | ClipPickerOptionsOneOfFour
  | ClipPickerOptionsOneOfSeven
  | ClipPickerOptionsOneOfOnezero

export type ClipPickerOptionsOneOfSevenAllOfType =
  (typeof ClipPickerOptionsOneOfSevenAllOfType)[keyof typeof ClipPickerOptionsOneOfSevenAllOfType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const ClipPickerOptionsOneOfSevenAllOfType = {
  equalLength: "equalLength",
} as const

export type ClipPickerOptionsOneOfSevenAllOf = {
  type: ClipPickerOptionsOneOfSevenAllOfType
}

export type ClipPickerOptionsOneOfSeven = EqualLengthClipOptions &
  ClipPickerOptionsOneOfSevenAllOf

export type ClipPickerOptionsOneOfFourAllOfType =
  (typeof ClipPickerOptionsOneOfFourAllOfType)[keyof typeof ClipPickerOptionsOneOfFourAllOfType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const ClipPickerOptionsOneOfFourAllOfType = {
  weightedRandom: "weightedRandom",
} as const

export type ClipPickerOptionsOneOfFourAllOf = {
  type: ClipPickerOptionsOneOfFourAllOfType
}

export type ClipPickerOptionsOneOfFour = WeightedRandomClipOptions &
  ClipPickerOptionsOneOfFourAllOf

export type ClipPickerOptionsOneOfAllOfType =
  (typeof ClipPickerOptionsOneOfAllOfType)[keyof typeof ClipPickerOptionsOneOfAllOfType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const ClipPickerOptionsOneOfAllOfType = {
  roundRobin: "roundRobin",
} as const

export type ClipPickerOptionsOneOfAllOf = {
  type: ClipPickerOptionsOneOfAllOfType
}

export type ClipPickerOptionsOneOf = RoundRobinClipOptions &
  ClipPickerOptionsOneOfAllOf

export type ClipOrder = (typeof ClipOrder)[keyof typeof ClipOrder]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const ClipOrder = {
  random: "random",
  "scene-order": "scene-order",
  "no-op": "no-op",
} as const

export interface ClipOptions {
  clipPicker: ClipPickerOptions
  order: ClipOrder
}

export type ClipRangeItem = number & number

export interface Clip {
  indexWithinMarker: number
  indexWithinVideo: number
  markerId: MarkerId
  /** Start and endpoint inside the video in seconds. */
  range: ClipRangeItem[]
  source: VideoSource
  videoId: VideoId
}

export interface Beats {
  length: number
  offsets: number[]
}

export type AddVideosRequestOneOfFiveType =
  (typeof AddVideosRequestOneOfFiveType)[keyof typeof AddVideosRequestOneOfFiveType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const AddVideosRequestOneOfFiveType = {
  stash: "stash",
} as const

export type AddVideosRequestOneOfFive = {
  scene_ids: number[]
  type: AddVideosRequestOneOfFiveType
}

export type AddVideosRequestOneOfThreeType =
  (typeof AddVideosRequestOneOfThreeType)[keyof typeof AddVideosRequestOneOfThreeType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const AddVideosRequestOneOfThreeType = {
  download: "download",
} as const

export type AddVideosRequestOneOfThree = {
  type: AddVideosRequestOneOfThreeType
  urls: string[]
}

export type AddVideosRequestOneOfType =
  (typeof AddVideosRequestOneOfType)[keyof typeof AddVideosRequestOneOfType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const AddVideosRequestOneOfType = {
  local: "local",
} as const

export type AddVideosRequestOneOf = {
  path: string
  recurse: boolean
  type: AddVideosRequestOneOfType
}

export type AddVideosRequest =
  | AddVideosRequestOneOf
  | AddVideosRequestOneOfThree
  | AddVideosRequestOneOfFive

export const listMarkers = () => {
  return customInstance<MarkerDto[]>({
    url: `/api/library/marker`,
    method: "get",
  })
}

export const createNewMarker = (createMarker: CreateMarker) => {
  return customInstance<MarkerDto>({
    url: `/api/library/marker`,
    method: "post",
    headers: {"Content-Type": "application/json"},
    data: createMarker,
  })
}

export const updateMarker = (updateMarker: UpdateMarker) => {
  return customInstance<MarkerDto>({
    url: `/api/library/marker`,
    method: "put",
    headers: {"Content-Type": "application/json"},
    data: updateMarker,
  })
}

export const deleteMarker = (id: number) => {
  return customInstance<DeleteMarker200>({
    url: `/api/library/marker/${id}`,
    method: "delete",
  })
}

export const splitMarker = (id: number, params: SplitMarkerParams) => {
  return customInstance<MarkerDto[]>({
    url: `/api/library/marker/${id}/split`,
    method: "post",
    params,
  })
}

export const listVideos = (params?: ListVideosParams) => {
  return customInstance<ListVideoDtoPage>({
    url: `/api/library/video`,
    method: "get",
    params,
  })
}

export const addNewVideos = (addVideosRequest: AddVideosRequest) => {
  return customInstance<VideoDto[]>({
    url: `/api/library/video`,
    method: "post",
    headers: {"Content-Type": "application/json"},
    data: addVideosRequest,
  })
}

export const getVideo = (id: string) => {
  return customInstance<ListVideoDto>({
    url: `/api/library/video/${id}`,
    method: "get",
  })
}

export const detectMarkers = (id: string, params?: DetectMarkersParams) => {
  return customInstance<MarkerDto[]>({
    url: `/api/library/video/${id}/detect-markers`,
    method: "post",
    params,
  })
}

export const getProgressInfo = () => {
  return customInstance<Progress>({url: `/api/progress/info`, method: "get"})
}

export const fetchClips = (createClipsBody: CreateClipsBody) => {
  return customInstance<ClipsResponse>({
    url: `/api/project/clips`,
    method: "post",
    headers: {"Content-Type": "application/json"},
    data: createClipsBody,
  })
}

export const createVideo = (createVideoBody: CreateVideoBody) => {
  return customInstance<string>({
    url: `/api/project/create`,
    method: "post",
    headers: {"Content-Type": "application/json"},
    data: createVideoBody,
  })
}

export const downloadVideo = (params: DownloadVideoParams) => {
  return customInstance<Blob>({
    url: `/api/project/download`,
    method: "get",
    params,
    responseType: "blob",
  })
}

export const getBeatFunscript = (
  createBeatFunscriptBody: CreateBeatFunscriptBody,
) => {
  return customInstance<Blob>({
    url: `/api/project/funscript/beat`,
    method: "get",
    headers: {"Content-Type": "application/json"},
    responseType: "blob",
    data: createBeatFunscriptBody,
  })
}

export const getCombinedFunscript = (
  createFunscriptBody: CreateFunscriptBody,
) => {
  return customInstance<Blob>({
    url: `/api/project/funscript/combined`,
    method: "get",
    headers: {"Content-Type": "application/json"},
    responseType: "blob",
    data: createFunscriptBody,
  })
}

export const getNewId = () => {
  return customInstance<NewId>({url: `/api/project/id`, method: "get"})
}

export const getConfig = () => {
  return customInstance<Config>({url: `/api/stash/config`, method: "get"})
}

type AwaitedInput<T> = PromiseLike<T> | T

type Awaited<O> = O extends AwaitedInput<infer T> ? T : never

export type ListMarkersResult = NonNullable<
  Awaited<ReturnType<typeof listMarkers>>
>
export type CreateNewMarkerResult = NonNullable<
  Awaited<ReturnType<typeof createNewMarker>>
>
export type UpdateMarkerResult = NonNullable<
  Awaited<ReturnType<typeof updateMarker>>
>
export type DeleteMarkerResult = NonNullable<
  Awaited<ReturnType<typeof deleteMarker>>
>
export type SplitMarkerResult = NonNullable<
  Awaited<ReturnType<typeof splitMarker>>
>
export type ListVideosResult = NonNullable<
  Awaited<ReturnType<typeof listVideos>>
>
export type AddNewVideosResult = NonNullable<
  Awaited<ReturnType<typeof addNewVideos>>
>
export type GetVideoResult = NonNullable<Awaited<ReturnType<typeof getVideo>>>
export type DetectMarkersResult = NonNullable<
  Awaited<ReturnType<typeof detectMarkers>>
>
export type GetProgressInfoResult = NonNullable<
  Awaited<ReturnType<typeof getProgressInfo>>
>
export type FetchClipsResult = NonNullable<
  Awaited<ReturnType<typeof fetchClips>>
>
export type CreateVideoResult = NonNullable<
  Awaited<ReturnType<typeof createVideo>>
>
export type DownloadVideoResult = NonNullable<
  Awaited<ReturnType<typeof downloadVideo>>
>
export type GetBeatFunscriptResult = NonNullable<
  Awaited<ReturnType<typeof getBeatFunscript>>
>
export type GetCombinedFunscriptResult = NonNullable<
  Awaited<ReturnType<typeof getCombinedFunscript>>
>
export type GetNewIdResult = NonNullable<Awaited<ReturnType<typeof getNewId>>>
export type GetConfigResult = NonNullable<Awaited<ReturnType<typeof getConfig>>>
