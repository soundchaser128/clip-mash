import {Result} from "@badrap/result"
import {MarkerDto} from "../../types.generated"
import {parseTimestamp} from "../../helpers"
import {JsonError} from "../../types/types"

export interface CreateMarker {
  videoId: string
  start: number
  end: number
  title: string
  indexWithinVideo: number
}

export interface MarkerInputs {
  title: string
  start: string | number
  end?: string | number
}

export async function persistMarker(
  videoId: string,
  marker: MarkerInputs,
  duration: number,
  index: number
): Promise<Result<MarkerDto, JsonError>> {
  const start = Math.max(parseTimestamp(marker.start), 0)
  const end = Math.min(parseTimestamp(marker.end!), duration)

  const payload = {
    start,
    end,
    title: marker.title.trim(),
    videoId,
    indexWithinVideo: index,
  } satisfies CreateMarker

  const response = await fetch("/api/local/video/marker", {
    method: "POST",
    body: JSON.stringify(payload),
    headers: {"Content-Type": "application/json"},
  })
  if (response.ok) {
    const marker = (await response.json()) as MarkerDto
    return Result.ok(marker)
  } else {
    const error = (await response.json()) as JsonError
    return Result.err(error)
  }
}
