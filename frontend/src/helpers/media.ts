import {StashConfig, VideoDto} from "@/api"
import {VideoSource} from "@/components/VideoPlayer"

function stashVideoSources(
  video: VideoDto,
  config: StashConfig,
): VideoSource[] {
  const baseUrl = new URL(
    `${config.stashUrl}/scene/${video.stashSceneId!}/stream.mp4`,
  )
  if (config.apiKey) {
    baseUrl.searchParams.append("apikey", config.apiKey)
  }

  const trancodeUrl = new URL(baseUrl)
  trancodeUrl.searchParams.append("resolution", "ORIGINAL")
  return [
    {
      src: baseUrl.toString(),
      type: "video/mp4",
    },
    {
      src: trancodeUrl.toString(),
      type: "video/mp4",
    },
  ]
}

export function getVideoSources(
  video: VideoDto,
  config?: StashConfig,
): VideoSource[] {
  if (video.source === "Stash" && config) {
    return stashVideoSources(video, config)
  } else {
    return [
      {
        src: `/api/library/video/${video.id}/file`,
        type: "video/mp4",
      },
    ]
  }
}
