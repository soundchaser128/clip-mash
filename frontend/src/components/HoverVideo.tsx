import HoverVideoPlayer from "react-hover-video-player"
import Loader from "./Loader"
import clsx from "clsx"
import {AspectRatio} from "./VideoCard"

interface Props {
  videoSource: string
  imageSource: string
  onImageClick?: () => void
  disabled?: boolean
  className?: string
  overlay?: JSX.Element
  aspectRatio: AspectRatio
}

export default function HoverVideo({
  videoSource,
  imageSource,
  onImageClick,
  disabled,
  className,
  overlay,
  aspectRatio,
}: Props) {
  const classes = clsx(className, "object-cover w-full h-full", {
    grayscale: disabled,
    "cursor-pointer": onImageClick,
    "aspect-[16/9]": aspectRatio === "wide",
    "aspect-square": aspectRatio === "square",
    "aspect-[9/16]": aspectRatio === "tall",
  })

  return (
    <HoverVideoPlayer
      className={classes}
      videoSrc={videoSource}
      pausedOverlay={
        <img
          width={499}
          height={280}
          onClick={onImageClick}
          src={imageSource}
          className={classes}
        />
      }
      loadingOverlay={<Loader />}
      videoClassName={classes}
      loop
      muted
      preload="none"
      unloadVideoOnPaused
      onClick={onImageClick}
      hoverOverlay={overlay}
    />
  )
}
