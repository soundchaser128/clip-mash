// AUTO-GENERATED by typescript-type-def

export type I64 = number
export type Usize = number
export type StashScene = {
  id: string
  performers: string[]
  imageUrl: string | null
  title: string
  studio: string | null
  tags: string[]
  rating: I64 | null
  interactive: boolean
  markerCount: Usize
}
export type VideoSource = "stash" | "localFile" | "downloadedLocalFile"
export type VideoId =
  | {type: "localFile"; id: string}
  | {type: "stash"; id: string}
export type MarkerId = {type: "localFile"; id: I64} | {type: "stash"; id: I64}
export type F64 = number
export type Clip = {
  source: VideoSource
  videoId: VideoId
  markerId: MarkerId
  /**
   * Start and endpoint inside the video in seconds.
   */
  range: [F64, F64]
  indexWithinVideo: Usize
  indexWithinMarker: Usize
}
export type SelectedMarker = {
  id: MarkerId
  videoId: VideoId
  selectedRange: [F64, F64]
  indexWithinVideo: Usize
  selected: boolean | null
  title: string
  loops: Usize
}
export type VideoResolution = "720" | "1080" | "4K"
export type U32 = number
export type VideoCodec = "av1" | "h264" | "h265"
export type VideoQuality = "low" | "medium" | "high" | "lossless"
export type EncodingEffort = "low" | "medium" | "high"
export type CreateVideoBody = {
  fileName: string
  clips: Clip[]
  selectedMarkers: SelectedMarker[]
  outputResolution: VideoResolution
  outputFps: U32
  songIds: I64[]
  musicVolume: F64 | null
  videoCodec: VideoCodec
  videoQuality: VideoQuality
  encodingEffort: EncodingEffort
}
export type ClipOrder = "random" | "scene-order" | "no-op"
export type RandomizedClipOptions = {baseDuration: F64; divisors: F64[]}
export type MeasureCount =
  | ({type: "fixed"} & {count: Usize})
  | ({type: "random"} & {min: Usize; max: Usize})
export type F32 = number
export type Beats = {offsets: F32[]; length: F32}
export type SongClipOptions = {
  beatsPerMeasure: Usize
  cutAfterMeasures: MeasureCount
  songs: Beats[]
}
export type PmvClipOptions =
  | ({type: "randomized"} & RandomizedClipOptions)
  | ({type: "songs"} & SongClipOptions)
export type RoundRobinClipOptions = {length: F64; clipLengths: PmvClipOptions}
export type WeightedRandomClipOptions = {
  weights: [string, F64][]
  length: F64
  clipLengths: PmvClipOptions
}
export type EqualLengthClipOptions = {clipDuration: F64; divisors: F64[]}
export type ClipPickerOptions =
  | ({type: "roundRobin"} & RoundRobinClipOptions)
  | ({type: "weightedRandom"} & WeightedRandomClipOptions)
  | ({type: "equalLength"} & EqualLengthClipOptions)
  | {type: "noSplit"}
export type ClipOptions = {clipPicker: ClipPickerOptions; order: ClipOrder}
export type CreateClipsBody = {
  clipOrder: ClipOrder
  markers: SelectedMarker[]
  seed: string | null
  clips: ClipOptions
}
export type VideoDto = {
  id: VideoId
  title: string
  performers: string[]
  fileName: string
  interactive: boolean
  source: VideoSource
  duration: F64
}
export type MarkerDto = {
  id: MarkerId
  videoId: VideoId
  primaryTag: string
  streamUrl: string
  start: F64
  end: F64
  sceneTitle: string | null
  performers: string[]
  fileName: string | null
  sceneInteractive: boolean
  tags: string[]
  screenshotUrl: string | null
  indexWithinVideo: Usize
}
export type ListVideoDto = {video: VideoDto; markers: MarkerDto[]}
export type ClipsResponse = {
  clips: Clip[]
  streams: Record<string, string>
  videos: VideoDto[]
  beatOffsets: F32[] | null
}
export type PerformerDto = {
  id: string
  sceneCount: I64
  name: string
  imageUrl: string | null
  tags: string[]
  rating: I64 | null
  favorite: boolean
}
export type TagDto = {name: string; id: string; markerCount: I64}
export type SongDto = {
  songId: I64
  duration: F64
  fileName: string
  url: string
  beats: F32[]
}
export type NewId = {id: string}
export type SortDirection = "asc" | "desc"
export type PageParameters = {
  page: Usize | null
  size: Usize | null
  sort: string | null
  dir: SortDirection | null
}
export type Progress = {
  itemsFinished: F64
  itemsTotal: F64
  done: boolean
  etaSeconds: F64
  message: string
}
export type CreateMarker = {
  videoId: string
  start: F64
  end: F64
  title: string
  indexWithinVideo: I64
  previewImagePath: string | null
  videoInteractive: boolean
}
export type UpdateMarker = {rowid: I64; start: F64; end: F64; title: string}
export type StrokeType =
  | {
      /**
       * Creates a stroke every `n` beats
       */
      everyNth: {n: Usize}
    }
  | {
      /**
       * Steadily accelerates the strokes from `start_strokes_per_beat` to `end_strokes_per_beat`
       */
      accelerate: {start_strokes_per_beat: F32; end_strokes_per_beat: F32}
    }
export type CreateBeatFunscriptBody = {songIds: I64[]; strokeType: StrokeType}