import {LoaderFunction} from "react-router-dom"
import {getFormState} from "@/helpers/form"
import {
  Clip,
  ClipPickerOptions,
  ClipsResponse,
  CreateClipsBody,
  ListVideosParams,
  ClipLengthOptions,
  StashVideoDtoPage,
  VideoDto,
  VideoSource,
  fetchClips,
  getVersion,
  listSongs,
  listStashVideos,
  listVideos,
  getNewId,
  getVideo,
  listMarkers,
  listAlexandriaVideos,
} from "@/api"
import {FormState} from "@/types/form-state"
import {
  ClipFormInputs,
  getDefaultOptions,
} from "./clips/settings/ClipSettingsForm"
import {lerpArrays} from "@/helpers/math"

export const DEFAULT_PAGE_LENGTH = 24
export const DEFAULT_CLIP_BASE_DURATION = 10
export const DEFAULT_SPREAD = 0.25

function getDivisors(spread: number): number[] {
  const MIN_DURATIONS = [1, 1, 1, 1]
  const MAX_DURATIONS = [1, 4, 8, 16]

  return lerpArrays(MIN_DURATIONS, MAX_DURATIONS, spread)
}

export interface ClipsLoaderData {
  clips: Clip[]
  streams: Record<string, string>
  videos: Record<string, VideoDto>
  beatOffsets?: number[]
}

const songsLength = (state: FormState): number => {
  return state.songs?.reduce((len, song) => len + song.duration, 0) || 0
}

const markerLength = (state: FormState): number => {
  return (
    state.markers?.reduce(
      (len, marker) => len + (marker.end - marker.start),
      0,
    ) || 0
  )
}

const getClipLengths = (
  options: {clipLengths?: ClipLengthOptions},
  state: FormState,
  spread: number,
): ClipLengthOptions => {
  if (!options.clipLengths || !options.clipLengths.type) {
    return {
      type: "randomized",
      baseDuration: 20,
      divisors: getDivisors(spread),
    }
  }

  if (options.clipLengths.type === "songs") {
    return {
      type: "songs",
      songs:
        state.songs?.map((s) => ({
          offsets: s.beats,
          length: s.duration,
        })) || [],
      beatsPerMeasure: options.clipLengths.beatsPerMeasure,
      cutAfterMeasures: options.clipLengths.cutAfterMeasures,
    }
  } else {
    return {
      type: "randomized",
      baseDuration: options.clipLengths.baseDuration,
      divisors: getDivisors(spread),
    }
  }
}

const getClipPickerOptions = (
  inputs: ClipFormInputs | undefined,
  state: FormState,
): ClipPickerOptions => {
  if (!inputs) {
    return {
      type: "equalLength",
      clipDuration: 20,
      divisors: getDivisors(DEFAULT_SPREAD),
      minClipDuration: 1.5,
    }
  }

  switch (inputs.clipStrategy) {
    case "roundRobin": {
      const length = state.songs?.length
        ? songsLength(state)
        : inputs?.maxDuration || markerLength(state)
      return {
        type: "roundRobin",
        length,
        clipLengths: getClipLengths(
          inputs.roundRobin,
          state,
          inputs.clipDurationSpread,
        ),
        lenientDuration: !inputs.useMusic,
        minClipDuration: inputs.minClipDuration,
      }
    }
    case "weightedRandom": {
      const length = state.songs?.length
        ? songsLength(state)
        : inputs?.maxDuration || markerLength(state)
      return {
        type: "weightedRandom",
        clipLengths: getClipLengths(
          inputs.weightedRandom,
          state,
          inputs.clipDurationSpread,
        ),
        length,
        // @ts-expect-error type definitions don't align
        weights: state.clipWeights!,
        minClipDuration: inputs.minClipDuration,
      }
    }
    case "equalLength": {
      const length = state.songs?.length
        ? songsLength(state)
        : inputs?.maxDuration
      return {
        type: "equalLength",
        clipDuration: inputs.equalLength.clipDuration,
        divisors: getDivisors(inputs.clipDurationSpread),
        length,
        minClipDuration: inputs.minClipDuration,
      }
    }
    case "noSplit": {
      return {
        type: "noSplit",
      }
    }
  }
}

export const clipsLoader: LoaderFunction = async () => {
  const state = getFormState()!

  const options = state.clipOptions || getDefaultOptions(state)
  const clipOrder = options.clipOrder || {type: "scene"}
  const seed = options.seed?.length === 0 ? undefined : options.seed

  const body = {
    clipOrder,
    markers: state.selectedMarkers!.filter((m) => m.selected),
    seed,
    clips: {
      order: clipOrder,
      clipPicker: getClipPickerOptions(options, state),
    },
  } satisfies CreateClipsBody

  const data: ClipsResponse = await fetchClips(body)

  const videos: Record<string, VideoDto> = {}
  data.videos.forEach((s) => {
    videos[s.id] = s
  })

  return {
    ...data,
    videos,
  }
}

export const localMarkerLoader: LoaderFunction = async () => {
  const state = getFormState()

  const videoIds = (state?.videoIds ?? []).join(",")
  const markers = listMarkers({videoIds})
  return markers
}

export const newIdLoader: LoaderFunction = async () => {
  const data = await getNewId()
  return data.id
}

export const videoDetailsLoader: LoaderFunction = async ({params}) => {
  const {id} = params
  const data = await getVideo(id!)
  return data
}

export const musicLoader: LoaderFunction = async () => {
  const data = await listSongs()

  return data
}

export const versionLoader: LoaderFunction = async () => {
  try {
    return await getVersion()
  } catch (e) {
    console.warn("Failed to get version", e)
    return {
      currentVersion: "0.0.0",
      needsUpdate: false,
      newestVersion: "0.0.0",
    }
  }
}

export type StashLoaderData = StashVideoDtoPage

export const stashVideoLoader: LoaderFunction = async ({request}) => {
  const url = new URL(request.url)
  const query = url.searchParams.get("query")
  const withMarkers = url.searchParams.get("withMarkers") === "true"
  const videos = await listStashVideos({
    query,
    page: Number(url.searchParams.get("page")) || 1,
    size: Number(url.searchParams.get("size")) || DEFAULT_PAGE_LENGTH,
    withMarkers: withMarkers ? true : null,
  })

  return videos
}

function parseBoolean(str: string | null): boolean | null {
  if (str === "true") {
    return true
  } else if (str === "false") {
    return false
  } else {
    return null
  }
}

export const makeVideoLoader: (
  params: Partial<ListVideosParams>,
) => LoaderFunction =
  (params) =>
  async ({request}) => {
    const url = new URL(request.url)
    const query = url.searchParams
    const videos = await listVideos({
      hasMarkers: params.hasMarkers || parseBoolean(query.get("hasMarkers")),
      page: Number(query.get("page")) || 0,
      size: Number(query.get("size")) || DEFAULT_PAGE_LENGTH,
      query: query.get("query"),
      sort: query.get("sort"),
      source: query.get("source") as VideoSource | null,
      isInteractive:
        params.isInteractive || parseBoolean(query.get("isInteractive")),
    })

    return videos
  }

export const alexandriaVideoLoader: LoaderFunction = async ({request}) => {
  const url = new URL(request.url)
  const query = url.searchParams
  const videos = await listAlexandriaVideos({
    page: Number(query.get("page")) || 0,
    size: Number(query.get("size")) || DEFAULT_PAGE_LENGTH,
    query: query.get("query"),
    sort: query.get("sort"),
  })

  return {videos}
}
