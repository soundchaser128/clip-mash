import {LoaderFunction, json} from "react-router-dom"
import {getFormState} from "../helpers"
import invariant from "tiny-invariant"
import {
  LocalVideosFormState,
  StashFormState,
  StateHelpers,
} from "../types/types"
import {
  Clip,
  ClipPickerOptions,
  ClipsResponse,
  CreateClipsBody,
  NewId,
  PmvClipOptions,
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

const getClipLengths = (
  state: LocalVideosFormState | StashFormState
): PmvClipOptions => {
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

const getClipSettings = (
  state: LocalVideosFormState | StashFormState
): ClipPickerOptions => {
  const options: Partial<ClipPickerOptions> = {
    type: state.clipStrategy || "roundRobin",
  }

  if (options.type === "weightedRandom") {
    options.weights = state.clipWeights
    options.clipLengths = getClipLengths(state)
  }

  if (options.type === "equalLength") {
    options.clipDuration = state.clipDuration || 30
    options.divisors = [2, 3, 4]
  }

  if (options.type === "roundRobin") {
    options.clipLengths = getClipLengths(state)
    if (state.songs && state.songs.length) {
      options.length = state.songs.reduce((sum, song) => sum + song.duration, 0)
    }
  }

  return options
}

export interface ClipsLoaderData {
  clips: Clip[]
  streams: Record<string, string>
  videos: Record<string, VideoDto>
  beatOffsets?: number[]
}

export const clipsLoader: LoaderFunction = async () => {
  const state = getFormState()!
  invariant(StateHelpers.isNotInitial(state))

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

export const localMarkerLoader: LoaderFunction = async () => {
  const formState = getFormState()!
  invariant(StateHelpers.isLocalFiles(formState))
  const videoIds = formState.videos?.map((v) => v.video.id.id).join(",") || ""
  const params = new URLSearchParams({ids: videoIds})

  const response = await fetch(`/api/local/video/marker?${params.toString()}`)
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
  const data = (await response.json()) as NewId
  return data.id
}

export const newIdLoader: LoaderFunction = async () => {
  const id = await loadNewId()

  return id
}
