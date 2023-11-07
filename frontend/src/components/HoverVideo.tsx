import HoverVideoPlayer from "react-hover-video-player"
import Loader from "./Loader"
import clsx from "clsx"

interface Props {
  videoSource: string
  imageSource: string
  onImageClick?: () => void
  disabled?: boolean
}

export default function HoverVideo({
  videoSource,
  imageSource,
  onImageClick,
  disabled,
}: Props) {
  const classes = clsx(
    "aspect-[16/9] object-cover w-full",
    disabled && "grayscale",
  )

  return (
    <HoverVideoPlayer
      className="w-full"
      videoSrc={videoSource}
      pausedOverlay={
        <img onClick={onImageClick} src={imageSource} className={classes} />
      }
      loadingOverlay={<Loader />}
      videoClassName={classes}
      loop
      muted
      preload="none"
      unloadVideoOnPaused
      restartOnPaused
      onClick={onImageClick}
    />
  )
}
