import {useStateMachine} from "little-state-machine"
import React, {useEffect, useMemo, useState} from "react"
import {useLoaderData, useNavigate, useRevalidator} from "react-router-dom"
import {updateForm} from "./actions"
import {
  HiArrowUturnLeft,
  HiArrowUturnRight,
  HiBackward,
  HiCheck,
  HiChevronRight,
  HiCog8Tooth,
  HiForward,
  HiPause,
  HiPlay,
  HiTrash,
} from "react-icons/hi2"
import clsx from "clsx"
import {useRef} from "react"
import {
  clamp,
  formatSeconds,
  getSegmentColor,
  getSegmentTextColor,
  pluralize,
} from "../helpers"
import {useForm} from "react-hook-form"
import {ClipsLoaderData} from "./loaders"
import Modal from "../components/Modal"
import {useImmer} from "use-immer"
import {Clip, ClipOrder} from "../api"
import {FormStage} from "../types/form-state"
import useUndo from "use-undo"
import {produce} from "immer"
import {useDrag, useDrop} from "react-dnd"

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

    const sceneIds = Array.from(new Set(clips.map((c) => c.clip.videoId)))
    sceneIds.sort()
    const sceneColors = new Map()
    sceneIds.forEach((id, index) => {
      const backgroundColor = getSegmentColor(index, sceneIds.length)
      const color = getSegmentTextColor(backgroundColor)
      sceneColors.set(id, [
        {
          backgroundColor,
          color,
        },
        index,
      ])
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
              "flex justify-center items-center text-sm cursor-pointer text-white",
              index !== currentClipIndex && "opacity-30 hover:opacity-60",
              index === currentClipIndex && "opacity-100",
            )}
            style={{width, ...style}}
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

  const markerCounts = useMemo(() => {
    const counts = new Map<string, MarkerCount>()
    for (const marker of state.data.selectedMarkers ?? []) {
      if (marker.selected) {
        const count = counts.get(marker.title) ?? {total: 0, current: 0}
        counts.set(marker.title, {
          total: count.total + 1,
          current: count.current,
        })
      }
    }
    for (const clip of clips) {
      const marker = state.data.selectedMarkers?.find(
        (m) => m.id === clip.markerId,
      )
      if (marker && marker.title && marker.selected) {
        const count = counts.get(marker.title) ?? {total: 0, current: 0}
        counts.set(marker.title, {
          total: count.total,
          current: count.current + 1,
        })
      }
    }

    return counts
  }, [state.data, clips])

  const [weights, setWeights] = useImmer<Array<[string, number]>>(() => {
    const markerTitles = Array.from(
      new Set(state.data.selectedMarkers?.map((m) => m.title.trim())),
    ).sort()
    if (state.data.clipWeights) {
      return state.data.clipWeights.filter(([title]) =>
        markerTitles.includes(title),
      )
    } else {
      const markerTitles = Array.from(
        new Set(
          state.data
            .selectedMarkers!.filter((m) => m.selected)
            .map((m) => m.title.trim()),
        ),
      ).sort()
      return Array.from(markerTitles).map((title) => [title, 1.0])
    }
  })

  const [enabled, setEnabled] = useState(
    state.data.clipStrategy === "weightedRandom",
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
        type="button"
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
                  !enabled && "opacity-50 cursor-not-allowed",
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

const MarkerOrderModal = () => {
  const [open, setOpen] = useState(false)
  const {state} = useStateMachine()
  const markers = state.data.markers ?? []
  const markerTitles = Array.from(
    new Set(markers.map((m) => m.primaryTag.trim())),
  )

  const onClose = () => {
    setOpen(false)
  }

  return (
    <>
      <button
        onClick={() => setOpen(true)}
        type="button"
        className="btn btn-primary mt-2"
      >
        Set marker order
      </button>
      <Modal position="top" size="fluid" isOpen={open} onClose={onClose}>
        <h1 className="text-2xl font-bold mb-2">Marker order</h1>
        <ul className="flex flex-col gap-1">
          {markerTitles.map((title, idx) => (
            <li
              className="w-full text-center bg-primary text-primary-content px-4 py-1 rounded-full cursor-pointer"
              key={idx}
            >
              {title}
            </li>
          ))}
        </ul>
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

interface SettingsFormProps {
  initialValues: Inputs
  clips: Clip[]
  onRemoveClip: () => void
  onUndo: () => void
  onRedo: () => void
  canUndo: boolean
  canRedo: boolean
  onShiftClips: (direction: "left" | "right") => void
  canShiftLeft: boolean
  canShiftRight: boolean
  confirmBeforeSubmit: boolean
}

const ClipSettingsForm: React.FC<SettingsFormProps> = ({
  initialValues,
  clips,
  onRemoveClip,
  onUndo,
  onRedo,
  canUndo,
  canRedo,
  onShiftClips,
  canShiftLeft,
  canShiftRight,
  confirmBeforeSubmit,
}) => {
  const {register, handleSubmit, watch} = useForm<Inputs>({
    defaultValues: initialValues,
  })
  const doSplitClips = watch("splitClips")
  const measureCountType = watch("measureCountType")

  const revalidator = useRevalidator()
  const {actions, state} = useStateMachine({updateForm})
  const isPmv = state.data.songs?.length !== 0
  const clipOrderType = watch("clipOrder.type")

  const onSubmit = (values: Inputs) => {
    if (
      confirmBeforeSubmit &&
      !window.confirm(
        "You have made manual changes to the clips, this would reset them. Are you sure you want to re-generate the clips?",
      )
    ) {
      return
    }
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
          <div className="w-full flex justify-between mb-4 mt-2">
            <div className="join">
              <div className="tooltip" data-tip="Undo">
                <button
                  disabled={!canUndo}
                  onClick={onUndo}
                  type="button"
                  className="join-item btn btn-sm btn-ghost btn-square"
                >
                  <HiArrowUturnLeft />
                </button>
              </div>
              <div className="tooltip" data-tip="Redo">
                <button
                  disabled={!canRedo}
                  onClick={onRedo}
                  type="button"
                  className="join-item btn btn-sm btn-ghost btn-square"
                >
                  <HiArrowUturnRight />
                </button>
              </div>
            </div>

            <div className="flex gap-2">
              <div className="join">
                <div className="tooltip" data-tip="Move clip left">
                  <button
                    disabled={!canShiftLeft}
                    onClick={() => onShiftClips("left")}
                    className="btn btn-sm btn-ghost join-item"
                    type="button"
                  >
                    <HiBackward />
                  </button>
                </div>
                <div className="tooltip" data-tip="Move clip right">
                  <button
                    disabled={!canShiftRight}
                    onClick={() => onShiftClips("right")}
                    className="btn btn-sm btn-ghost join-item"
                    type="button"
                  >
                    <HiForward />
                  </button>
                </div>
              </div>
              <div className="tooltip" data-tip="Delete current clip">
                <button
                  onClick={onRemoveClip}
                  type="button"
                  className="btn btn-sm btn-error"
                >
                  <HiTrash />
                </button>
              </div>
            </div>
          </div>
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
        <select
          className="select select-bordered"
          {...register("clipOrder.type")}
        >
          <option disabled value="none">
            Select clip ordering
          </option>
          <option value="scene">Scene order</option>
          <option value="fixed">Fixed</option>
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
      {clipOrderType === "fixed" && <MarkerOrderModal />}
      <div className="flex w-full justify-between items-center mt-4">
        <WeightsModal clips={clips} />

        <button type="submit" className="btn btn-primary">
          <HiCheck className="mr-2" />
          Apply
        </button>
      </div>
    </form>
  )
}

function PreviewClips() {
  const revalidator = useRevalidator()
  const [wasRevalidated, setWasRevalidated] = useState(false)

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
  const isPmv = state.data.songs && state.data.songs.length >= 1
  const [songIndex, setSongIndex] = useState(0)

  useEffect(() => {
    if (revalidator.state === "loading") {
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
          muted={videoMuted}
          autoPlay={autoPlay}
          onTimeUpdate={onVideoTimeUpdate}
          ref={videoRef}
        />
        <div className="flex flex-col px-4 py-2 w-1/4 bg-base-200 justify-between">
          <ClipSettingsForm
            clips={clips.map((c) => c.clip)}
            onRemoveClip={onRemoveClip}
            initialValues={{
              clipDuration: state.data.clipDuration || 30,
              clipOrder: state.data.clipOrder || {type: "scene"},
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
            onUndo={undo}
            onRedo={redo}
            canUndo={canUndo}
            canRedo={canRedo}
            onShiftClips={onShiftClips}
            canShiftLeft={currentClipIndex > 0}
            canShiftRight={currentClipIndex < clips.length - 1}
            confirmBeforeSubmit={clipsState.past.length > 0}
          />

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
                autoPlay ? "btn-warning" : "btn-success",
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
