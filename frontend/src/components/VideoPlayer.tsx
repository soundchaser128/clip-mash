import useHotkeys from "@/hooks/useHotkeys"
import clsx from "clsx"
import {useEffect} from "react"
import {useRef} from "react"
import {HiPause, HiPlay, HiSpeakerWave, HiSpeakerXMark} from "react-icons/hi2"
import {hookstate, useHookstate} from "@hookstate/core"
import {subscribable} from "@hookstate/subscribable"

interface Props {
  src: string
  className?: string
  wrapperClassName?: string
  autoPlay?: boolean
}

interface PlayerState {
  isPlaying: boolean
  isMuted: boolean
  currentTime: number
  duration: number
  playbackRate: number
}

const initialState: PlayerState = {
  isPlaying: false,
  isMuted: false,
  currentTime: 0,
  duration: 0,
  playbackRate: 1,
}

const playerState = hookstate(initialState, subscribable())

export function usePlayer() {
  return useHookstate(playerState)
}

export function Player({src, className, ...rest}: Props) {
  const videoRef = useRef<HTMLVideoElement>(null)
  const state = useHookstate(playerState)

  useEffect(() => {
    state.isPlaying.subscribe((isPlaying) => {
      if (videoRef.current) {
        if (isPlaying) {
          videoRef.current.play()
        } else {
          videoRef.current.pause()
        }
      }
    })
  }, [])

  useEffect(() => {
    state.isMuted.subscribe((isMuted) => {
      if (videoRef.current) {
        videoRef.current.muted = isMuted
      }
    })
  }, [])

  const onLoadedMetadata = () => {
    if (videoRef.current) {
      state.merge({
        duration: videoRef.current.duration,
      })
    }
  }

  const onTimeUpdate = () => {
    if (videoRef.current) {
      state.merge({
        currentTime: videoRef.current.currentTime,
      })
    }
  }

  return (
    <video
      {...rest}
      onLoadedMetadata={onLoadedMetadata}
      onTimeUpdate={onTimeUpdate}
      className={className}
      src={src}
      ref={videoRef}
      muted
    />
  )
}

export function PlayerControls() {
  const state = useHookstate(playerState)

  const onTogglePlay = () => {
    state.merge((s) => ({isPlaying: !s.isPlaying}))
  }

  const onToggleMuted = () => {
    state.merge((s) => ({isMuted: !s.isMuted}))
  }

  const onJump = (seconds: number) => {
    state.merge((s) => ({currentTime: s.currentTime + seconds}))
  }

  useHotkeys("space", onTogglePlay)
  useHotkeys("v m", onToggleMuted)
  useHotkeys("k", () => onJump(-0.5))
  useHotkeys("l", () => onJump(0.5))
  useHotkeys("right", () => onJump(5))
  useHotkeys("left", () => onJump(-5))

  return (
    <>
      <button
        onClick={onTogglePlay}
        className={clsx("btn btn-square", {
          "btn-success": !state.isPlaying,
          "btn-neutral": state.isPlaying,
        })}
        type="button"
      >
        {state.isPlaying ? (
          <HiPause className="w-5 h-5" />
        ) : (
          <HiPlay className="w-5 h-5" />
        )}
      </button>
      <button onClick={onToggleMuted} className="btn btn-square" type="button">
        {state.isMuted ? (
          <HiSpeakerWave className="w-5 h-5" />
        ) : (
          <HiSpeakerXMark className="w-5 h-5" />
        )}
      </button>
    </>
  )
}
