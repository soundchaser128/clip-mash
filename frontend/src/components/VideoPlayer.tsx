import useHotkeys from "@/hooks/useHotkeys"
import clsx from "clsx"
import {useRef, useState} from "react"
import {HiPause, HiPlay, HiSpeakerWave, HiSpeakerXMark} from "react-icons/hi2"

interface Props extends React.HTMLAttributes<HTMLVideoElement> {
  src: string
  className?: string
  wrapperClassName?: string
  extraControls?: React.ReactNode
}

export default function VideoPlayer({
  src,
  className,
  extraControls,
  wrapperClassName,
  ...rest
}: Props) {
  const videoRef = useRef<HTMLVideoElement>(null)
  const [isMuted, setIsMuted] = useState(true)
  const isPlaying = videoRef.current?.paused === false

  const onTogglePlay = () => {
    if (videoRef.current) {
      if (videoRef.current.paused) {
        videoRef.current.play()
      } else {
        videoRef.current.pause()
      }
    }
  }

  const onToggleMuted = () => {
    setIsMuted((prev) => !prev)
  }

  const onJump = (seconds: number) => {
    if (videoRef.current) {
      videoRef.current.currentTime += seconds
    }
  }

  useHotkeys("space", onTogglePlay)
  useHotkeys("v m", onToggleMuted)
  useHotkeys("k", () => onJump(-0.5))
  useHotkeys("l", () => onJump(0.5))
  useHotkeys("right", () => onJump(5))
  useHotkeys("left", () => onJump(-5))

  return (
    <div className={wrapperClassName}>
      <video
        {...rest}
        className={className}
        src={src}
        ref={videoRef}
        muted={isMuted}
      />
      <div className="flex gap-2 items-center w-full">
        <button
          onClick={onTogglePlay}
          className={clsx("btn btn-square", {
            "btn-success": !isPlaying,
            "btn-neutral": isPlaying,
          })}
          type="button"
        >
          {isPlaying ? (
            <HiPause className="w-5 h-5" />
          ) : (
            <HiPlay className="w-5 h-5" />
          )}
        </button>
        <button
          onClick={onToggleMuted}
          className="btn btn-square"
          type="button"
        >
          {isMuted ? (
            <HiSpeakerWave className="w-5 h-5" />
          ) : (
            <HiSpeakerXMark className="w-5 h-5" />
          )}
        </button>

        {extraControls}
      </div>
    </div>
  )
}
