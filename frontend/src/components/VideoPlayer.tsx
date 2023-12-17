import useHotkeys from "@/hooks/useHotkeys"
import clsx from "clsx"
import React, {useContext, useEffect, useReducer} from "react"
import {useRef} from "react"
import {HiPause, HiPlay, HiSpeakerWave, HiSpeakerXMark} from "react-icons/hi2"

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
  videoElement?: HTMLVideoElement
}

const initialState: PlayerState = {
  isPlaying: false,
  isMuted: false,
  currentTime: 0,
  duration: 0,
  playbackRate: 1,
}

const PlayerContext = React.createContext<{
  state: PlayerState
  dispatch: React.Dispatch<PlayerAction>
}>({
  state: initialState,
  dispatch: () => {},
})

type PlayerAction =
  | {type: "init"; payload: HTMLVideoElement}
  | {type: "togglePlay"}
  | {type: "toggleMute"}
  | {type: "setDuration"; payload: number}
  | {type: "setCurrentTime"; payload: number}
  | {type: "jump"; payload: number}
  | {type: "setPlaybackRate"; payload: number}

function reducer(state: PlayerState, action: PlayerAction): PlayerState {
  switch (action.type) {
    case "togglePlay":
      if (state.isPlaying) {
        if (state.videoElement) {
          state.videoElement.pause()
        }
        return {...state, isPlaying: false}
      } else {
        if (state.videoElement) {
          state.videoElement.play()
        }
        return {...state, isPlaying: true}
      }
    case "toggleMute":
      if (state.isMuted) {
        if (state.videoElement) {
          state.videoElement.muted = false
        }
        return {...state, isMuted: false}
      } else {
        if (state.videoElement) {
          state.videoElement.muted = true
        }
        return {...state, isMuted: true}
      }

    case "setDuration":
      return {...state, duration: action.payload}
    case "setCurrentTime":
      return {...state, currentTime: action.payload}
    case "jump":
      if (state.videoElement) {
        state.videoElement.currentTime += action.payload
      }
      return {...state, currentTime: state.videoElement!.currentTime}
    case "setPlaybackRate":
      state.videoElement!.playbackRate = action.payload
      return {...state, playbackRate: action.payload}
    case "init":
      return {...state, videoElement: action.payload}
    default:
      return state
  }
}

export function PlayerContextProvider({children}: {children: React.ReactNode}) {
  const [state, dispatch] = useReducer(reducer, initialState)

  return (
    <PlayerContext.Provider value={{state, dispatch}}>
      {children}
    </PlayerContext.Provider>
  )
}

export function usePlayer() {
  return useContext(PlayerContext)
}

export function Player({src, className, ...rest}: Props) {
  const videoRef = useRef<HTMLVideoElement>(null)
  const {dispatch} = usePlayer()

  useEffect(() => {
    if (videoRef.current) {
      dispatch({type: "init", payload: videoRef.current})
    }
  }, [dispatch])

  const onLoadedMetadata = () => {
    if (videoRef.current) {
      dispatch({type: "setDuration", payload: videoRef.current.duration})
    }
  }

  const onTimeUpdate = () => {
    if (videoRef.current) {
      dispatch({type: "setCurrentTime", payload: videoRef.current.currentTime})
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
  const {state, dispatch} = usePlayer()
  const {isPlaying, isMuted} = state

  const onTogglePlay = () => {
    dispatch({type: "togglePlay"})
  }

  const onToggleMuted = () => {
    dispatch({type: "toggleMute"})
  }

  const onJump = (seconds: number) => {
    dispatch({type: "jump", payload: seconds})
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
      <button onClick={onToggleMuted} className="btn btn-square" type="button">
        {isMuted ? (
          <HiSpeakerWave className="w-5 h-5" />
        ) : (
          <HiSpeakerXMark className="w-5 h-5" />
        )}
      </button>
    </>
  )
}
