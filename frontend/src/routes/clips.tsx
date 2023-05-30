import {useStateMachine} from "little-state-machine"
import React, {useMemo, useState} from "react"
import {useLoaderData, useNavigate, useRevalidator} from "react-router-dom"
import {FormStage, StateHelpers} from "../types/types"
import {updateForm} from "./actions"
import {
  HiBackward,
  HiCheck,
  HiChevronRight,
  HiForward,
  HiPause,
  HiPlay,
} from "react-icons/hi2"
import clsx from "clsx"
import {useRef} from "react"
import invariant from "tiny-invariant"
import {formatSeconds, getSegmentColor} from "../helpers"
import {Clip, ClipOrder} from "../types.generated"
import {useForm} from "react-hook-form"
import {ClipsLoaderData} from "./loaders"
import styles from "./clips.module.css"

const DEBUG = false

interface ClipState {
  included: boolean
  clip: Clip
}

interface TimelineProps {
  clips: ClipState[]
  currentClipIndex: number
  setCurrentClipIndex: (n: number) => void
}

const Timeline: React.FC<TimelineProps> = ({
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

    const sceneIds = Array.from(new Set(clips.map((c) => c.clip.videoId.id)))
    sceneIds.sort()
    const sceneColors = new Map()
    sceneIds.forEach((id, index) => {
      sceneColors.set(id, [getSegmentColor(index), index])
    })

    return [segments, sceneColors]
  }, [clips])

  return (
    <div className="flex h-10 mt-2 gap-0.5">
      {segments.map((width, index) => {
        const clip = clips[index].clip
        const [color, sceneId] = sceneColors.get(clip.videoId.id)
        return (
          <div
            key={index}
            className={clsx(
              styles.timelineSegment,
              color,
              index !== currentClipIndex && styles.inactive
            )}
            style={{width}}
            onClick={() => setCurrentClipIndex(index)}
          >
            {sceneId + 1}
          </div>
        )
      })}
    </div>
  )
}

interface Inputs {
  clipOrder: ClipOrder
  seed?: string
  splitClips: boolean
  clipDuration: number
  beatsPerMeasure: number
  measureCountType: "fixed" | "random"
  measureCountFixed: number
  measureCountRandomStart: number
  measureCountRandomEnd: number
}

const ClipSettingsForm: React.FC<{initialValues: Inputs}> = ({
  initialValues,
}) => {
  const {register, handleSubmit, watch} = useForm<Inputs>({
    defaultValues: initialValues,
  })
  const doSplitClips = watch("splitClips")
  const measureCountType = watch("measureCountType")

  const revalidator = useRevalidator()
  const {actions, state} = useStateMachine({updateForm})
  invariant(StateHelpers.isNotInitial(state.data))
  const isPmv =
    state.data.songs?.length !== 0 && state.data.clipStrategy === "pmv"

  const onSubmit = (values: Inputs) => {
    actions.updateForm({
      clipDuration: values.clipDuration,
      clipOrder: values.clipOrder,
      splitClips: values.splitClips,
      seed: values.seed,
      beatsPerMeasure: values.beatsPerMeasure,
      cutAfterMeasures:
        values.measureCountType === "fixed"
          ? {type: "fixed", count: values.measureCountFixed}
          : {
              type: "random",
              min: values.measureCountRandomStart,
              max: values.measureCountRandomEnd,
            },
    })

    revalidator.revalidate()
  }

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="flex flex-col mb-4">
      <h2 className="text-xl font-bold">Settings</h2>
      {!isPmv && (
        <>
          <div className="form-control">
            <label className="label cursor-pointer">
              <span className="label-text mr-2">
                Split up marker videos into clips
              </span>
              <input
                type="checkbox"
                className="toggle"
                {...register("splitClips")}
              />
            </label>
          </div>
          <div className="form-control">
            <label className="label">
              <span className="label-text">
                Maximum duration per clip (in seconds):
              </span>
            </label>
            <input
              type="number"
              className="input input-bordered"
              disabled={!doSplitClips}
              {...register("clipDuration", {valueAsNumber: true})}
            />
          </div>
        </>
      )}

      {isPmv && (
        <>
          <div className="form-control">
            <label className="label">
              <span className="label-text">Beats per measure</span>
            </label>
            <input
              type="number"
              className="input input-bordered"
              disabled={!doSplitClips}
              {...register("beatsPerMeasure", {valueAsNumber: true})}
            />
          </div>
          <div className="form-control">
            <label className="label">
              <span className="label-text">Cut after ... measures</span>
            </label>
            <select
              className="select select-bordered"
              {...register("measureCountType")}
            >
              <option disabled value="none">
                Select how to cut...
              </option>
              <option value="random">Randomized</option>
              <option value="fixed">Fixed</option>
            </select>
          </div>

          {measureCountType === "fixed" && (
            <div className="form-control">
              <label className="label cursor-pointer">
                <span className="label-text">Cut after how many measures?</span>
              </label>
              <input
                type="number"
                className="input input-bordered"
                {...register("measureCountFixed", {valueAsNumber: true})}
              />
            </div>
          )}

          {measureCountType === "random" && (
            <>
              <div className="form-control">
                <label className="label cursor-pointer">
                  <span className="label-text">Minimum</span>
                </label>
                <input
                  type="number"
                  className="input input-bordered"
                  {...register("measureCountRandomStart", {
                    valueAsNumber: true,
                  })}
                />
              </div>
              <div className="form-control">
                <label className="label cursor-pointer">
                  <span className="label-text">Maximum</span>
                </label>
                <input
                  type="number"
                  className="input input-bordered"
                  {...register("measureCountRandomEnd", {valueAsNumber: true})}
                />
              </div>
            </>
          )}
        </>
      )}

      <div className="form-control">
        <label className="label">
          <span className="label-text">Clip order:</span>
        </label>
        <select className="select select-bordered" {...register("clipOrder")}>
          <option disabled value="none">
            Select clip ordering
          </option>
          <option value="scene-order">Scene order</option>
          <option value="random">Random</option>
        </select>
      </div>

      <div className="form-control">
        <label className="label">
          <span className="label-text">Random seed:</span>
        </label>
        <input
          type="text"
          className="input input-bordered"
          placeholder="Enter a value to control random number generation (optional)"
          {...register("seed")}
        />
      </div>
      <button className="btn btn-primary self-end mt-4">
        <HiCheck className="mr-2" />
        Apply
      </button>
    </form>
  )
}

function PreviewClips() {
  const loaderData = useLoaderData() as ClipsLoaderData
  const streams = loaderData.streams
  const {actions, state} = useStateMachine({updateForm})
  invariant(StateHelpers.isNotInitial(state.data))
  const songs = state.data.songs ?? []
  const clips = loaderData.clips.map((clip) => ({clip, included: true}))

  const [currentClipIndex, setCurrentClipIndex] = useState(0)
  const [autoPlay, setAutoPlay] = useState(false)
  const currentClip = clips[currentClipIndex].clip
  const streamUrl = streams[currentClip.videoId.id]
  const clipUrl = `${streamUrl}#t=${currentClip.range[0]},${currentClip.range[1]}`
  const navigate = useNavigate()
  const videoRef = useRef<HTMLVideoElement>(null)
  const audioRef = useRef<HTMLAudioElement>(null)
  const totalLength = clips.reduce(
    (len, {clip}) => len + (clip.range[1] - clip.range[0]),
    0
  )
  const [withMusic, setWithMusic] = useState(false)
  const isPmv = state.data.songs && state.data.songs.length >= 1
  const [songIndex, setSongIndex] = useState(0)

  const onNextStage = () => {
    actions.updateForm({
      stage: FormStage.Wait,
      clips: clips.filter((c) => c.included).map((c) => c.clip),
    })
    navigate("/stash/progress")
  }

  const onVideoTimeUpdate: React.ReactEventHandler<HTMLVideoElement> = (
    event
  ) => {
    const endTimestamp = currentClip.range[1]
    const currentTime = event.currentTarget.currentTime
    if (Math.abs(endTimestamp - currentTime) <= 0.5 && autoPlay) {
      setCurrentClipIndex((c) => (c + 1) % clips.length)
    }
  }

  const onAudioTimeUpdate: React.ReactEventHandler<HTMLAudioElement> = (
    event
  ) => {
    const duration = event.currentTarget.duration
    const position = event.currentTarget.currentTime
    if (Math.abs(duration - position) <= 0.1 && autoPlay) {
      console.log("switching song")
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

  return (
    <>
      <div className="mb-4 grid grid-cols-3">
        <div className="text-sm text-gray-600">
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
          {DEBUG && (
            <>
              <p>Index within the marker: {currentClip.indexWithinMarker}</p>
              <p>Index within its video: {currentClip.indexWithinVideo}</p>
            </>
          )}
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

      {isPmv && (
        <audio
          ref={audioRef}
          src={`/api/song/${songs[songIndex]?.songId}/stream`}
          autoPlay={autoPlay}
          muted={!withMusic}
          onTimeUpdate={onAudioTimeUpdate}
        />
      )}

      <div className="flex">
        <video
          className="w-3/4 h-[650px]"
          src={clipUrl}
          muted
          autoPlay={autoPlay}
          onTimeUpdate={onVideoTimeUpdate}
          ref={videoRef}
        />
        <div className="flex flex-col px-4 py-2 w-1/4 bg-slate-100 justify-between">
          <ClipSettingsForm
            initialValues={{
              clipDuration: state.data.clipDuration || 30,
              clipOrder: state.data.clipOrder || "scene-order",
              splitClips: state.data.splitClips || true,
              seed: state.data.seed,
              beatsPerMeasure: state.data.beatsPerMeasure || 4,
              measureCountFixed:
                state.data.cutAfterMeasures?.type === "fixed"
                  ? state.data.cutAfterMeasures.count
                  : 4,
              measureCountRandomStart:
                state.data.cutAfterMeasures?.type === "random"
                  ? state.data.cutAfterMeasures.min
                  : 2,
              measureCountRandomEnd:
                state.data.cutAfterMeasures?.type === "random"
                  ? state.data.cutAfterMeasures.max
                  : 4,
              measureCountType: state.data.cutAfterMeasures?.type || "fixed",
            }}
          />

          <div className="btn-group justify-center">
            <button
              className="btn btn-square btn-lg"
              onClick={() => setCurrentClipIndex((i) => i - 1)}
              disabled={currentClipIndex === 0}
            >
              <HiBackward />
            </button>
            <button
              className={clsx(
                "btn btn-square btn-lg",
                autoPlay ? "btn-warning" : "btn-success"
              )}
              onClick={toggleAutoPlay}
            >
              {autoPlay ? <HiPause /> : <HiPlay />}
            </button>
            <button
              className="btn btn-square btn-lg"
              onClick={() => setCurrentClipIndex((i) => i + 1)}
              disabled={currentClipIndex >= clips.length - 1}
            >
              <HiForward />
            </button>
          </div>

          {isPmv && (
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
      </div>

      <Timeline
        clips={clips}
        currentClipIndex={currentClipIndex}
        setCurrentClipIndex={setCurrentClipIndex}
      />
    </>
  )
}

export default PreviewClips
