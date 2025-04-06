/**
 * Generated by orval v7.2.0 🍺
 * Do not edit manually.
 * clip-mash
 * OpenAPI spec version: 0.22.0-pre.5
 */
import {customInstance} from "./custom-client"
export type GetStashHealthParams = {
  url: string
  apiKey?: string | null
}

export type DownloadMusicParams = {
  url: string
}

export type ListSongsParams = {
  shuffle?: boolean | null
}

export type DownloadVideoParams = {
  videoId: string
}

export type DetectMarkersParams = {
  /**
   * The threshold for the marker detection (from 0.0 to 1.0)
   */
  threshold?: number | null
}

export type ListStashVideosParams = {
  query?: string | null
  withMarkers?: boolean | null
  page?: number | null
  size?: number | null
  sort?: string | null
  dir?: null | SortDirection
}

export type ListVideosParams = {
  query?: string | null
  source?: null | VideoSource
  hasMarkers?: boolean | null
  isInteractive?: boolean | null
  page?: number | null
  size?: number | null
  sort?: string | null
  dir?: null | SortDirection
}

export type GetFileStats200ItemItem0 =
  (typeof GetFileStats200ItemItem0)[keyof typeof GetFileStats200ItemItem0]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const GetFileStats200ItemItem0 = {
  tempVideo: "tempVideo",
  compilationVideo: "compilationVideo",
  downloadedVideo: "downloadedVideo",
  music: "music",
  database: "database",
  config: "config",
  previewImages: "previewImages",
} as const

export type ListPerformersParams = {
  prefix?: string | null
}

export type SplitMarkerParams = {
  /**
   * The time to split the marker at
   */
  time: number
}

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

export type HandyStatus200 = null | ControllerStatus

export type WeightedRandomClipOptionsMinClipDuration = number | null

export interface WeightedRandomClipOptions {
  clipLengths: ClipLengthOptions
  length: number
  minClipDuration?: WeightedRandomClipOptionsMinClipDuration
  weights: [string, number][]
}

export type VideoUpdateTitle = string | null

export type VideoUpdateTags = string[] | null

export interface VideoUpdate {
  tags?: VideoUpdateTags
  title?: VideoUpdateTitle
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

export type VideoDtoStashSceneId = number | null

export type VideoDtoFilePath = string | null

export interface VideoDto {
  createdOn: number
  duration: number
  fileName: string
  filePath?: VideoDtoFilePath
  id: string
  interactive: boolean
  source: VideoSource
  stashSceneId?: VideoDtoStashSceneId
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

export type UpdateMarkerTitle = string | null

export type UpdateMarkerStashMarkerId = number | null

export type UpdateMarkerStart = number | null

export type UpdateMarkerEnd = number | null

export interface UpdateMarker {
  end?: UpdateMarkerEnd
  start?: UpdateMarkerStart
  stashMarkerId?: UpdateMarkerStashMarkerId
  title?: UpdateMarkerTitle
}

export interface TagCount {
  count: number
  tag: string
}

/**
 * Steadily accelerates the strokes from `start_strokes_per_beat` to `end_strokes_per_beat`
 */
export type StrokeTypeOneOfThreeAccelerate = {
  end_strokes_per_beat: number
  start_strokes_per_beat: number
}

/**
 * Steadily accelerates the strokes from `start_strokes_per_beat` to `end_strokes_per_beat`
 */
export type StrokeTypeOneOfThree = {
  /** Steadily accelerates the strokes from `start_strokes_per_beat` to `end_strokes_per_beat` */
  accelerate: StrokeTypeOneOfThreeAccelerate
}

/**
 * Creates a stroke every `n` beats
 */
export type StrokeTypeOneOfEveryNth = {
  /** @minimum 0 */
  n: number
}

/**
 * Creates a stroke every `n` beats
 */
export type StrokeTypeOneOf = {
  /** Creates a stroke every `n` beats */
  everyNth: StrokeTypeOneOfEveryNth
}

export type StrokeType = StrokeTypeOneOf | StrokeTypeOneOfThree

export type StashVideoDtoStashSceneId = number | null

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
  stashSceneId?: StashVideoDtoStashSceneId
  tags: string[]
  title: string
}

export type StashConfigApiKey = string | null

export interface StashConfig {
  apiKey?: StashConfigApiKey
  stashUrl: string
}

export interface StartHandyParameters {
  key: string
  pattern: HandyPattern
}

export type SortDirection = (typeof SortDirection)[keyof typeof SortDirection]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const SortDirection = {
  asc: "asc",
  desc: "desc",
} as const

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

export type SettingsHandy = null | HandyConfig

export interface Settings {
  handy?: SettingsHandy
  stash: StashConfig
}

export type SelectedMarkerSelected = boolean | null

export interface SelectedMarker {
  id: number
  /** @minimum 0 */
  indexWithinVideo: number
  /** @minimum 0 */
  loops: number
  selected?: SelectedMarkerSelected
  selectedRange: [number, number]
  source: VideoSource
  title: string
  videoId: string
}

export type RoundRobinClipOptionsMinClipDuration = number | null

export interface RoundRobinClipOptions {
  clipLengths: ClipLengthOptions
  length: number
  lenientDuration: boolean
  minClipDuration?: RoundRobinClipOptionsMinClipDuration
}

export interface Range {
  max: number
  min: number
}

export interface RandomizedClipOptions {
  baseDuration: number
  spread: number
}

export type RandomParametersSeed = string | null

export interface RandomParameters {
  intervalRange: Range
  jitter: number
  seed?: RandomParametersSeed
  slideRange: Range
  speedRange: Range
}

export interface ProjectCreateResponse {
  finalFileName: string
}

export type ProgressEtaSeconds = number | null

export interface Progress {
  done: boolean
  etaSeconds?: ProgressEtaSeconds
  itemsFinished: number
  itemsTotal: number
  message: string
  timestamp: string
  videoId: string
}

export type PageStashVideoDtoContentItemStashSceneId = number | null

export type PageStashVideoDtoContentItem = {
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
  stashSceneId?: PageStashVideoDtoContentItemStashSceneId
  tags: string[]
  title: string
}

export interface PageStashVideoDto {
  content: PageStashVideoDtoContentItem[]
  /** @minimum 0 */
  pageNumber: number
  /** @minimum 0 */
  pageSize: number
  /** @minimum 0 */
  totalItems: number
  /** @minimum 0 */
  totalPages: number
}

export type PageListVideoDtoContentItem = {
  /** @minimum 0 */
  markerCount: number
  video: VideoDto
}

export interface PageListVideoDto {
  content: PageListVideoDtoContentItem[]
  /** @minimum 0 */
  pageNumber: number
  /** @minimum 0 */
  pageSize: number
  /** @minimum 0 */
  totalItems: number
  /** @minimum 0 */
  totalPages: number
}

export type PaddingType = (typeof PaddingType)[keyof typeof PaddingType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const PaddingType = {
  black: "black",
  blur: "blur-sm",
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

export type MarkerDtoSceneTitle = string | null

export type MarkerDtoFileName = string | null

export interface MarkerDto {
  createdOn: number
  end: number
  fileName?: MarkerDtoFileName
  id: number
  /** @minimum 0 */
  indexWithinVideo: number
  primaryTag: string
  sceneInteractive: boolean
  sceneTitle?: MarkerDtoSceneTitle
  screenshotUrl: string
  source: VideoSource
  start: number
  streamUrl: string
  tags: string[]
  videoId: string
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

export interface ListPerformerResponse {
  /** @minimum 0 */
  count: number
  title: string
}

export interface ListFileEntriesResponse {
  directory: string
  drives: string[]
  entries: FileSystemEntry[]
}

export type InteractiveClipsQueryOneOfFiveType =
  (typeof InteractiveClipsQueryOneOfFiveType)[keyof typeof InteractiveClipsQueryOneOfFiveType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const InteractiveClipsQueryOneOfFiveType = {
  videoTags: "videoTags",
} as const

export type InteractiveClipsQueryOneOfFive = {
  data: string[]
  type: InteractiveClipsQueryOneOfFiveType
}

export type InteractiveClipsQueryOneOfThreeType =
  (typeof InteractiveClipsQueryOneOfThreeType)[keyof typeof InteractiveClipsQueryOneOfThreeType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const InteractiveClipsQueryOneOfThreeType = {
  performers: "performers",
} as const

export type InteractiveClipsQueryOneOfThree = {
  data: string[]
  type: InteractiveClipsQueryOneOfThreeType
}

export type InteractiveClipsQueryOneOfType =
  (typeof InteractiveClipsQueryOneOfType)[keyof typeof InteractiveClipsQueryOneOfType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const InteractiveClipsQueryOneOfType = {
  markerTitles: "markerTitles",
} as const

export type InteractiveClipsQueryOneOf = {
  data: string[]
  type: InteractiveClipsQueryOneOfType
}

export type InteractiveClipsQuery =
  | InteractiveClipsQueryOneOf
  | InteractiveClipsQueryOneOfThree
  | InteractiveClipsQueryOneOfFive

export type HandyPatternOneOfFiveType =
  (typeof HandyPatternOneOfFiveType)[keyof typeof HandyPatternOneOfFiveType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const HandyPatternOneOfFiveType = {
  accellerate: "accellerate",
} as const

export type HandyPatternOneOfFive = {
  parameters: AccellerateParameters
  type: HandyPatternOneOfFiveType
}

export type HandyPattern =
  | HandyPatternOneOf
  | HandyPatternOneOfThree
  | HandyPatternOneOfFive

export type HandyPatternOneOfThreeType =
  (typeof HandyPatternOneOfThreeType)[keyof typeof HandyPatternOneOfThreeType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const HandyPatternOneOfThreeType = {
  random: "random",
} as const

export type HandyPatternOneOfThree = {
  parameters: RandomParameters
  type: HandyPatternOneOfThreeType
}

export type HandyPatternOneOfType =
  (typeof HandyPatternOneOfType)[keyof typeof HandyPatternOneOfType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const HandyPatternOneOfType = {
  "cycle-accellerate": "cycle-accellerate",
} as const

export type HandyPatternOneOf = {
  parameters: CycleAccellerateParameters
  type: HandyPatternOneOfType
}

export interface HandyConnectedResponse {
  connected: boolean
}

export interface HandyConfig {
  enabled: boolean
  key: string
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

export type EqualLengthClipOptionsMinClipDuration = number | null

export type EqualLengthClipOptionsLength = number | null

export interface EqualLengthClipOptions {
  clipDuration: number
  length?: EqualLengthClipOptionsLength
  minClipDuration?: EqualLengthClipOptionsMinClipDuration
  spread: number
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
} as const

export interface DescriptionData {
  body: string
  contentType: string
}

export interface CycleAccellerateParameters {
  cycleDuration: number
  endRange: Range
  sessionDuration: number
  slideRange: Range
  startRange: Range
}

export type CreateVideoBodyPadding = null | PaddingType

export type CreateVideoBodyMusicVolume = number | null

export interface CreateVideoBody {
  clips: Clip[]
  encodingEffort: EncodingEffort
  fileName: string
  forceReEncode: boolean
  includeOriginalFileName: boolean
  musicVolume?: CreateVideoBodyMusicVolume
  /** @minimum 0 */
  outputFps: number
  outputResolution: [number, number]
  padding?: CreateVideoBodyPadding
  selectedMarkers: SelectedMarker[]
  songIds: number[]
  videoCodec: VideoCodec
  videoId: string
  videoQuality: VideoQuality
}

export type CreateMarkerPreviewImagePath = string | null

export type CreateMarkerMarkerStashId = number | null

export type CreateMarkerCreatedOn = number | null

export interface CreateMarker {
  createdOn?: CreateMarkerCreatedOn
  end: number
  indexWithinVideo: number
  markerStashId?: CreateMarkerMarkerStashId
  previewImagePath?: CreateMarkerPreviewImagePath
  start: number
  title: string
  videoId: string
  videoInteractive: boolean
}

export interface CreateMarkerRequest {
  createInStash: boolean
  marker: CreateMarker
}

export type CreateInteractiveClipsBodySeed = string | null

export interface CreateInteractiveClipsBody {
  clipDuration: number
  order: ClipOrder
  query: InteractiveClipsQuery
  seed?: CreateInteractiveClipsBodySeed
}

export interface CreateFunscriptBody {
  clips: Clip[]
}

export type CreateClipsBodySeed = string | null

export interface CreateClipsBody {
  clips: ClipOptions
  markers: SelectedMarker[]
  seed?: CreateClipsBodySeed
}

export interface CreateBeatFunscriptBody {
  songIds: number[]
  strokeType: StrokeType
}

export interface ControllerStatus {
  /** @minimum 0 */
  currentVelocity: number
  elapsed: number
  paused: boolean
}

export type ClipsResponseStreams = {[key: string]: string}

export type ClipsResponseBeatOffsets = number[] | null

export interface ClipsResponse {
  beatOffsets?: ClipsResponseBeatOffsets
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

export interface Clip {
  /** @minimum 0 */
  indexWithinMarker: number
  /** @minimum 0 */
  indexWithinVideo: number
  markerId: number
  markerTitle: string
  /** Start and endpoint inside the video in seconds. */
  range: [number, number]
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

export type AddVideosRequestOneOfSevenType =
  (typeof AddVideosRequestOneOfSevenType)[keyof typeof AddVideosRequestOneOfSevenType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const AddVideosRequestOneOfSevenType = {
  stash: "stash",
} as const

export type AddVideosRequestOneOfSeven = {
  sceneIds: number[]
  type: AddVideosRequestOneOfSevenType
}

export type AddVideosRequestOneOfFourType =
  (typeof AddVideosRequestOneOfFourType)[keyof typeof AddVideosRequestOneOfFourType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const AddVideosRequestOneOfFourType = {
  download: "download",
} as const

export type AddVideosRequestOneOfFourTags = string[] | null

export type AddVideosRequestOneOfFour = {
  tags?: AddVideosRequestOneOfFourTags
  type: AddVideosRequestOneOfFourType
  urls: string[]
}

export type AddVideosRequestOneOfType =
  (typeof AddVideosRequestOneOfType)[keyof typeof AddVideosRequestOneOfType]

// eslint-disable-next-line @typescript-eslint/no-redeclare
export const AddVideosRequestOneOfType = {
  local: "local",
} as const

export type AddVideosRequestOneOfTags = string[] | null

export type AddVideosRequestOneOf = {
  path: string
  recurse: boolean
  tags?: AddVideosRequestOneOfTags
  type: AddVideosRequestOneOfType
}

export type AddVideosRequest =
  | AddVideosRequestOneOf
  | AddVideosRequestOneOfFour
  | AddVideosRequestOneOfSeven

export interface AccellerateParameters {
  endSpeed: number
  sessionDuration: number
  slideRange: Range
  startSpeed: number
}

/**
 * @summary Get the current status of the handy
 */
export const handyStatus = () => {
  return customInstance<HandyStatus200>({url: `/api/handy`, method: "GET"})
}

/**
 * @summary Get the connection status of the handy
 */
export const handyConnected = () => {
  return customInstance<HandyConnectedResponse>({
    url: `/api/handy/connected`,
    method: "GET",
  })
}

/**
 * @summary Pause the handy's movement
 */
export const pauseHandy = () => {
  return customInstance<unknown>({url: `/api/handy/pause`, method: "POST"})
}

/**
 * @summary Start the handy with the given pattern and key.
 */
export const startHandy = (startHandyParameters: StartHandyParameters) => {
  return customInstance<unknown>({
    url: `/api/handy/start`,
    method: "POST",
    headers: {"Content-Type": "application/json"},
    data: startHandyParameters,
  })
}

/**
 * @summary Stop the handy's movement
 */
export const stopHandy = () => {
  return customInstance<unknown>({url: `/api/handy/stop`, method: "POST"})
}

/**
 * @summary Deletes all generated files in the specified folder.
 */
export const cleanupFolder = (folderType: FolderType) => {
  return customInstance<unknown>({
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

/**
 * @summary Lists all markers for a set of video IDs.
 */
export const listMarkers = (params?: ListMarkersParams) => {
  return customInstance<MarkerDto[]>({
    url: `/api/library/marker`,
    method: "GET",
    params,
  })
}

/**
 * @summary Creates a new marker for a video.
 */
export const createNewMarker = (createMarkerRequest: CreateMarkerRequest) => {
  return customInstance<MarkerDto>({
    url: `/api/library/marker`,
    method: "POST",
    headers: {"Content-Type": "application/json"},
    data: createMarkerRequest,
  })
}

/**
 * @summary Lists marker titles and nunber of occurrences
 */
export const listMarkerTitles = (params?: ListMarkerTitlesParams) => {
  return customInstance<MarkerCount[]>({
    url: `/api/library/marker/title`,
    method: "GET",
    params,
  })
}

/**
 * @summary Update a marker, additionally updates the marker in Stash if applicable and desired.
 */
export const updateMarker = (id: number, updateMarker: UpdateMarker) => {
  return customInstance<MarkerDto>({
    url: `/api/library/marker/${id}`,
    method: "PUT",
    headers: {"Content-Type": "application/json"},
    data: updateMarker,
  })
}

/**
 * @summary Deletes a marker.
 */
export const deleteMarker = (id: number) => {
  return customInstance<unknown>({
    url: `/api/library/marker/${id}`,
    method: "DELETE",
  })
}

/**
 * @summary Splits a marker into two at the specified time.
 */
export const splitMarker = (id: number, params: SplitMarkerParams) => {
  return customInstance<MarkerDto[]>({
    url: `/api/library/marker/${id}/split`,
    method: "POST",
    params,
  })
}

export const migratePreviewImages = () => {
  return customInstance<unknown>({
    url: `/api/library/migrate/preview`,
    method: "POST",
  })
}

/**
 * @summary Lists all performers from videos and their number of markers
 */
export const listPerformers = (params?: ListPerformersParams) => {
  return customInstance<ListPerformerResponse[]>({
    url: `/api/library/performers`,
    method: "GET",
    params,
  })
}

export const getFileStats = () => {
  return customInstance<[GetFileStats200ItemItem0, number][]>({
    url: `/api/library/stats`,
    method: "GET",
  })
}

/**
 * @summary Lists videos (paginated, with search)
 */
export const listVideos = (params?: ListVideosParams) => {
  return customInstance<PageListVideoDto>({
    url: `/api/library/video`,
    method: "GET",
    params,
  })
}

/**
 * @summary Adds new videos either via stash, local files or URL (to download)
 */
export const addNewVideos = (addVideosRequest: AddVideosRequest) => {
  return customInstance<VideoDto[]>({
    url: `/api/library/video`,
    method: "POST",
    headers: {"Content-Type": "application/json"},
    data: addVideosRequest,
  })
}

/**
 * @summary Removes videos that don't exist on disk
 */
export const cleanupVideos = () => {
  return customInstance<VideoCleanupResponse>({
    url: `/api/library/video/cleanup`,
    method: "POST",
  })
}

/**
 * @summary Returns whether a set of videos need to be re-encoded or not
 */
export const videosNeedEncoding = (videosNeedEncodingBody: string[]) => {
  return customInstance<boolean>({
    url: `/api/library/video/need-encoding`,
    method: "POST",
    headers: {"Content-Type": "application/json"},
    data: videosNeedEncodingBody,
  })
}

/**
 * @summary Lists videos on the configured Stash instance
 */
export const listStashVideos = (params?: ListStashVideosParams) => {
  return customInstance<PageStashVideoDto>({
    url: `/api/library/video/stash`,
    method: "GET",
    params,
  })
}

export const listVideoTags = () => {
  return customInstance<TagCount[]>({
    url: `/api/library/video/tags`,
    method: "GET",
  })
}

/**
 * @summary Gets details on a single video
 */
export const getVideo = (id: string) => {
  return customInstance<VideoDetailsDto>({
    url: `/api/library/video/${id}`,
    method: "GET",
  })
}

/**
 * @summary Updates video metadata
 */
export const updateVideo = (id: string, videoUpdate: VideoUpdate) => {
  return customInstance<unknown>({
    url: `/api/library/video/${id}`,
    method: "PUT",
    headers: {"Content-Type": "application/json"},
    data: videoUpdate,
  })
}

/**
 * @summary Deletes a video
 */
export const deleteVideo = (id: string) => {
  return customInstance<unknown>({
    url: `/api/library/video/${id}`,
    method: "DELETE",
  })
}

/**
 * @summary Tries to detect markers in a video by detecting scene changes.
 */
export const detectMarkers = (id: string, params?: DetectMarkersParams) => {
  return customInstance<MarkerDto[]>({
    url: `/api/library/video/${id}/detect-markers`,
    method: "POST",
    params,
  })
}

/**
 * @summary Synchronizes a single video with stash
 */
export const mergeStashVideo = (id: string) => {
  return customInstance<ListVideoDto>({
    url: `/api/library/video/${id}/stash/merge`,
    method: "POST",
  })
}

export const deleteProgress = (id: string) => {
  return customInstance<unknown>({url: `/api/progress/${id}`, method: "DELETE"})
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
  return customInstance<number[]>({
    url: `/api/project/download`,
    method: "GET",
    params,
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

/**
 * @summary Generate a possible random seed (a random word)
 */
export const generateRandomSeed = () => {
  return customInstance<string>({
    url: `/api/project/random-seed`,
    method: "GET",
  })
}

/**
 * @summary List all songs
 */
export const listSongs = (params?: ListSongsParams) => {
  return customInstance<SongDto[]>({url: `/api/song`, method: "GET", params})
}

export const downloadMusic = (params: DownloadMusicParams) => {
  return customInstance<SongDto>({
    url: `/api/song/download`,
    method: "POST",
    params,
  })
}

/**
 * @summary Upload a song file
 */
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

/**
 * @summary Get beats for a song, or detect them if they are not yet available.
 */
export const getBeats = (id: number) => {
  return customInstance<Beats>({url: `/api/song/${id}/beats`, method: "GET"})
}

export const getStashHealth = (params: GetStashHealthParams) => {
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
  return customInstance<unknown>({
    url: `/api/system/configuration`,
    method: "POST",
    headers: {"Content-Type": "application/json"},
    data: settings,
  })
}

export const getAppHealth = () => {
  return customInstance<unknown>({url: `/api/system/health`, method: "GET"})
}

export const restart = () => {
  return customInstance<unknown>({url: `/api/system/restart`, method: "POST"})
}

export const getVersion = () => {
  return customInstance<AppVersion>({url: `/api/system/version`, method: "GET"})
}

export type HandyStatusResult = NonNullable<
  Awaited<ReturnType<typeof handyStatus>>
>
export type HandyConnectedResult = NonNullable<
  Awaited<ReturnType<typeof handyConnected>>
>
export type PauseHandyResult = NonNullable<
  Awaited<ReturnType<typeof pauseHandy>>
>
export type StartHandyResult = NonNullable<
  Awaited<ReturnType<typeof startHandy>>
>
export type StopHandyResult = NonNullable<Awaited<ReturnType<typeof stopHandy>>>
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
export type ListPerformersResult = NonNullable<
  Awaited<ReturnType<typeof listPerformers>>
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
export type ListVideoTagsResult = NonNullable<
  Awaited<ReturnType<typeof listVideoTags>>
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
export type GenerateRandomSeedResult = NonNullable<
  Awaited<ReturnType<typeof generateRandomSeed>>
>
export type ListSongsResult = NonNullable<Awaited<ReturnType<typeof listSongs>>>
export type DownloadMusicResult = NonNullable<
  Awaited<ReturnType<typeof downloadMusic>>
>
export type UploadMusicResult = NonNullable<
  Awaited<ReturnType<typeof uploadMusic>>
>
export type GetBeatsResult = NonNullable<Awaited<ReturnType<typeof getBeats>>>
export type GetStashHealthResult = NonNullable<
  Awaited<ReturnType<typeof getStashHealth>>
>
export type GetConfigResult = NonNullable<Awaited<ReturnType<typeof getConfig>>>
export type SetConfigResult = NonNullable<Awaited<ReturnType<typeof setConfig>>>
export type GetAppHealthResult = NonNullable<
  Awaited<ReturnType<typeof getAppHealth>>
>
export type RestartResult = NonNullable<Awaited<ReturnType<typeof restart>>>
export type GetVersionResult = NonNullable<
  Awaited<ReturnType<typeof getVersion>>
>
