import {useStateMachine} from "little-state-machine"
import React, {useEffect, useMemo, useState} from "react"
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
import {Clip, ClipOrder, VideoDto} from "../types.generated"
import {useForm} from "react-hook-form"
import {ClipsLoaderData} from "./loaders"

const DEBUG = false

const BeatIndicator: React.FC<{offsets: number[]; autoPlay: boolean}> = ({
  offsets,
  autoPlay,
}) => {
  const lastTime = useRef(0)
  const totalTime = useRef(0)
  const requestRef = useRef<number>()
  const offsetIndex = useRef(0)
  const [showBeat, setShowBeat] = useState(false)

  const onAnimationFrame = (time: number) => {
    time -= performance.timeOrigin / 1e9
    time /= 1000.0

    if (offsets && autoPlay && !showBeat) {
      const nextBeat = offsets[offsetIndex.current]
      const diff = Math.abs(nextBeat - totalTime.current)
      if (diff <= 0.05) {
        setShowBeat(true)
        window.setTimeout(() => setShowBeat(false), 250)
        offsetIndex.current += 1
      }
      const delta = time - lastTime.current
      totalTime.current += delta
      lastTime.current = time
      requestRef.current = requestAnimationFrame(onAnimationFrame)
    }
  }

  useEffect(() => {
    if (autoPlay) {
      requestRef.current = requestAnimationFrame(onAnimationFrame)
      return () => cancelAnimationFrame(requestRef.current!)
    }
  }, [autoPlay])

  return (
    <div
      className={clsx(
        "w-12 h-12 self-center rounded-full my-2",
        showBeat ? "bg-red-500" : "bg-white"
      )}
    />
  )
}

interface ClipState {
  included: boolean
  clip: Clip
}

interface TimelineProps {
  clips: ClipState[]
  videos: Record<string, VideoDto>
  currentClipIndex: number
  setCurrentClipIndex: (n: number) => void
}

const Timeline: React.FC<TimelineProps> = ({
  clips,
  videos,
  currentClipIndex,
  setCurrentClipIndex,
}) => {
  const [segments, sceneColors] = useMemo(() => {
    const clipLengths = clips.map(({clip}) => clip.range[1] - clip.range[0])
    const total = clipLengths.reduce((total, len) => total + len, 0)
    const segments = clipLengths.map((len) => `${(len / total) * 100}%`)

    const sceneIds = Array.from(new Set(clips.map((c) => c.clip.videoId.id)))
    sceneIds.sort()
    const sceneColors = new Map()
    sceneIds.forEach((id, index) => {
      sceneColors.set(id, [getSegmentColor(index), index])
    })

    return [segments, sceneColors]
  }, [clips])

  return (
    <div className="w-full h-8 flex mt-2 gap-0.5">
      {segments.map((width, index) => {
        const clip = clips[index].clip
        const video = videos[clip.videoId.id]
        const [color, sceneId] = sceneColors.get(clip.videoId.id)
        let tooltip = video.title
        if (video.performers.length > 0) {
          tooltip = `${video.performers.join(", ")} - ${video.title}`
        }
        return (
          <div
            key={index}
            data-tip={tooltip}
            className={clsx(
              "h-full tooltip transition-opacity flex items-center justify-center",
              color,
              index !== currentClipIndex && "bg-opacity-25 hover:bg-opacity-50"
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
}

const ClipSettingsForm: React.FC<{initialValues: Inputs}> = ({
  initialValues,
}) => {
  const {register, handleSubmit, watch} = useForm<Inputs>({
    defaultValues: initialValues,
  })
  const doSplitClips = watch("splitClips")
  const revalidator = useRevalidator()
  const {actions} = useStateMachine({updateForm})

  const onSubmit = (values: Inputs) => {
    actions.updateForm({
      clipDuration: values.clipDuration,
      clipOrder: values.clipOrder,
      splitClips: values.splitClips,
      seed: values.seed,
    })

    revalidator.revalidate()
  }

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="flex flex-col mb-4">
      <h2 className="text-xl font-bold">Settings</h2>
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
      <button className="btn btn-success self-end mt-4">
        <HiCheck className="mr-2" />
        Apply
      </button>
    </form>
  )
}

function PreviewClips() {
  const loaderData = useLoaderData() as ClipsLoaderData
  const streams = loaderData.streams
  // const [clips, setClips] = useImmer<ClipState[]>(
  //   loaderData.clips.map((clip) => ({clip, included: true}))
  // )
  const clips = loaderData.clips.map((clip) => ({clip, included: true}))

  const [currentClipIndex, setCurrentClipIndex] = useState(0)
  const [autoPlay, setAutoPlay] = useState(false)
  const currentClip = clips[currentClipIndex].clip
  const streamUrl = streams[currentClip.videoId.id]
  const clipUrl = `${streamUrl}#t=${currentClip.range[0]},${currentClip.range[1]}`
  const {actions, state} = useStateMachine({updateForm})
  invariant(StateHelpers.isNotInitial(state.data))
  const navigate = useNavigate()
  const videoRef = useRef<HTMLVideoElement>(null)
  const audioRef = useRef<HTMLAudioElement>(null)
  const totalLength = clips.reduce(
    (len, {clip}) => len + (clip.range[1] - clip.range[0]),
    0
  )

  const onNextStage = () => {
    actions.updateForm({
      stage: FormStage.Wait,
      clips: clips.filter((c) => c.included).map((c) => c.clip),
    })
    navigate("/stash/progress")
  }

  const onTimeUpdate: React.ReactEventHandler<HTMLVideoElement> = (event) => {
    const endTimestamp = currentClip.range[1]
    const currentTime = event.currentTarget.currentTime
    if (Math.abs(endTimestamp - currentTime) <= 0.5 && autoPlay) {
      setCurrentClipIndex((c) => (c + 1) % clips.length)
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
      <div className="mb-4 grid grid-cols-3 items-center">
        <div></div>
        <div className="text-center">
          <p className="">
            Showing clip{" "}
            <strong>
              {currentClipIndex + 1} / {clips.length}
            </strong>
          </p>
          <p>
            Current clip duration:{" "}
            <strong>{formatSeconds(currentClip.range, "short")}</strong>
          </p>
          <p>
            Total video duration:{" "}
            <strong>{formatSeconds(totalLength, "short")}</strong>
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

      <audio
        ref={audioRef}
        src={`/api/song/${state.data.songs![0].songId}/stream`}
        autoPlay={autoPlay}
      />

      <div className="flex">
        <video
          className="w-3/4"
          src={clipUrl}
          muted
          autoPlay={autoPlay}
          onTimeUpdate={onTimeUpdate}
          ref={videoRef}
        />
        <div className="flex flex-col px-4 py-2 w-1/4 bg-slate-100 justify-between">
          <ClipSettingsForm
            initialValues={{
              clipDuration: state.data.clipDuration || 30,
              clipOrder: state.data.clipOrder || "scene-order",
              splitClips: state.data.splitClips || true,
              seed: state.data.seed,
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

          {/* {loaderData.beatOffsets && (
            <BeatIndicator
              autoPlay={autoPlay}
              offsets={loaderData.beatOffsets}
            />
          )} */}
        </div>
      </div>

      <Timeline
        clips={clips}
        videos={loaderData.videos}
        currentClipIndex={currentClipIndex}
        setCurrentClipIndex={setCurrentClipIndex}
      />
    </>
  )
}

export default PreviewClips
