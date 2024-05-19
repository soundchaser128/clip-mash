import clsx from "clsx"
import {StashConfig, ListVideoDto, VideoDto} from "@/api"
import {
  HiAdjustmentsVertical,
  HiArrowDownTray,
  HiCalendar,
  HiCheck,
  HiClock,
  HiPlus,
  HiTag,
  HiUsers,
  HiXMark,
} from "react-icons/hi2"
import {dateTimeFormat, formatSeconds} from "@/helpers/time"
import React from "react"
import EditableText from "./EditableText"
import HoverVideo from "./HoverVideo"
import {Link, useLocation} from "react-router-dom"
import {searchLink} from "@/helpers/links"
import {pluralize} from "@/helpers/formatting"

export type AspectRatio = "tall" | "square" | "wide"

const Label: React.FC<{
  children: React.ReactNode
  aspectRatio: AspectRatio
}> = ({children, aspectRatio}) => {
  if (aspectRatio === "tall") {
    return null
  } else {
    return <>{children}</>
  }
}

const OverlayText: React.FC<{
  children: React.ReactNode
  className?: string
}> = ({children, className}) => {
  return (
    <span
      style={{textShadow: "0px 0px 8px black"}}
      className={clsx(
        "absolute text-white truncate max-w-full text-xs",
        className,
      )}
    >
      {children}
    </span>
  )
}

interface Props {
  video: ListVideoDto
  stashConfig?: StashConfig
  actionChildren?: React.ReactNode
  onEditTitle?: (title: string) => void
  onImageClick?: (id: string) => void
  disabled?: boolean
  onAddTag?: (video: ListVideoDto) => void
  zoomOnHover?: boolean
  hideDetails?: boolean
  aspectRatio: AspectRatio
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

function getVideo(video: VideoDto, config?: StashConfig): string {
  if (video.source === "Stash" && config) {
    return `${config.stashUrl}/scene/${video.stashSceneId!}/stream?apikey=${
      config.apiKey
    }`
  } else {
    return `/api/library/video/${video.id}/file`
  }
}

const VideoCard: React.FC<Props> = ({
  video,
  stashConfig,
  actionChildren,
  onEditTitle,
  onImageClick,
  disabled,
  onAddTag,
  zoomOnHover,
  hideDetails,
  aspectRatio,
}) => {
  const location = useLocation()

  if (hideDetails) {
    return (
      <HoverVideo
        onImageClick={() => onImageClick && onImageClick(video.video.id)}
        imageSource={getPreview(video.video, stashConfig)}
        videoSource={getVideo(video.video, stashConfig)}
        disabled={disabled}
        aspectRatio={aspectRatio}
        className={clsx(
          "rounded-lg",
          zoomOnHover &&
            "transition-transform duration-150 hover:scale-105 hover:z-40 hover:shadow-2xl",
        )}
        overlay={
          <>
            <OverlayText className="top-2 left-2">
              {video.video.title}
            </OverlayText>
            <OverlayText
              className={clsx({
                "bottom-2 right-2": aspectRatio !== "tall",
                "bottom-2 left-2": aspectRatio === "tall",
              })}
            >
              <HiClock className="inline mr-1" />
              {formatSeconds(video.video.duration)}
            </OverlayText>
            {aspectRatio !== "tall" && (
              <OverlayText className="left-2 bottom-2">
                <HiTag className="inline mr-1" />
                Markers: <strong>{video.markerCount}</strong>
              </OverlayText>
            )}
          </>
        }
      />
    )
  }

  const tags = video.video.tags?.filter(Boolean) ?? []
  const date = new Date(video.video.createdOn * 1000)
  const isoDate = date.toISOString()
  const humanDate = dateTimeFormat.format(date)

  return (
    <article
      className={clsx(
        "card card-compact bg-base-200 shadow-xl animate-in fade-in",
        video.markerCount > 0 && "ring ring-green-500",
        disabled && "opacity-50",
        zoomOnHover &&
          "transition-transform duration-150 hover:scale-105 hover:z-40 hover:shadow-2xl",
      )}
    >
      <figure>
        <HoverVideo
          onImageClick={() => onImageClick && onImageClick(video.video.id)}
          imageSource={getPreview(video.video, stashConfig)}
          videoSource={getVideo(video.video, stashConfig)}
          disabled={disabled}
          aspectRatio={aspectRatio}
        />
      </figure>
      <section className="card-body gap-0">
        <h2 className="card-title">
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
        <ul className="flex flex-col gap-2 self-start mb-2">
          <li className="mb-2 flex items-center">
            {tags.length > 0 && (
              <span className="inline-flex flex-wrap gap-y-1 gap-x-0.5 -ml-2">
                {tags.map((tag) => (
                  <Link
                    to={searchLink(location, tag)}
                    key={tag}
                    className="badge"
                  >
                    {tag}
                  </Link>
                ))}
              </span>
            )}
            {tags.length === 0 && (
              <span className="text-gray-400">No tags</span>
            )}
            {onAddTag && (
              <button
                onClick={() => onAddTag(video)}
                type="button"
                className="btn btn-square btn-success btn-xs ml-2"
              >
                <HiPlus />
              </button>
            )}
          </li>
          <li>
            <HiAdjustmentsVertical className="inline mr-2" />
            <Label aspectRatio={aspectRatio}>Interactive: </Label>
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
            <Label aspectRatio={aspectRatio}>Markers: </Label>
            <strong>{video.markerCount}</strong>
          </li>
          <li>
            <HiArrowDownTray className="inline mr-2" />
            <Label aspectRatio={aspectRatio}>Source: </Label>
            <strong>{video.video.source}</strong>
          </li>
          <li>
            <HiClock className="inline mr-2" />
            <Label aspectRatio={aspectRatio}>Duration: </Label>
            <strong>{formatSeconds(video.video.duration)}</strong>
          </li>
          <li>
            <HiCalendar className="inline mr-2" />
            <Label aspectRatio={aspectRatio}>Created: </Label>
            <strong>
              <time dateTime={isoDate}>{humanDate}</time>
            </strong>
          </li>
          {video.video.performers.length > 0 && (
            <li>
              <HiUsers className="inline mr-2" />
              <Label aspectRatio={aspectRatio}>
                {pluralize("Performer", video.video.performers.length)}:{" "}
              </Label>
              <strong>{video.video.performers.join(", ")}</strong>
            </li>
          )}
        </ul>

        <div
          className={clsx("card-actions justify-between grow", {
            "items-end": aspectRatio !== "tall",
            "items-center": aspectRatio === "tall",
          })}
        >
          {actionChildren}
        </div>
      </section>
    </article>
  )
}

export default VideoCard
