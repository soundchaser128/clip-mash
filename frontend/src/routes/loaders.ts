import {LoaderFunction, json} from "react-router-dom"
import {getFormState} from "../helpers"
import invariant from "tiny-invariant"
import {FormState, StateHelpers} from "../types/types"
import {
  Clip,
  ClipPickerOptions,
  ClipsResponse,
  CreateClipsBody,
  NewId,
  PmvClipOptions,
  SongDto,
  VideoDto,
} from "../types.generated"

export const configLoader: LoaderFunction = async () => {
  const response = await fetch("/api/stash/config")
  if (response.ok) {
    const config = await response.json()
    return config
  } else {
    const error = await response.text()
    throw json({error, request: "/api/stash/config"}, {status: 500})
  }
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
      weights: state.clipWeights!,
      clipLengths: getClipLengths(state),
      length:
        state.songs && state.songs.length > 0
          ? state.songs.reduce((sum, song) => sum + song.duration, 0)
          : state.selectedMarkers!.reduce(
              (sum, {selectedRange: [start, end]}) => sum + (end - start),
              0
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

  const response = await fetch("/api/clips", {
    method: "POST",
    body: JSON.stringify(body),
    headers: {"content-type": "application/json"},
  })
  if (response.ok) {
    const data: ClipsResponse = await response.json()

    const videos: Record<string, VideoDto> = {}
    data.videos.forEach((s) => {
      videos[s.id.id] = s
    })

    return {
      ...data,
      videos,
    } // satisfies ClipsLoaderData
  } else {
    const text = await response.text()
    throw json({error: text, request: "/api/clips"}, {status: 500})
  }
}

export const localMarkerLoader: LoaderFunction = async ({request}) => {
  const formState = getFormState()!
  invariant(StateHelpers.isLocalFiles(formState))
  const params = new URL(request.url).search

  const response = await fetch(`/api/local/video/marker${params}`)
  if (response.ok) {
    const json = await response.json()
    return json
  } else {
    const text = await response.text()
    throw json({error: text, request: "/api/local/video/marker"}, {status: 500})
  }
}

export const loadNewId = async () => {
  const response = await fetch("/api/id")
  if (response.ok) {
    const data = (await response.json()) as NewId
    return data.id
  } else {
    const text = await response.text()
    throw json({error: text, request: "/api/id"}, {status: 500})
  }
}

export const newIdLoader: LoaderFunction = async () => {
  const id = await loadNewId()

  return id
}

export const videoDetailsLoader: LoaderFunction = async ({params}) => {
  const {id} = params
  const response = await fetch(`/api/local/video/${id}`)
  if (response.ok) {
    const data = await response.json()
    return data
  } else {
    const text = await response.text()
    throw json({error: text, request: `/api/video/${id}`}, {status: 500})
  }
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
