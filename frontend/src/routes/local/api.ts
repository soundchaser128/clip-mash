import {Result} from "@badrap/result"
import {parseTimestamp} from "../../helpers"
import {JsonError} from "../../types/types"
import {
  CreateMarker,
  MarkerDto,
  UpdateMarker,
  VideoDto,
  createNewMarker,
  updateMarker as updateMarkerApi,
} from "../../api"

export interface MarkerInputs {
  title: string
  start: string | number
  end?: string | number
}

export async function createMarker(
  videoDto: VideoDto,
  marker: MarkerInputs,
  duration: number,
  index: number,
): Promise<Result<MarkerDto, JsonError>> {
  const start = Math.max(parseTimestamp(marker.start), 0)
  const end = Math.min(parseTimestamp(marker.end!), duration)
  const payload = {
    start,
    end,
    title: marker.title.trim(),
    videoId: videoDto.id.id,
    indexWithinVideo: index,
    previewImagePath: null,
    videoInteractive: videoDto.interactive,
  } satisfies CreateMarker

  try {
    const marker = await createNewMarker(payload)
    return Result.ok(marker)
  } catch (e) {
    const error = e as JsonError
    return Result.err(error)
  }
}

export async function updateMarker(
  id: number,
  marker: MarkerInputs,
): Promise<Result<MarkerDto, JsonError>> {
  const payload = {
    rowid: id,
    start: parseTimestamp(marker.start),
    end: parseTimestamp(marker.end!),
    title: marker.title.trim(),
  } satisfies UpdateMarker

  try {
    const marker = await updateMarkerApi(payload)
    return Result.ok(marker)
  } catch (e) {
    const error = e as JsonError
    return Result.err(error)
  }
}
