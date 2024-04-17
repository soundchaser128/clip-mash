import HoverVideoPlayer from "react-hover-video-player"
import Loader from "./Loader"
import clsx from "clsx"

interface Props {
  videoSource: string
  imageSource: string
  onImageClick?: () => void
  disabled?: boolean
  className?: string
  overlay?: JSX.Element
}

export default function HoverVideo({
  videoSource,
  imageSource,
  onImageClick,
  disabled,
  className,
  overlay,
}: Props) {
  const classes = clsx(className, "aspect-[16/9] object-cover w-full h-full", {
    grayscale: disabled,
    "cursor-pointer": onImageClick,
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
