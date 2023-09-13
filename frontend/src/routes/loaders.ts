import {LoaderFunction} from "react-router-dom"
import {getFormState} from "../helpers"
import {
  Clip,
  ClipPickerOptions,
  ClipsResponse,
  CreateClipsBody,
  PmvClipOptions,
  SongDto,
  VideoDto,
  fetchClips,
} from "../api"
import {FormState} from "../types/form-state"
import {getConfig, getNewId, getVideo, listMarkers} from "../api"

export const configLoader: LoaderFunction = async () => {
  return await getConfig()
}

const getClipLengths = (state: FormState): PmvClipOptions => {
  if (state.songs && state.songs.length) {
    return {
      type: "songs",
      beatsPerMeasure: state.beatsPerMeasure || 4,
      cutAfterMeasures: state.cutAfterMeasures || {type: "fixed", count: 4},
      songs: state.songs.map((song) => ({
        length: song.duration,
        offsets: song.beats,
      })),
    }
  } else {
    return {
      type: "randomized",
      baseDuration: state.clipDuration || 30,
      divisors: [2, 3, 4],
    }
  }
}

const getClipSettings = (state: FormState): ClipPickerOptions => {
  if (state.clipStrategy === "weightedRandom") {
    return {
      type: "weightedRandom",
      // @ts-expect-error form state needs to align with this
      weights: state.clipWeights!,
      clipLengths: getClipLengths(state),
      length:
        state.songs && state.songs.length > 0
          ? state.songs.reduce((sum, song) => sum + song.duration, 0)
          : state.selectedMarkers!.reduce(
              (sum, {selectedRange: [start, end]}) => sum + (end - start),
              0,
            ),
    }
  } else if (state.clipStrategy === "equalLength") {
    return {
      type: "equalLength",
      clipDuration: state.clipDuration || 30,
      divisors: [2, 3, 4],
    }
  } else if (
    state.clipStrategy === "roundRobin" &&
    state.songs &&
    state.songs.length > 0
  ) {
    return {
      type: "roundRobin",
      clipLengths: getClipLengths(state),
      length: state.songs.reduce((sum, song) => sum + song.duration, 0),
    }
  } else if (state.clipStrategy === "noSplit") {
    return {type: "noSplit"}
  } else {
    return {
      type: "equalLength",
      clipDuration: state.clipDuration || 30,
      divisors: [2, 3, 4],
    }
  }
}

export interface ClipsLoaderData {
  clips: Clip[]
  streams: Record<string, string>
  videos: Record<string, VideoDto>
  beatOffsets?: number[]
}

export const clipsLoader: LoaderFunction = async () => {
  const state = getFormState()!

  const body = {
    clipOrder: state.clipOrder || "scene-order",
    markers: state.selectedMarkers!.filter((m) => m.selected),
    seed: state.seed || null,
    clips: {
      clipPicker: getClipSettings(state),
      order: state.clipOrder || "scene-order",
    },
  } satisfies CreateClipsBody

  const data: ClipsResponse = await fetchClips(body)

  const videos: Record<string, VideoDto> = {}
  data.videos.forEach((s) => {
    videos[s.id.id] = s
  })

  return {
    ...data,
    videos,
  } // satisfies ClipsLoaderData
}

export const localMarkerLoader: LoaderFunction = async () => {
  const markers = listMarkers()
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
  const response = await fetch("/api/song")
  const data = (await response.json()) as SongDto[]
  return data
}

export const versionLoader: LoaderFunction = async () => {
  const response = await fetch("/api/version")
  const data = (await response.json()) as {version: string}
  return data.version
}
