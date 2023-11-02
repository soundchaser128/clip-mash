import clsx from "clsx"
import {StashConfig, ListVideoDto, VideoDto} from "../api"
import {
  HiAdjustmentsVertical,
  HiArrowDownTray,
  HiCalendar,
  HiCheck,
  HiClock,
  HiTag,
  HiXMark,
} from "react-icons/hi2"
import {dateTimeFormat, formatSeconds} from "../helpers"
import React from "react"
import EditableText from "./EditableText"

interface Props {
  video: ListVideoDto
  stashConfig?: StashConfig
  actionChildren?: React.ReactNode
  onEditTitle?: (title: string) => void
  onImageClick?: (id: string) => void
  disabled?: boolean
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
  onImageClick,
  disabled,
}) => {
  const tags = video.video.tags?.filter(Boolean) ?? []
  const date = new Date(video.video.createdOn * 1000)
  const isoDate = date.toISOString()
  const humanDate = dateTimeFormat.format(date)
  return (
    <article
      className={clsx(
        "card card-compact shadow-xl bg-base-200",
        video.markerCount > 0 && "ring-4 ring-green-500",
        disabled && "opacity-50",
      )}
    >
      <figure>
        <img
          className={clsx(
            "aspect-[16/9] object-cover w-full",
            onImageClick && "cursor-pointer",
            disabled && "grayscale",
          )}
          src={getPreview(video.video, stashConfig)}
          width={499}
          height={281}
          onClick={() => onImageClick && onImageClick(video.video.id)}
        />
      </figure>
      <div className="card-body">
        <h2 className="card-title gap-0">
          {onEditTitle && (
            <EditableText
              value={video.video.title || video.video.fileName}
              onSave={onEditTitle}
            />
          )}
          {!onEditTitle && (
            <span className="truncate">
              {video.video.title || video.video.fileName}
            </span>
          )}
        </h2>
        <ul className="flex flex-col gap-2 self-start">
          <li>
            {tags.length > 0 && (
              <span className="inline-flex flex-wrap gap-y-1 gap-x-0.5 ">
                {tags.map((tag) => (
                  <span key={tag} className="badge">
                    {tag}
                  </span>
                ))}
              </span>
            )}
            {tags.length === 0 && (
              <span className="text-gray-400">No tags</span>
            )}
          </li>
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
            Markers: <strong>{video.markerCount}</strong>
          </li>
          <li>
            <HiArrowDownTray className="inline mr-2" />
            Source: <strong>{video.video.source}</strong>
          </li>
          <li>
            <HiClock className="inline mr-2" />
            Duration: <strong>{formatSeconds(video.video.duration)}</strong>
          </li>
          <li>
            <HiCalendar className="inline mr-2" />
            Created:{" "}
            <strong>
              <time dateTime={isoDate}>{humanDate}</time>
            </strong>
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
