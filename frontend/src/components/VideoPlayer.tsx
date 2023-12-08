import useHotkeys from "@/hooks/useHotkeys"
import clsx from "clsx"
import React, {useContext, useReducer} from "react"
import {useRef, useState} from "react"
import {HiPause, HiPlay, HiSpeakerWave, HiSpeakerXMark} from "react-icons/hi2"

interface Props extends React.HTMLAttributes<HTMLVideoElement> {
  src: string
  className?: string
  wrapperClassName?: string
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

const PlayerContext = React.createContext<{
  state: PlayerState
  dispatch: React.Dispatch<PlayerAction>
}>({
  state: initialState,
  dispatch: () => {},
})

type PlayerAction =
  | {type: "play"}
  | {type: "pause"}
  | {type: "mute"}
  | {type: "unmute"}
  | {type: "setDuration"; payload: number}
  | {type: "setCurrentTime"; payload: number}
  | {type: "setPlaybackRate"; payload: number}

export function PlayerContextProvider({children}: {children: React.ReactNode}) {
  const reducer = (state: PlayerState, action: PlayerAction) => {
    switch (action.type) {
      case "play":
        return {...state, isPlaying: true}
      case "pause":
        return {...state, isPlaying: false}
      case "mute":
        return {...state, isMuted: true}
      case "unmute":
        return {...state, isMuted: false}
      case "setDuration":
        return {...state, duration: action.payload}
      case "setCurrentTime":
        return {...state, currentTime: action.payload}
      case "setPlaybackRate":
        return {...state, playbackRate: action.payload}
      default:
        return state
    }
  }

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

  const onTogglePlay = () => {
    if (videoRef.current) {
      if (videoRef.current.paused) {
        videoRef.current.play()
        dispatch({type: "play"})
      } else {
        videoRef.current.pause()
        dispatch({type: "pause"})
      }
    }
  }

  const onToggleMuted = () => {
    if (videoRef.current) {
      if (videoRef.current.muted) {
        videoRef.current.muted = false
        dispatch({type: "unmute"})
      } else {
        videoRef.current.muted = true
        dispatch({type: "mute"})
      }
    }
  }

  const onJump = (seconds: number) => {
    if (videoRef.current) {
      const newTime = videoRef.current.currentTime + seconds
      videoRef.current.currentTime = newTime
      dispatch({type: "setCurrentTime", payload: newTime})
    }
  }

  useHotkeys("space", onTogglePlay)
  useHotkeys("v m", onToggleMuted)
  useHotkeys("k", () => onJump(-0.5))
  useHotkeys("l", () => onJump(0.5))
  useHotkeys("right", () => onJump(5))
  useHotkeys("left", () => onJump(-5))

  return (
    <video {...rest} className={className} src={src} ref={videoRef} muted />
  )
}

export function PlayerControls() {
  const {state, dispatch} = usePlayer()
  const {isPlaying, isMuted} = state
  return (
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
      <button onClick={onToggleMuted} className="btn btn-square" type="button">
        {isMuted ? (
          <HiSpeakerWave className="w-5 h-5" />
        ) : (
          <HiSpeakerXMark className="w-5 h-5" />
        )}
      </button>
    </div>
  )
}
