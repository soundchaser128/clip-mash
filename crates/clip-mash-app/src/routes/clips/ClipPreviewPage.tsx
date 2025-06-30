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
import {getSegmentStyle} from "@/helpers/style"
import {clamp} from "@/helpers/math"
import {ClipsLoaderData} from "../loaders"
import {Clip} from "@/api"
import {FormStage} from "@/types/form-state"
import useUndo from "use-undo"
import {produce} from "immer"
import ClipSettingsForm from "./settings/ClipSettingsForm"
import {formatSeconds} from "@/helpers/time"
import {getClipUrl} from "@/helpers/clips"

interface IncludedClip {
  clip: Clip
  included: boolean
}

interface ClipInfoProps {
  currentClipIndex: number
  clips: IncludedClip[]
  totalLength: number
  onNextStage: () => void
  currentMarker?: {title: string}
}

const ClipInfo: React.FC<ClipInfoProps> = ({
  currentClipIndex,
  clips,
  totalLength,
  onNextStage,
  currentMarker,
}) => {
  const currentClip = clips[currentClipIndex]?.clip

  return (
    <div className="mb-4 grid grid-cols-3">
      <div className="text-sm text-opacity-80">
        Preview the clips included in the final compilation. You can change the
        settings for clip generation and apply to see changes instantly. The
        number and color of the timeline segment identify the video it comes
        from.
      </div>
      <div className="text-center text-sm">
        <p>
          Showing clip{" "}
          <strong>
            {currentClipIndex + 1} / {clips.length}
          </strong>
        </p>
        {currentClip && (
          <p>
            Current clip duration:{" "}
            <span className="font-semibold">
              {formatSeconds(currentClip.range, "short-with-ms")}
            </span>
          </p>
        )}

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
  )
}

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
  // TODO don't crash when clips are empty
  const currentClip = clips[currentClipIndex].clip

  const clipUrl = getClipUrl(streams, currentClip)
  const navigate = useNavigate()
  const videoRef = useRef<HTMLVideoElement>(null)
  const audioRef = useRef<HTMLAudioElement>(null)
  const currentMarker = currentClip
    ? state.data.selectedMarkers?.find((m) => currentClip.markerId === m.id)
    : undefined
  const totalLength = clips.reduce(
    (len, {clip}) => len + (clip.range[1] - clip.range[0]),
    0,
  )
  const [withMusic, setWithMusic] = useState(false)
  const [videoMuted, setVideoMuted] = useState(true)
  const hasAudio = state.data.songs && state.data.songs.length > 0
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

  useEffect(() => {
    if (videoRef.current) {
      videoRef.current.load()
    }
  }, [currentClipIndex])

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
    if (currentClip) {
      const endTimestamp = currentClip.range[1]
      const currentTime = event.currentTarget.currentTime
      if (Math.abs(endTimestamp - currentTime) <= 0.5 && autoPlay) {
        setCurrentClipIndex((c) => (c + 1) % clips.length)
      }
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
      <ClipInfo
        currentClipIndex={currentClipIndex}
        clips={clips}
        totalLength={totalLength}
        onNextStage={onNextStage}
        currentMarker={currentMarker}
      />
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
        {clips.length === 0 && (
          <div className="w-3/4 h-[650px] flex justify-center items-center">
            No clips were generated. Check the clip duration and minimum clip
            length settings.
          </div>
        )}
        {clips.length > 0 && (
          <video
            className="w-3/4 h-[650px]"
            muted={videoMuted}
            autoPlay={autoPlay}
            onTimeUpdate={onVideoTimeUpdate}
            ref={videoRef}
          >
            {clipUrl?.map((url, index) => (
              <source key={index} src={url.src} type={url.type} />
            ))}
          </video>
        )}
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
                <div className="join justify-center">
                  <button
                    type="button"
                    className="btn btn-square btn-lg join-item"
                    onClick={() => setCurrentClipIndex((i) => i - 1)}
                    disabled={currentClipIndex === 0}
                  >
                    <HiBackward />
                  </button>
                  <button
                    type="button"
                    className={clsx(
                      "btn btn-square btn-lg join-item",
                      autoPlay ? "btn-neutral" : "btn-success",
                    )}
                    onClick={toggleAutoPlay}
                  >
                    {autoPlay ? <HiPause /> : <HiPlay />}
                  </button>
                  <button
                    type="button"
                    className="btn btn-square btn-lg join-item"
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
