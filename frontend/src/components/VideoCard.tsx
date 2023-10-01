import clsx from "clsx"
import {Config, ListVideoDto, VideoDto} from "../api"
import {
  HiAdjustmentsVertical,
  HiArrowDownTray,
  HiCheck,
  HiClock,
  HiTag,
  HiXMark,
} from "react-icons/hi2"
import {formatSeconds} from "../helpers"
import React from "react"

interface Props {
  video: ListVideoDto
  stashConfig?: Config
  actionChildren?: React.ReactNode
}

function getPreview(video: VideoDto, config?: Config): string {
  if (video.source === "Stash" && config) {
    return `${config.stashUrl}/scene/${video.stashSceneId!}/screenshot?apikey=${
      config.apiKey
    }`
  } else {
    return `/api/library/video/${video.id}/preview`
  }
}

const VideoCard: React.FC<Props> = ({video, stashConfig, actionChildren}) => {
  return (
    <article
      className={clsx(
        "card card-compact shadow-xl bg-base-200",
        video.markers.length > 0 && "ring-4 ring-green-500",
      )}
    >
      <figure>
        <img
          className="aspect-[16/9] object-cover w-full"
          src={getPreview(video.video, stashConfig)}
        />
      </figure>
      <div className="card-body">
        <h2 className="card-title">
          {video.video.title || video.video.fileName}
        </h2>
        <ul className="flex flex-col gap-2 self-start">
          <li>
            <HiAdjustmentsVertical className="inline mr-2" />
            Interactive:{" "}
            <strong>
              {video.video.interactive ? (
                <HiCheck className="text-green-600 inline" />
              ) : (
                <HiXMark className="text-red-600 inline" />
              )}
            </strong>
          </li>
          <li>
            <HiTag className="inline mr-2" />
            Markers: <strong>{video.markers.length}</strong>
          </li>
          <li>
            <HiArrowDownTray className="inline mr-2" />
            Source: {video.video.source}
          </li>
          <li>
            <HiClock className="inline mr-2" />
            Duration: <strong>{formatSeconds(video.video.duration)}</strong>
          </li>
        </ul>
        <div className="card-actions justify-between grow items-end">
          {actionChildren}
        </div>
      </div>
    </article>
  )
}

export default VideoCard
