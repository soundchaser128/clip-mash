import clsx from "clsx"
import {StashConfig, ListVideoDto, VideoDto} from "../api"
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
import EditableText from "./EditableText"

interface Props {
  video: ListVideoDto
  stashConfig?: StashConfig
  actionChildren?: React.ReactNode
  onEditTitle: (title: string) => void
}

function getPreview(video: VideoDto, config?: StashConfig): string {
  if (video.source === "Stash" && config) {
    return `${config.stashUrl}/scene/${video.stashSceneId!}/screenshot?apikey=${
      config.apiKey
    }`
  } else {
    return `/api/library/video/${video.id}/preview`
  }
}

const VideoCard: React.FC<Props> = ({
  video,
  stashConfig,
  actionChildren,
  onEditTitle,
}) => {
  const tags = video.video.tags?.filter(Boolean) ?? []
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
          width={499}
          height={281}
        />
      </figure>
      <div className="card-body">
        <h2 className="card-title">
          <EditableText
            value={video.video.title || video.video.fileName}
            onSave={onEditTitle}
          />
        </h2>
        <ul className="flex flex-col gap-2 self-start">
          {tags.length > 0 && (
            <li>
              <span className="inline-flex flex-wrap gap-y-1 gap-x-0.5 ">
                {tags.map((tag) => (
                  <span key={tag} className="badge">
                    {tag}
                  </span>
                ))}
              </span>
            </li>
          )}
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
            Source: <strong>{video.video.source}</strong>
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
