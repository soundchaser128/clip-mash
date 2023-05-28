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
  ClipsResponse,
  CreateClipsBody,
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
): ClipOptions => {
  if (state.songs && state.songs.length > 0) {
    return {
      type: "pmv",
      song_ids: state.songs.map(({songId}) => songId),
      clips: {
        type: "songs",
        beatsPerMeasure: state.beatsPerMeasure || 4,
        cutAfterMeasures: state.cutAfterMeasures || {type: "fixed", count: 4},
      },
    }
  } else {
    if (!state.splitClips) {
      return {
        type: "noSplit",
      }
    } else {
      return {
        type: "default",
        baseDuration: state.clipDuration || 30,
        divisors: [2.0, 3.0, 4.0],
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
    clips: getClipSettings(state),
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
  // todo
  return []
}
