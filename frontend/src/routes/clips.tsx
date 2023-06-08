import {useStateMachine} from "little-state-machine"
import React, {useMemo, useState} from "react"
import {useLoaderData, useNavigate, useRevalidator} from "react-router-dom"
import {FormStage, StateHelpers} from "../types/types"
import {updateForm} from "./actions"
import {
  HiBackward,
  HiCheck,
  HiChevronRight,
  HiCog8Tooth,
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
import Modal from "../components/Modal"
import {useImmer} from "use-immer"

function pluralize(word: string, count: number | undefined | null): string {
  return count === 1 ? word : `${word}s`
}

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

interface MarkerCount {
  total: number
  current: number
}

interface WeightsModalProps {
  className?: string
  clips: Clip[]
}

const WeightsModal: React.FC<WeightsModalProps> = ({className, clips}) => {
  const revalidator = useRevalidator()
  const {state, actions} = useStateMachine({updateForm})
  invariant(StateHelpers.isNotInitial(state.data))
  const markerCounts = useMemo(() => {
    invariant(StateHelpers.isNotInitial(state.data))

    const counts = new Map<string, MarkerCount>()
    for (const marker of state.data.selectedMarkers ?? []) {
      const count = counts.get(marker.title) ?? {total: 0, current: 0}
      counts.set(marker.title, {total: count.total + 1, current: count.current})
    }
    for (const clip of clips) {
      const marker = state.data.selectedMarkers?.find(
        (m) => m.id.id === clip.markerId.id
      )
      if (marker && marker.title) {
        const count = counts.get(marker.title) ?? {total: 0, current: 0}
        counts.set(marker.title, {
          total: count.total,
          current: count.current + 1,
        })
      }
    }

    return counts
  }, [state.data.selectedMarkers])

  const [weights, setWeights] = useImmer<Array<[string, number]>>(() => {
    invariant(StateHelpers.isNotInitial(state.data))
    if (state.data.clipWeights) {
      return state.data.clipWeights
    } else {
      const markerTitles = Array.from(
        new Set(state.data.selectedMarkers?.map((m) => m.title.trim()))
      ).sort()
      return Array.from(markerTitles).map((title) => [title, 1.0])
    }
  })

  const [enabled, setEnabled] = useState(
    state.data.clipStrategy === "weightedRandom"
  )
  const [open, setOpen] = useState(false)

  const onWeightChange = (title: string, weight: number) => {
    setWeights((draft) => {
      const index = draft.findIndex((e) => e[0] === title)
      if (index !== -1) {
        draft[index][1] = weight / 100
      }
    })
  }

  const onClose = () => {
    setOpen(false)
    if (enabled) {
      actions.updateForm({
        clipWeights: weights,
        clipStrategy: "weightedRandom",
      })

      revalidator.revalidate()
    } else {
      actions.updateForm({
        clipStrategy: "roundRobin",
      })
    }
  }

  return (
    <>
      <button
        onClick={() => setOpen(true)}
        className={clsx("btn btn-secondary", className)}
      >
        <HiCog8Tooth className="mr-2" />
        Adjust marker ratios
      </button>
      <Modal position="top" size="fluid" isOpen={open} onClose={onClose}>
        <h1 className="text-2xl font-bold mb-2">Marker ratios</h1>
        <p className="text-sm mb-4">
          Here, you can adjust the likelihood of each marker type to be included
          in the compilation.
        </p>

        <div className="flex flex-col gap-4 items-center">
          <div className="form-control w-72">
            <label className="label cursor-pointer">
              <span className="label-text">Enable marker ratios</span>
              <input
                type="checkbox"
                className="checkbox checkbox-primary"
                onChange={(e) => setEnabled(e.target.checked)}
                checked={enabled}
              />
            </label>
          </div>
          {weights.map(([title, weight]) => {
            const count = markerCounts.get(title)
            const markerLabel = pluralize("marker", count?.total ?? 0)
            const clipLabel = pluralize("clip", count?.current ?? 0)

            return (
              <div
                className={clsx(
                  "form-control",
                  !enabled && "opacity-50 cursor-not-allowed"
                )}
                key={title}
              >
                <label className="label">
                  <span className="label-text font-semibold">
                    {title} ({count?.total} {markerLabel}, {count?.current}{" "}
                    {clipLabel})
                  </span>
                </label>
                <input
                  disabled={!enabled}
                  type="range"
                  min="0"
                  max="100"
                  className="range range-sm w-72"
                  step="5"
                  value={weight * 100}
                  onChange={(e) =>
                    onWeightChange(title, e.target.valueAsNumber)
                  }
                />
                <div className="w-full flex justify-between text-xs px-2">
                  <span>0%</span>
                  <span className="font-bold">{Math.round(weight * 100)}%</span>
                  <span>100%</span>
                </div>
              </div>
            )
          })}

          <button onClick={onClose} className="btn btn-primary self-end">
            Done
          </button>
        </div>
      </Modal>
    </>
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

const ClipSettingsForm: React.FC<{initialValues: Inputs; clips: Clip[]}> = ({
  initialValues,
  clips,
}) => {
  const {register, handleSubmit, watch} = useForm<Inputs>({
    defaultValues: initialValues,
  })
  const doSplitClips = watch("splitClips")
  const measureCountType = watch("measureCountType")

  const revalidator = useRevalidator()
  const {actions, state} = useStateMachine({updateForm})
  invariant(StateHelpers.isNotInitial(state.data))
  const isPmv = state.data.songs?.length !== 0

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
      <WeightsModal className="my-4" clips={clips} />
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
  const currentMarker = state.data.selectedMarkers?.find(
    (m) => currentClip.markerId.id === m.id.id
  )
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
        <div className="flex flex-col px-4 py-2 w-1/4 bg-base-200 justify-between">
          <ClipSettingsForm
            clips={clips.map((c) => c.clip)}
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
