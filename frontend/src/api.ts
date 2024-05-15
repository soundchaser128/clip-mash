/**
 * Generated by orval v6.28.2 🍺
 * Do not edit manually.
 * clip-mash
 * OpenAPI spec version: 0.21.1
 */
import {customInstance} from "./custom-client"
/**
 * @nullable
 */
export type SetConfig204 = unknown | null

export type GetHealthParams = {
  url: string
  apiKey?: string | null
}

export type DownloadMusicParams = {
  url: string
}

export type DownloadVideoParams = {
  videoId: string
}

/**
 * @nullable
 */
export type DeleteProgress200 = unknown | null

export type DetectMarkersParams = {
  /**
   * The threshold for the marker detection (from 0.0 to 1.0)
   */
  threshold?: number | null
}

/**
 * @nullable
 */
export type DeleteVideo200 = unknown | null

/**
 * @nullable
 */
export type UpdateVideo200 = unknown | null

export type GetFileStats200ItemItem = FolderType & number

/**
 * @nullable
 */
export type MigratePreviewImages200 = unknown | null

export type SplitMarkerParams = {
  /**
   * The time to split the marker at
   */
  time: number
}

/**
 * @nullable
 */
export type DeleteMarker200 = unknown | null

export type ListMarkerTitlesParams = {
  count?: number | null
  prefix?: string | null
}

export type ListMarkersParams = {
  videoIds?: string | null
}

export type ListFileEntriesParams = {
  path?: string | null
  includeHidden?: boolean | null
}

/**
 * @nullable
 */
export type CleanupFolder200 = unknown | null

export type WeightedRandomClipOptionsWeightsItemItem = string & number

export interface WeightedRandomClipOptions {
  clipLengths: ClipLengthOptions
  length: number
  /** @nullable */
  minClipDuration?: number | null
  weights: WeightedRandomClipOptionsWeightsItemItem[][]
}

export interface VideoUpdate {
  /** @nullable */
  tags?: string[] | null
  /** @nullable */
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
  createdOn: number
  duration: number
  fileName: string
  /** @nullable */
  filePath?: string | null
  id: string
  interactive: boolean
  performers: string[]
  source: VideoSource
  /** @nullable */
  stashSceneId?: number | null
  tags: string[]
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
  /** @minimum 0 */
  deletedCount: number
}

export interface UpdateMarker {
  /** @nullable */
  end?: number | null
  /** @nullable */
  start?: number | null
  /** @nullable */
  stashMarkerId?: number | null
  /** @nullable */
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
  /** @minimum 0 */
  n: number
}

export type StrokeTypeOneOf = {
  /** Creates a stroke every `n` beats */
  everyNth: StrokeTypeOneOfEveryNth
}

export interface StashVideoDto {
  createdOn: number
  duration: number
  existsInDatabase: boolean
  fileName: string
  id: string
  interactive: boolean
  /** @minimum 0 */
  markerCount: number
  performers: string[]
  source: VideoSource
  /** @nullable */
  stashSceneId?: number | null
  tags: string[]
  title: string
}

export interface StashVideoDtoPage {
  content: StashVideoDto[]
  /** @minimum 0 */
  pageNumber: number
  /** @minimum 0 */
  pageSize: number
  /** @minimum 0 */
  totalItems: number
  /** @minimum 0 */
  totalPages: number
}

export interface StashConfig {
  /** @nullable */
  apiKey?: string | null
  stashUrl: string
}

export type SortDirection = (typeof SortDirection)[keyof typeof SortDirection]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const SortDirection = {
  asc: "asc",
  desc: "desc",
} as const

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
  source?: VideoSource | null
  hasMarkers?: boolean | null
  isInteractive?: boolean | null
  page?: number | null
  size?: number | null
  sort?: string | null
  dir?: SortDirection | null
}

export interface SongUpload {
  file: Blob
}

export interface SongDto {
  beats: number[]
  duration: number
  fileName: string
  songId: number
  url: string
}

export interface SongClipOptions {
  /** @minimum 0 */
  beatsPerMeasure: number
  cutAfterMeasures: MeasureCount
  songs: Beats[]
}

/**
 * @nullable
 */
export type SettingsApp = AppConfig | null

export interface Settings {
  /** @nullable */
  app?: SettingsApp
  stash: StashConfig
}

export type SelectedMarkerSelectedRangeItem = number & number

export interface SelectedMarker {
  id: number
  /** @minimum 0 */
  indexWithinVideo: number
  /** @minimum 0 */
  loops: number
  /** @nullable */
  selected?: boolean | null
  selectedRange: SelectedMarkerSelectedRangeItem[]
  source: VideoSource
  title: string
  videoId: string
}

export interface RoundRobinClipOptions {
  clipLengths: ClipLengthOptions
  length: number
  lenientDuration: boolean
  /** @nullable */
  minClipDuration?: number | null
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
  /** @nullable */
  etaSeconds?: number | null
  itemsFinished: number
  itemsTotal: number
  message: string
  timestamp: string
  videoId: string
}

export type PaddingType = (typeof PaddingType)[keyof typeof PaddingType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const PaddingType = {
  black: "black",
  blur: "blur",
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
  /** @minimum 0 */
  max: number
  /** @minimum 0 */
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
  /** @minimum 0 */
  count: number
  type: MeasureCountOneOfType
}

export interface MarkerTitle {
  /** @minimum 0 */
  count: number
  title: string
}

export interface MarkerGroup {
  markers: MarkerTitle[]
  name: string
}

export interface MarkerDto {
  createdOn: number
  end: number
  /** @nullable */
  fileName?: string | null
  id: number
  /** @minimum 0 */
  indexWithinVideo: number
  primaryTag: string
  sceneInteractive: boolean
  /** @nullable */
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
  /** @minimum 0 */
  pageNumber: number
  /** @minimum 0 */
  pageSize: number
  /** @minimum 0 */
  totalItems: number
  /** @minimum 0 */
  totalPages: number
}

export interface MarkerCount {
  count: number
  title: string
}

export interface ListVideoDto {
  /** @minimum 0 */
  markerCount: number
  video: VideoDto
}

export interface ListVideoDtoPage {
  content: ListVideoDto[]
  /** @minimum 0 */
  pageNumber: number
  /** @minimum 0 */
  pageSize: number
  /** @minimum 0 */
  totalItems: number
  /** @minimum 0 */
  totalPages: number
}

export interface ListFileEntriesResponse {
  directory: string
  entries: FileSystemEntry[]
}

export type FolderType = (typeof FolderType)[keyof typeof FolderType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const FolderType = {
  tempVideo: "tempVideo",
  compilationVideo: "compilationVideo",
  downloadedVideo: "downloadedVideo",
  music: "music",
  database: "database",
  config: "config",
  previewImages: "previewImages",
} as const

export type FileSystemEntryOneOfThreeType =
  (typeof FileSystemEntryOneOfThreeType)[keyof typeof FileSystemEntryOneOfThreeType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const FileSystemEntryOneOfThreeType = {
  file: "file",
} as const

export type FileSystemEntryOneOfThree = {
  fileName: string
  fullPath: string
  /** @minimum 0 */
  size: number
  type: FileSystemEntryOneOfThreeType
}

export type FileSystemEntryOneOfType =
  (typeof FileSystemEntryOneOfType)[keyof typeof FileSystemEntryOneOfType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const FileSystemEntryOneOfType = {
  directory: "directory",
} as const

export type FileSystemEntryOneOf = {
  fileName: string
  fullPath: string
  type: FileSystemEntryOneOfType
}

export type FileSystemEntry = FileSystemEntryOneOf | FileSystemEntryOneOfThree

export interface EqualLengthClipOptions {
  clipDuration: number
  divisors: number[]
  /** @nullable */
  length?: number | null
  /** @nullable */
  minClipDuration?: number | null
}

export type EncodingEffort =
  (typeof EncodingEffort)[keyof typeof EncodingEffort]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const EncodingEffort = {
  low: "low",
  medium: "medium",
  high: "high",
} as const

export type DescriptionType =
  (typeof DescriptionType)[keyof typeof DescriptionType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const DescriptionType = {
  markdown: "markdown",
  json: "json",
  yaml: "yaml",
} as const

export interface DescriptionData {
  body: string
  contentType: string
}

/**
 * @nullable
 */
export type CreateVideoBodyPadding = PaddingType | null

export type CreateVideoBodyOutputResolutionItem = number & number

export interface CreateVideoBody {
  clips: Clip[]
  encodingEffort: EncodingEffort
  fileName: string
  forceReEncode: boolean
  /** @nullable */
  musicVolume?: number | null
  /** @minimum 0 */
  outputFps: number
  outputResolution: CreateVideoBodyOutputResolutionItem[]
  /** @nullable */
  padding?: CreateVideoBodyPadding
  selectedMarkers: SelectedMarker[]
  songIds: number[]
  videoCodec: VideoCodec
  videoId: string
  videoQuality: VideoQuality
}

export interface CreateMarker {
  /** @nullable */
  createdOn?: number | null
  end: number
  indexWithinVideo: number
  /** @nullable */
  markerStashId?: number | null
  /** @nullable */
  previewImagePath?: string | null
  start: number
  title: string
  videoId: string
  videoInteractive: boolean
}

export interface CreateMarkerRequest {
  createInStash: boolean
  marker: CreateMarker
}

export interface CreateInteractiveClipsBody {
  clipDuration: number
  markerTitles: string[]
}

export interface CreateFunscriptBody {
  clips: Clip[]
}

export interface CreateClipsBody {
  clipOrder: ClipOrder
  clips: ClipOptions
  markers: SelectedMarker[]
  /** @nullable */
  seed?: string | null
}

export interface CreateBeatFunscriptBody {
  songIds: number[]
  strokeType: StrokeType
}

export type ClipsResponseStreams = {[key: string]: string}

export interface ClipsResponse {
  /** @nullable */
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

export type ClipPickerOptions =
  | ClipPickerOptionsOneOf
  | ClipPickerOptionsOneOfFour
  | ClipPickerOptionsOneOfSeven
  | ClipPickerOptionsOneOfOnezero

export type ClipOrderOneOfSevenType =
  (typeof ClipOrderOneOfSevenType)[keyof typeof ClipOrderOneOfSevenType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const ClipOrderOneOfSevenType = {
  fixed: "fixed",
} as const

export type ClipOrderOneOfSeven = {
  markerTitleGroups: MarkerGroup[]
  type: ClipOrderOneOfSevenType
}

export type ClipOrderOneOfFiveType =
  (typeof ClipOrderOneOfFiveType)[keyof typeof ClipOrderOneOfFiveType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const ClipOrderOneOfFiveType = {
  "no-op": "no-op",
} as const

export type ClipOrderOneOfFive = {
  type: ClipOrderOneOfFiveType
}

export type ClipOrderOneOfThreeType =
  (typeof ClipOrderOneOfThreeType)[keyof typeof ClipOrderOneOfThreeType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const ClipOrderOneOfThreeType = {
  scene: "scene",
} as const

export type ClipOrderOneOfThree = {
  type: ClipOrderOneOfThreeType
}

export type ClipOrderOneOfType =
  (typeof ClipOrderOneOfType)[keyof typeof ClipOrderOneOfType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const ClipOrderOneOfType = {
  random: "random",
} as const

export type ClipOrderOneOf = {
  type: ClipOrderOneOfType
}

export type ClipOrder =
  | ClipOrderOneOf
  | ClipOrderOneOfThree
  | ClipOrderOneOfFive
  | ClipOrderOneOfSeven

export interface ClipOptions {
  clipPicker: ClipPickerOptions
  order: ClipOrder
}

export type ClipLengthOptionsOneOfFourAllOfType =
  (typeof ClipLengthOptionsOneOfFourAllOfType)[keyof typeof ClipLengthOptionsOneOfFourAllOfType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const ClipLengthOptionsOneOfFourAllOfType = {
  songs: "songs",
} as const

export type ClipLengthOptionsOneOfFourAllOf = {
  type: ClipLengthOptionsOneOfFourAllOfType
}

export type ClipLengthOptionsOneOfFour = SongClipOptions &
  ClipLengthOptionsOneOfFourAllOf

export type ClipLengthOptions =
  | ClipLengthOptionsOneOf
  | ClipLengthOptionsOneOfFour

export type ClipLengthOptionsOneOfAllOfType =
  (typeof ClipLengthOptionsOneOfAllOfType)[keyof typeof ClipLengthOptionsOneOfAllOfType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const ClipLengthOptionsOneOfAllOfType = {
  randomized: "randomized",
} as const

export type ClipLengthOptionsOneOfAllOf = {
  type: ClipLengthOptionsOneOfAllOfType
}

export type ClipLengthOptionsOneOf = RandomizedClipOptions &
  ClipLengthOptionsOneOfAllOf

export type ClipRangeItem = number & number

export interface Clip {
  /** @minimum 0 */
  indexWithinMarker: number
  /** @minimum 0 */
  indexWithinVideo: number
  markerId: number
  markerTitle: string
  /** Start and endpoint inside the video in seconds. */
  range: ClipRangeItem[]
  source: VideoSource
  videoId: string
}

export interface Beats {
  length: number
  offsets: number[]
}

export interface AppVersion {
  currentVersion: string
  needsUpdate: boolean
  newestVersion: string
}

export interface AppConfig {
  [key: string]: unknown
}

export type AddVideosRequestOneOfFiveType =
  (typeof AddVideosRequestOneOfFiveType)[keyof typeof AddVideosRequestOneOfFiveType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const AddVideosRequestOneOfFiveType = {
  stash: "stash",
} as const

export type AddVideosRequestOneOfFive = {
  sceneIds: number[]
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

export const cleanupFolder = (folderType: FolderType) => {
  return customInstance<CleanupFolder200>({
    url: `/api/library/cleanup/${folderType}`,
    method: "POST",
  })
}

export const listFileEntries = (params?: ListFileEntriesParams) => {
  return customInstance<ListFileEntriesResponse>({
    url: `/api/library/directory`,
    method: "GET",
    params,
  })
}

export const listMarkers = (params?: ListMarkersParams) => {
  return customInstance<MarkerDto[]>({
    url: `/api/library/marker`,
    method: "GET",
    params,
  })
}

export const createNewMarker = (createMarkerRequest: CreateMarkerRequest) => {
  return customInstance<MarkerDto>({
    url: `/api/library/marker`,
    method: "POST",
    headers: {"Content-Type": "application/json"},
    data: createMarkerRequest,
  })
}

export const listMarkerTitles = (params?: ListMarkerTitlesParams) => {
  return customInstance<MarkerCount[]>({
    url: `/api/library/marker/title`,
    method: "GET",
    params,
  })
}

export const updateMarker = (id: number, updateMarker: UpdateMarker) => {
  return customInstance<MarkerDto>({
    url: `/api/library/marker/${id}`,
    method: "PUT",
    headers: {"Content-Type": "application/json"},
    data: updateMarker,
  })
}

export const deleteMarker = (id: number) => {
  return customInstance<DeleteMarker200>({
    url: `/api/library/marker/${id}`,
    method: "DELETE",
  })
}

export const splitMarker = (id: number, params: SplitMarkerParams) => {
  return customInstance<MarkerDto[]>({
    url: `/api/library/marker/${id}/split`,
    method: "POST",
    params,
  })
}

export const migratePreviewImages = () => {
  return customInstance<MigratePreviewImages200>({
    url: `/api/library/migrate/preview`,
    method: "POST",
  })
}

export const getFileStats = () => {
  return customInstance<GetFileStats200ItemItem[][]>({
    url: `/api/library/stats`,
    method: "GET",
  })
}

export const listVideos = (params?: ListVideosParams) => {
  return customInstance<ListVideoDtoPage>({
    url: `/api/library/video`,
    method: "GET",
    params,
  })
}

export const addNewVideos = (addVideosRequest: AddVideosRequest) => {
  return customInstance<VideoDto[]>({
    url: `/api/library/video`,
    method: "POST",
    headers: {"Content-Type": "application/json"},
    data: addVideosRequest,
  })
}

export const cleanupVideos = () => {
  return customInstance<VideoCleanupResponse>({
    url: `/api/library/video/cleanup`,
    method: "POST",
  })
}

export const videosNeedEncoding = (videosNeedEncodingBody: string[]) => {
  return customInstance<boolean>({
    url: `/api/library/video/need-encoding`,
    method: "POST",
    headers: {"Content-Type": "application/json"},
    data: videosNeedEncodingBody,
  })
}

export const listStashVideos = (params?: ListStashVideosParams) => {
  return customInstance<StashVideoDtoPage>({
    url: `/api/library/video/stash`,
    method: "GET",
    params,
  })
}

export const getVideo = (id: string) => {
  return customInstance<VideoDetailsDto>({
    url: `/api/library/video/${id}`,
    method: "GET",
  })
}

export const updateVideo = (id: string, videoUpdate: VideoUpdate) => {
  return customInstance<UpdateVideo200>({
    url: `/api/library/video/${id}`,
    method: "PUT",
    headers: {"Content-Type": "application/json"},
    data: videoUpdate,
  })
}

export const deleteVideo = (id: string) => {
  return customInstance<DeleteVideo200>({
    url: `/api/library/video/${id}`,
    method: "DELETE",
  })
}

export const detectMarkers = (id: string, params?: DetectMarkersParams) => {
  return customInstance<MarkerDto[]>({
    url: `/api/library/video/${id}/detect-markers`,
    method: "POST",
    params,
  })
}

export const mergeStashVideo = (id: string) => {
  return customInstance<ListVideoDto>({
    url: `/api/library/video/${id}/stash/merge`,
    method: "POST",
  })
}

export const deleteProgress = (id: string) => {
  return customInstance<DeleteProgress200>({
    url: `/api/progress/${id}`,
    method: "DELETE",
  })
}

export const getProgressInfo = (id: string) => {
  return customInstance<Progress>({
    url: `/api/progress/${id}/info`,
    method: "GET",
  })
}

export const fetchClips = (createClipsBody: CreateClipsBody) => {
  return customInstance<ClipsResponse>({
    url: `/api/project/clips`,
    method: "POST",
    headers: {"Content-Type": "application/json"},
    data: createClipsBody,
  })
}

export const fetchClipsInteractive = (
  createInteractiveClipsBody: CreateInteractiveClipsBody,
) => {
  return customInstance<ClipsResponse>({
    url: `/api/project/clips/interactive`,
    method: "POST",
    headers: {"Content-Type": "application/json"},
    data: createInteractiveClipsBody,
  })
}

export const createVideo = (createVideoBody: CreateVideoBody) => {
  return customInstance<ProjectCreateResponse>({
    url: `/api/project/create`,
    method: "POST",
    headers: {"Content-Type": "application/json"},
    data: createVideoBody,
  })
}

export const generateDescription = (
  type: DescriptionType,
  createVideoBody: CreateVideoBody,
) => {
  return customInstance<DescriptionData>({
    url: `/api/project/description/${type}`,
    method: "POST",
    headers: {"Content-Type": "application/json"},
    data: createVideoBody,
  })
}

export const downloadVideo = (params: DownloadVideoParams) => {
  return customInstance<Blob>({
    url: `/api/project/download`,
    method: "GET",
    params,
    responseType: "blob",
  })
}

export const listFinishedVideos = () => {
  return customInstance<string[]>({url: `/api/project/finished`, method: "GET"})
}

export const getBeatFunscript = (
  createBeatFunscriptBody: CreateBeatFunscriptBody,
) => {
  return customInstance<unknown>({
    url: `/api/project/funscript/beat`,
    method: "POST",
    headers: {"Content-Type": "application/json"},
    data: createBeatFunscriptBody,
  })
}

export const getCombinedFunscript = (
  createFunscriptBody: CreateFunscriptBody,
) => {
  return customInstance<unknown>({
    url: `/api/project/funscript/combined`,
    method: "POST",
    headers: {"Content-Type": "application/json"},
    data: createFunscriptBody,
  })
}

export const getNewId = () => {
  return customInstance<NewId>({url: `/api/project/id`, method: "GET"})
}

export const listSongs = () => {
  return customInstance<SongDto[]>({url: `/api/song`, method: "GET"})
}

export const downloadMusic = (params: DownloadMusicParams) => {
  return customInstance<SongDto>({
    url: `/api/song/download`,
    method: "POST",
    params,
  })
}

export const uploadMusic = (songUpload: SongUpload) => {
  const formData = new FormData()
  formData.append("file", songUpload.file)

  return customInstance<SongDto>({
    url: `/api/song/upload`,
    method: "POST",
    headers: {"Content-Type": "multipart/form-data"},
    data: formData,
  })
}

export const getBeats = (id: number) => {
  return customInstance<Beats>({url: `/api/song/${id}/beats`, method: "GET"})
}

export const getHealth = (params: GetHealthParams) => {
  return customInstance<string>({
    url: `/api/stash/health`,
    method: "GET",
    params,
  })
}

export const getConfig = () => {
  return customInstance<Settings>({
    url: `/api/system/configuration`,
    method: "GET",
  })
}

export const setConfig = (settings: Settings) => {
  return customInstance<SetConfig204>({
    url: `/api/system/configuration`,
    method: "POST",
    headers: {"Content-Type": "application/json"},
    data: settings,
  })
}

export const getVersion = () => {
  return customInstance<AppVersion>({url: `/api/system/version`, method: "GET"})
}

export type CleanupFolderResult = NonNullable<
  Awaited<ReturnType<typeof cleanupFolder>>
>
export type ListFileEntriesResult = NonNullable<
  Awaited<ReturnType<typeof listFileEntries>>
>
export type ListMarkersResult = NonNullable<
  Awaited<ReturnType<typeof listMarkers>>
>
export type CreateNewMarkerResult = NonNullable<
  Awaited<ReturnType<typeof createNewMarker>>
>
export type ListMarkerTitlesResult = NonNullable<
  Awaited<ReturnType<typeof listMarkerTitles>>
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
export type MigratePreviewImagesResult = NonNullable<
  Awaited<ReturnType<typeof migratePreviewImages>>
>
export type GetFileStatsResult = NonNullable<
  Awaited<ReturnType<typeof getFileStats>>
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
export type VideosNeedEncodingResult = NonNullable<
  Awaited<ReturnType<typeof videosNeedEncoding>>
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
export type MergeStashVideoResult = NonNullable<
  Awaited<ReturnType<typeof mergeStashVideo>>
>
export type DeleteProgressResult = NonNullable<
  Awaited<ReturnType<typeof deleteProgress>>
>
export type GetProgressInfoResult = NonNullable<
  Awaited<ReturnType<typeof getProgressInfo>>
>
export type FetchClipsResult = NonNullable<
  Awaited<ReturnType<typeof fetchClips>>
>
export type FetchClipsInteractiveResult = NonNullable<
  Awaited<ReturnType<typeof fetchClipsInteractive>>
>
export type CreateVideoResult = NonNullable<
  Awaited<ReturnType<typeof createVideo>>
>
export type GenerateDescriptionResult = NonNullable<
  Awaited<ReturnType<typeof generateDescription>>
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
export type ListSongsResult = NonNullable<Awaited<ReturnType<typeof listSongs>>>
export type DownloadMusicResult = NonNullable<
  Awaited<ReturnType<typeof downloadMusic>>
>
export type UploadMusicResult = NonNullable<
  Awaited<ReturnType<typeof uploadMusic>>
>
export type GetBeatsResult = NonNullable<Awaited<ReturnType<typeof getBeats>>>
export type GetHealthResult = NonNullable<Awaited<ReturnType<typeof getHealth>>>
export type GetConfigResult = NonNullable<Awaited<ReturnType<typeof getConfig>>>
export type SetConfigResult = NonNullable<Awaited<ReturnType<typeof setConfig>>>
export type GetVersionResult = NonNullable<
  Awaited<ReturnType<typeof getVersion>>
>
