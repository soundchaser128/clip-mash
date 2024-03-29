import {useStateMachine} from "little-state-machine"
import React, {useEffect, useMemo, useState} from "react"
import {useLoaderData, useNavigate, useRevalidator} from "react-router-dom"
import {updateForm} from "../actions"
import {
  HiBackward,
  HiChevronLeft,
  HiChevronRight,
  HiForward,
  HiPause,
  HiPlay,
} from "react-icons/hi2"
import clsx from "clsx"
import {useRef} from "react"
import {clamp, formatSeconds, getSegmentStyle} from "@/helpers"
import {ClipsLoaderData} from "../loaders"
import {Clip} from "@/api"
import {FormStage} from "@/types/form-state"
import useUndo from "use-undo"
import {produce} from "immer"
import ClipSettingsForm from "./settings/ClipSettingsForm"

const HelpText: React.FC<{onBack: () => void}> = ({onBack}) => {
  return (
    <div className="text-sm">
      <p className="text-sm link mb-2" onClick={onBack}>
        <HiChevronLeft className="mr-1 inline" />
        Back
      </p>
      <h2 className="text-xl font-bold mb-2">Information</h2>
      <p>
        <strong>Clip order</strong> - The order in which the clips will be
        played.
      </p>

      <p>
        <strong>Use music for clip generation?</strong> - If checked, the clips
        will be generated based on the music&apos;s beats and measures. If
        unchecked, the clips will be generated based on the video&apos;s
        duration.
      </p>

      <p>
        <strong>Clip generation method</strong> - TODO write help text
      </p>
    </div>
  )
}

interface ClipState {
  included: boolean
  clip: Clip
}

interface ClipsTimelineProps {
  clips: ClipState[]
  currentClipIndex: number
  setCurrentClipIndex: (n: number) => void
}

const ClipsTimeline: React.FC<ClipsTimelineProps> = ({
  clips,
  currentClipIndex,
  setCurrentClipIndex,
}) => {
  const [segments, sceneColors] = useMemo(() => {
    const clipLengths = clips.map(({clip}) => clip.range[1] - clip.range[0])
    const total = clipLengths.reduce((total, len) => total + len, 0)
    const segments = clipLengths.map((len) => {
      const percent = (len / total) * 100
      return `${percent}%`
    })

    const sceneIds = Array.from(new Set(clips.map((c) => c.clip.videoId)))
    sceneIds.sort()
    const sceneColors = new Map()
    sceneIds.forEach((id, index) => {
      const styles = getSegmentStyle(index, sceneIds.length)
      sceneColors.set(id, [styles, index])
    })

    return [segments, sceneColors]
  }, [clips])

  return (
    <div className="flex h-10 mt-2 gap-0.5">
      {segments.map((width, index) => {
        const clip = clips[index].clip
        const [style, sceneId] = sceneColors.get(clip.videoId)
        return (
          <div
            key={index}
            className={clsx(
              "flex justify-center items-center text-sm cursor-pointer text-white tooltip tooltip-bottom",
              index !== currentClipIndex && "opacity-30 hover:opacity-60",
              index === currentClipIndex && "opacity-100 ",
            )}
            style={{width, ...style}}
            onClick={() => setCurrentClipIndex(index)}
            data-tip={clip.markerTitle}
          >
            {sceneId + 1}
          </div>
        )
      })}
    </div>
  )
}

function PreviewClips() {
  const revalidator = useRevalidator()
  const [wasRevalidated, setWasRevalidated] = useState(false)
  const [manualChangesMade, setManualChangesMade] = useState(false)
  const [helpOpen, setHelpOpen] = useState(false)

  const {actions, state} = useStateMachine({updateForm})
  const loaderData = useLoaderData() as ClipsLoaderData
  const initialClips = wasRevalidated
    ? loaderData.clips
    : state.data.clips || loaderData.clips
  const streams = loaderData.streams
  const songs = state.data.songs ?? []
  const [clipsState, {set: setClips, undo, redo, canUndo, canRedo}] = useUndo(
    initialClips.map((clip) => ({clip, included: true})),
  )
  const clips = clipsState.present.filter((c) => c.included)

  const [currentClipIndex, setCurrentClipIndex] = useState(0)
  const [autoPlay, setAutoPlay] = useState(false)
  const currentClip = clips[currentClipIndex].clip
  const streamUrl = streams[currentClip.videoId]
  const clipUrl = `${streamUrl}#t=${currentClip.range[0]},${currentClip.range[1]}`
  const navigate = useNavigate()
  const videoRef = useRef<HTMLVideoElement>(null)
  const audioRef = useRef<HTMLAudioElement>(null)
  const currentMarker = state.data.selectedMarkers?.find(
    (m) => currentClip.markerId === m.id,
  )
  const totalLength = clips.reduce(
    (len, {clip}) => len + (clip.range[1] - clip.range[0]),
    0,
  )
  const [withMusic, setWithMusic] = useState(false)
  const [videoMuted, setVideoMuted] = useState(true)
  const hasAudio = state.data.songs && state.data.songs.length >= 1
  const [songIndex, setSongIndex] = useState(0)

  useEffect(() => {
    if (revalidator.state === "loading") {
      setManualChangesMade(false)
      setWasRevalidated(true)
    }
  }, [revalidator.state])

  useEffect(() => {
    if (wasRevalidated && revalidator.state === "idle") {
      setClips(initialClips.map((clip) => ({clip, included: true})))
    }
  }, [initialClips, revalidator.state, setClips, wasRevalidated])

  const onNextStage = () => {
    actions.updateForm({
      stage: FormStage.CreateVideo,
      clips: clips.map((c) => c.clip),
    })
    navigate("/generate")
  }

  const onVideoTimeUpdate: React.ReactEventHandler<HTMLVideoElement> = (
    event,
  ) => {
    const endTimestamp = currentClip.range[1]
    const currentTime = event.currentTarget.currentTime
    if (Math.abs(endTimestamp - currentTime) <= 0.5 && autoPlay) {
      setCurrentClipIndex((c) => (c + 1) % clips.length)
    }
  }

  const onAudioTimeUpdate: React.ReactEventHandler<HTMLAudioElement> = (
    event,
  ) => {
    const duration = event.currentTarget.duration
    const position = event.currentTarget.currentTime
    if (Math.abs(duration - position) <= 0.1 && autoPlay) {
      setSongIndex((idx) => Math.min(songs.length - 1, idx + 1))
    }
  }

  const toggleAutoPlay = () => {
    if (autoPlay) {
      videoRef.current?.pause()
      audioRef.current?.pause()
    } else {
      videoRef.current?.play()
      audioRef.current?.play()
    }

    setAutoPlay(!autoPlay)
  }

  const onRemoveClip = () => {
    setClips(
      clips.map(({clip}, i) => ({
        clip: clip,
        included: i !== currentClipIndex,
      })),
    )
    setCurrentClipIndex(clamp(currentClipIndex, 0, clips.length - 2))
    setManualChangesMade(true)
  }

  const onShiftClips = (direction: "left" | "right") => {
    const indexToSwap =
      direction === "left" ? currentClipIndex - 1 : currentClipIndex + 1
    const newClips = produce(clips, (draft) => {
      const temp = draft[currentClipIndex]
      draft[currentClipIndex] = draft[indexToSwap]
      draft[indexToSwap] = temp
    })
    setClips(newClips)
    setCurrentClipIndex(indexToSwap)
    setManualChangesMade(true)
  }

  return (
    <>
      <div className="mb-4 grid grid-cols-3">
        <div className="text-sm text-opacity-80">
          Preview the clips included in the final compilation. You can change
          the settings for clip generation and apply to see changes instantly.
          The number and color of the timeline segment identify the video it
          comes from.
        </div>
        <div className="text-center text-sm">
          <p>
            Showing clip{" "}
            <strong>
              {currentClipIndex + 1} / {clips.length}
            </strong>
          </p>
          <p>
            Current clip duration:{" "}
            <span className="font-semibold">
              {formatSeconds(currentClip.range, "short")}
            </span>
          </p>
          <p>
            Total video duration:{" "}
            <span className="font-semibold">
              {formatSeconds(totalLength, "short")}
            </span>
          </p>
          <p>
            Marker title:{" "}
            <span className="font-semibold">{currentMarker?.title}</span>
          </p>
        </div>
        <button
          type="button"
          onClick={onNextStage}
          className="btn btn-success place-self-end"
        >
          Next
          <HiChevronRight className="ml-1" />
        </button>
      </div>

      {hasAudio && (
        <audio
          ref={audioRef}
          src={`/api/song/${songs[songIndex]?.songId}/stream`}
          autoPlay={autoPlay}
          muted={!withMusic}
          onTimeUpdate={onAudioTimeUpdate}
        />
      )}

      <section className="flex">
        <video
          className="w-3/4 h-[650px]"
          src={clipUrl}
          muted={videoMuted}
          autoPlay={autoPlay}
          onTimeUpdate={onVideoTimeUpdate}
          ref={videoRef}
        />
        <section className="flex flex-col px-4 py-2 w-2/5 bg-base-200 justify-between">
          {helpOpen && <HelpText onBack={() => setHelpOpen(false)} />}
          {!helpOpen && (
            <>
              <ClipSettingsForm
                onRemoveClip={onRemoveClip}
                onUndo={undo}
                onRedo={redo}
                canUndo={canUndo}
                canRedo={canRedo}
                onShiftClips={onShiftClips}
                canShiftLeft={currentClipIndex > 0}
                canShiftRight={currentClipIndex < clips.length - 1}
                confirmBeforeSubmit={manualChangesMade}
                setHelpOpen={setHelpOpen}
              />
              <div className="flex flex-col">
                <div className="btn-group justify-center">
                  <button
                    type="button"
                    className="btn btn-square btn-lg"
                    onClick={() => setCurrentClipIndex((i) => i - 1)}
                    disabled={currentClipIndex === 0}
                  >
                    <HiBackward />
                  </button>
                  <button
                    type="button"
                    className={clsx(
                      "btn btn-square btn-lg",
                      autoPlay ? "btn-neutral" : "btn-success",
                    )}
                    onClick={toggleAutoPlay}
                  >
                    {autoPlay ? <HiPause /> : <HiPlay />}
                  </button>
                  <button
                    type="button"
                    className="btn btn-square btn-lg"
                    onClick={() => setCurrentClipIndex((i) => i + 1)}
                    disabled={currentClipIndex >= clips.length - 1}
                  >
                    <HiForward />
                  </button>
                </div>
                <div className="form-control">
                  <label className="label cursor-pointer">
                    <span className="label-text mr-2">Mute video</span>
                    <input
                      type="checkbox"
                      className="toggle"
                      checked={videoMuted}
                      onChange={(e) => setVideoMuted(e.target.checked)}
                    />
                  </label>
                </div>

                {hasAudio && (
                  <div className="form-control">
                    <label className="label cursor-pointer">
                      <span className="label-text mr-2">Enable music</span>
                      <input
                        type="checkbox"
                        className="toggle"
                        checked={withMusic}
                        onChange={(e) => setWithMusic(e.target.checked)}
                      />
                    </label>
                  </div>
                )}
              </div>
            </>
          )}
        </section>
      </section>

      <ClipsTimeline
        clips={clips}
        currentClipIndex={currentClipIndex}
        setCurrentClipIndex={setCurrentClipIndex}
      />
    </>
  )
}

export default PreviewClips
