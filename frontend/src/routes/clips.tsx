import {useStateMachine} from "little-state-machine"
import React, {useEffect, useMemo, useState} from "react"
import {
  LoaderFunction,
  json,
  useLoaderData,
  useNavigate,
  useRevalidator,
} from "react-router-dom"
import {
  FormStage,
  LocalVideosFormState,
  StashFormState,
  StateHelpers,
} from "../types/types"
import {updateForm} from "./actions"
import {HiChevronRight, HiPause, HiPlay} from "react-icons/hi2"
import clsx from "clsx"
import {useRef} from "react"
import {useImmer} from "use-immer"
import invariant from "tiny-invariant"
import {formatSeconds, getFormState, getSegmentColor} from "../helpers"
import {
  Clip,
  ClipOptions,
  ClipOrder,
  CreateClipsBody,
  VideoDto,
} from "../types.generated"
import {useForm} from "react-hook-form"

const DEBUG = false

interface ClipsResponse {
  clips: Clip[]
  streams: Record<string, string>
  videos: VideoDto[]
  beatOffsets?: number[]
}

interface Data {
  clips: Clip[]
  streams: Record<string, string>
  videos: Record<string, VideoDto>
  beatOffsets?: number[]
}

const getClipSettings = (
  state: LocalVideosFormState | StashFormState
): ClipOptions => {
  if (!state.splitClips) {
    return {
      type: "noSplit",
    }
  } else if (state.songs && state.songs.length > 0) {
    return {
      type: "pmv",
      song_ids: state.songs.map(({songId}) => songId),
      clips: {
        type: "songs",
        beatsPerMeasure: 4,
        cutAfterMeasures: {
          random: [2, 4],
        },
      },
    }
  } else {
    return {
      type: "default",
      baseDuration: state.clipDuration || 30,
      divisors: [2.0, 3.0, 4.0],
    }
  }
}

export const loader: LoaderFunction = async () => {
  const state = getFormState()!
  invariant(StateHelpers.isNotInitial(state))

  const body = {
    clipOrder: state.clipOrder || "scene-order",
    markers: state.selectedMarkers!.filter((m) => m.selected),
    seed: state.seed || null,
    clips: getClipSettings(state),
  } satisfies CreateClipsBody

  const response = await fetch("/api/clips", {
    method: "POST",
    body: JSON.stringify(body),
    headers: {"content-type": "application/json"},
  })
  if (response.ok) {
    const data: ClipsResponse = await response.json()

    const videos: Record<string, VideoDto> = {}
    data.videos.forEach((s) => {
      videos[s.id.id] = s
    })

    return {
      ...data,
      videos,
    } satisfies Data
  } else {
    const text = await response.text()
    throw json({error: text, request: "/api/clips"}, {status: 500})
  }
}

interface ClipState {
  clip: Clip
  included: boolean
}

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
    <form onSubmit={handleSubmit(onSubmit)} className="flex flex-col">
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
      <button className="btn btn-success self-end mt-4">Apply</button>
    </form>
  )
}

function PreviewClips() {
  const loaderData = useLoaderData() as Data
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
    } else {
      videoRef.current?.play()
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

      <div className="flex">
        <video
          className="w-3/4"
          src={clipUrl}
          muted
          autoPlay={autoPlay}
          onTimeUpdate={onTimeUpdate}
          ref={videoRef}
        />
        <div className="flex flex-col pl-4 w-1/4">
          <h2 className="text-xl font-bold">Settings</h2>
          <ClipSettingsForm
            initialValues={{
              clipDuration: state.data.clipDuration || 30,
              clipOrder: state.data.clipOrder || "scene-order",
              splitClips: state.data.splitClips || true,
              seed: state.data.seed,
            }}
          />

          <div className="flex gap-4 items-center">
            <button
              className={clsx("btn", autoPlay ? "btn-warning" : "btn-success")}
              onClick={toggleAutoPlay}
            >
              {autoPlay ? (
                <HiPause className="mr-2" />
              ) : (
                <HiPlay className="mr-2" />
              )}
              {autoPlay ? "Pause" : "Play"}
            </button>
          </div>

          <div className="flex gap-2">
            <button
              className="btn"
              onClick={() => setCurrentClipIndex((i) => i - 1)}
              disabled={currentClipIndex === 0}
            >
              Previous
            </button>
            <button
              className="btn"
              onClick={() => setCurrentClipIndex((i) => i + 1)}
              disabled={currentClipIndex >= clips.length - 1}
            >
              Next
            </button>
          </div>

          {loaderData.beatOffsets && (
            <BeatIndicator
              autoPlay={autoPlay}
              offsets={loaderData.beatOffsets}
            />
          )}
        </div>
      </div>

      <div className="w-full h-8 flex mt-2 gap-0.5">
        {segments.map((width, index) => {
          const clip = clips[index].clip
          const video = loaderData.videos[clip.videoId.id]
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
                index !== currentClipIndex &&
                  "bg-opacity-25 hover:bg-opacity-50"
              )}
              style={{width}}
              onClick={() => setCurrentClipIndex(index)}
            >
              {sceneId + 1}
            </div>
          )
        })}
      </div>
    </>
  )
}

export default PreviewClips
