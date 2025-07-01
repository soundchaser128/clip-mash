import {Clip} from "@/api"
import {VideoSource} from "@/components/VideoPlayer"
import {BASE_URL} from "@/custom-client"

export function getClipUrl(
  streams: Record<string, string>,
  currentClip: Clip | undefined,
): VideoSource[] | undefined {
  if (!currentClip) {
    return undefined
  } else {
    const streamUrl = streams[currentClip.videoId]
    console.log(streams, currentClip)
    const hash = `t=${currentClip.range[0]},${currentClip.range[1]}`
    if (streamUrl.startsWith("/")) {
      return [
        {
          src: BASE_URL + streamUrl + "#" + hash,
          type: "video/mp4",
        },
      ]
    } else {
      const transcodeUrl = new URL(streamUrl)
      transcodeUrl.searchParams.append("resolution", "ORIGINAL")
      transcodeUrl.hash = hash
      return [
        {
          src: streamUrl + "#" + hash,
          type: "video/mp4",
        },
        {
          src: transcodeUrl.toString(),
          type: "video/mp4",
        },
      ]
    }
  }
}
