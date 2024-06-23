import {Clip} from "@/api"

export function getClipUrl(
  streams: Record<string, string>,
  currentClip: Clip | undefined,
) {
  if (!currentClip) {
    return undefined
  } else {
    const streamUrl = streams[currentClip.videoId]
    return `${streamUrl}#t=${currentClip.range[0]},${currentClip.range[1]}`
  }
}
