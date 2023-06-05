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
  ClipOptions,
  ClipPickerOptions,
  ClipsResponse,
  CreateClipsBody,
  NewId,
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

const getClipSettings = (
  state: LocalVideosFormState | StashFormState
): ClipPickerOptions => {
  if (state.songs && state.songs.length > 0) {
    const songsLength = state.songs.reduce(
      (sum, song) => sum + song.duration,
      0
    )
    if (state.clipStrategy === "pmv") {
      return {
        type: "roundRobin",
        clipLengths: {
          type: "songs",
          beatsPerMeasure: state.beatsPerMeasure || 4,
          cutAfterMeasures: state.cutAfterMeasures || {type: "fixed", count: 4},
          songs: state.songs.map((s) => ({
            length: s.duration,
            offsets: s.beats,
          })),
        },
        length: songsLength,
      }
    } else {
      return {
        type: "roundRobin",
        clipLengths: {
          type: "randomized",
          baseDuration: state.clipDuration || 30,
          divisors: [2, 3, 4],
        },
        length: null,
      }
    }
  } else {
    if (state.splitClips === false) {
      return {
        type: "noSplit",
      }
    } else {
      return {
        type: "equalLength",
        clipDuration: state.clipDuration || 30,
        divisors: [2, 3, 4],
      }
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
