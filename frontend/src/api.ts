/**
 * Generated by orval v6.17.0 🍺
 * Do not edit manually.
 * clip-mash
 * OpenAPI spec version: 0.17.0-pre.1
 */
import {customInstance} from "./custom-client"
export type GetHealthParams = {
  url: string
  apiKey: string
}

export type SetConfig204 = unknown | null

export type DownloadVideoParams = {
  videoId: string
}

export type DetectMarkersParams = {
  /**
   * The threshold for the marker detection (from 0.0 to 1.0)
   */
  threshold?: number | null
}

export type DeleteVideo200 = unknown | null

export type UpdateVideo200 = unknown | null

export type ListStashVideosParams = {
  query?: string | null
  withMarkers?: boolean | null
  page?: number | null
  size?: number | null
  sort?: string | null
  dir?: SortDirection | null
}

export type ListVideosParams = {
  query?: string | null
  page?: number | null
  size?: number | null
  sort?: string | null
  dir?: SortDirection | null
}

export type SplitMarkerParams = {
  /**
   * The time to split the marker at
   */
  time: number
}

export type DeleteMarker200 = unknown | null

export type ListMarkersParams = {
  videoIds?: string | null
}

export type WeightedRandomClipOptionsWeightsItemItem = string & number

export interface WeightedRandomClipOptions {
  clipLengths: PmvClipOptions
  length: number
  weights: WeightedRandomClipOptionsWeightsItemItem[][]
}

export interface VideoUpdate {
  tags?: string[] | null
  title?: string | null
}

export type VideoSource = (typeof VideoSource)[keyof typeof VideoSource]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const VideoSource = {
  Folder: "Folder",
  Download: "Download",
  Stash: "Stash",
} as const

export type VideoQuality = (typeof VideoQuality)[keyof typeof VideoQuality]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const VideoQuality = {
  low: "low",
  medium: "medium",
  high: "high",
  lossless: "lossless",
} as const

export interface VideoDto {
  duration: number
  fileName: string
  filePath?: string | null
  id: string
  interactive: boolean
  performers: string[]
  source: VideoSource
  stashSceneId?: number | null
  tags?: string[] | null
  title: string
}

export interface VideoDetailsDto {
  markers: MarkerDto[]
  video: VideoDto
}

export type VideoCodec = (typeof VideoCodec)[keyof typeof VideoCodec]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const VideoCodec = {
  av1: "av1",
  h264: "h264",
  h265: "h265",
} as const

export interface VideoCleanupResponse {
  deletedCount: number
}

export interface UpdateMarker {
  end?: number | null
  start?: number | null
  title?: string | null
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

export type StrokeType = StrokeTypeOneOf | StrokeTypeOneOfThree

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

export interface StashVideoDto {
  duration: number
  existsInDatabase: boolean
  fileName: string
  id: string
  interactive: boolean
  markerCount: number
  performers: string[]
  source: VideoSource
  stashSceneId?: number | null
  tags: string[]
  title: string
}

export interface StashVideoDtoPage {
  content: StashVideoDto[]
  pageNumber: number
  pageSize: number
  totalItems: number
  totalPages: number
}

export interface StashConfig {
  apiKey: string
  stashUrl: string
}

export type SortDirection = (typeof SortDirection)[keyof typeof SortDirection]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const SortDirection = {
  asc: "asc",
  desc: "desc",
} as const

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
  id: number
  indexWithinVideo: number
  loops: number
  selected?: boolean | null
  selectedRange: SelectedMarkerSelectedRangeItem[]
  source: VideoSource
  title: string
  videoId: string
}

export interface RoundRobinClipOptions {
  clipLengths: PmvClipOptions
  length: number
}

export interface RandomizedClipOptions {
  baseDuration: number
  divisors: number[]
}

export interface ProjectCreateResponse {
  finalFileName: string
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

export type PmvClipOptionsOneOfAllOf = {
  type: PmvClipOptionsOneOfAllOfType
}

export type PmvClipOptionsOneOf = RandomizedClipOptions &
  PmvClipOptionsOneOfAllOf

export type PmvClipOptions = PmvClipOptionsOneOf | PmvClipOptionsOneOfFour

export type PmvClipOptionsOneOfAllOfType =
  (typeof PmvClipOptionsOneOfAllOfType)[keyof typeof PmvClipOptionsOneOfAllOfType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const PmvClipOptionsOneOfAllOfType = {
  randomized: "randomized",
} as const

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

export type MeasureCount = MeasureCountOneOf | MeasureCountOneOfThree

export type MeasureCountOneOfType =
  (typeof MeasureCountOneOfType)[keyof typeof MeasureCountOneOfType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const MeasureCountOneOfType = {
  fixed: "fixed",
} as const

export type MeasureCountOneOf = {
  count: number
  type: MeasureCountOneOfType
}

export interface MarkerDto {
  end: number
  fileName?: string | null
  id: number
  indexWithinVideo: number
  primaryTag: string
  sceneInteractive: boolean
  sceneTitle?: string | null
  screenshotUrl: string
  source: VideoSource
  start: number
  streamUrl: string
  tags: string[]
  videoId: string
}

export interface MarkerDtoPage {
  content: MarkerDto[]
  pageNumber: number
  pageSize: number
  totalItems: number
  totalPages: number
}

export interface ListVideoDto {
  markerCount: number
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

export type CreateVideoBodyOutputResolutionItem = number & number

export interface CreateVideoBody {
  clips: Clip[]
  encodingEffort: EncodingEffort
  fileName: string
  musicVolume?: number | null
  outputFps: number
  outputResolution: CreateVideoBodyOutputResolutionItem[]
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
  markerId: number
  /** Start and endpoint inside the video in seconds. */
  range: ClipRangeItem[]
  source: VideoSource
  videoId: string
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

export const listMarkers = (params?: ListMarkersParams) => {
  return customInstance<MarkerDto[]>({
    url: `/api/library/marker`,
    method: "get",
    params,
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

export const updateMarker = (id: number, updateMarker: UpdateMarker) => {
  return customInstance<MarkerDto>({
    url: `/api/library/marker/${id}`,
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

export const cleanupVideos = () => {
  return customInstance<VideoCleanupResponse>({
    url: `/api/library/video/cleanup`,
    method: "post",
  })
}

export const listStashVideos = (params?: ListStashVideosParams) => {
  return customInstance<StashVideoDtoPage>({
    url: `/api/library/video/stash`,
    method: "get",
    params,
  })
}

export const getVideo = (id: string) => {
  return customInstance<VideoDetailsDto>({
    url: `/api/library/video/${id}`,
    method: "get",
  })
}

export const updateVideo = (id: string, videoUpdate: VideoUpdate) => {
  return customInstance<UpdateVideo200>({
    url: `/api/library/video/${id}`,
    method: "put",
    headers: {"Content-Type": "application/json"},
    data: videoUpdate,
  })
}

export const deleteVideo = (id: string) => {
  return customInstance<DeleteVideo200>({
    url: `/api/library/video/${id}`,
    method: "delete",
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
  return customInstance<ProjectCreateResponse>({
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

export const listFinishedVideos = () => {
  return customInstance<string[]>({url: `/api/project/finished`, method: "get"})
}

export const getBeatFunscript = (
  createBeatFunscriptBody: CreateBeatFunscriptBody,
) => {
  return customInstance<unknown>({
    url: `/api/project/funscript/beat`,
    method: "post",
    headers: {"Content-Type": "application/json"},
    data: createBeatFunscriptBody,
  })
}

export const getCombinedFunscript = (
  createFunscriptBody: CreateFunscriptBody,
) => {
  return customInstance<unknown>({
    url: `/api/project/funscript/combined`,
    method: "post",
    headers: {"Content-Type": "application/json"},
    data: createFunscriptBody,
  })
}

export const getNewId = () => {
  return customInstance<NewId>({url: `/api/project/id`, method: "get"})
}

export const getConfig = () => {
  return customInstance<StashConfig>({url: `/api/stash/config`, method: "get"})
}

export const setConfig = (stashConfig: StashConfig) => {
  return customInstance<SetConfig204>({
    url: `/api/stash/config`,
    method: "post",
    headers: {"Content-Type": "application/json"},
    data: stashConfig,
  })
}

export const getHealth = (params: GetHealthParams) => {
  return customInstance<string>({
    url: `/api/stash/health`,
    method: "get",
    params,
  })
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
export type CleanupVideosResult = NonNullable<
  Awaited<ReturnType<typeof cleanupVideos>>
>
export type ListStashVideosResult = NonNullable<
  Awaited<ReturnType<typeof listStashVideos>>
>
export type GetVideoResult = NonNullable<Awaited<ReturnType<typeof getVideo>>>
export type UpdateVideoResult = NonNullable<
  Awaited<ReturnType<typeof updateVideo>>
>
export type DeleteVideoResult = NonNullable<
  Awaited<ReturnType<typeof deleteVideo>>
>
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
export type ListFinishedVideosResult = NonNullable<
  Awaited<ReturnType<typeof listFinishedVideos>>
>
export type GetBeatFunscriptResult = NonNullable<
  Awaited<ReturnType<typeof getBeatFunscript>>
>
export type GetCombinedFunscriptResult = NonNullable<
  Awaited<ReturnType<typeof getCombinedFunscript>>
>
export type GetNewIdResult = NonNullable<Awaited<ReturnType<typeof getNewId>>>
export type GetConfigResult = NonNullable<Awaited<ReturnType<typeof getConfig>>>
export type SetConfigResult = NonNullable<Awaited<ReturnType<typeof setConfig>>>
export type GetHealthResult = NonNullable<Awaited<ReturnType<typeof getHealth>>>
