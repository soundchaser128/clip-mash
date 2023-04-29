import {useStateMachine} from "little-state-machine"
import {useMemo, useState} from "react"
import {
  LoaderFunction,
  json,
  useLoaderData,
  useNavigate,
} from "react-router-dom"
import {Clip, FormStage, Scene, StateHelpers} from "../types/types"
import {updateForm} from "./actions"
import {HiChevronRight, HiPause, HiPlay} from "react-icons/hi2"
import clsx from "clsx"
import {useRef} from "react"
import {useImmer} from "use-immer"
import invariant from "tiny-invariant"
import {getFormState, getSegmentColor} from "../helpers"

interface ClipsResponse {
  clips: Clip[]
  streams: Record<string, string>
  scenes: Scene[]
}

interface Data {
  clips: Clip[]
  streams: Record<string, string>
  scenes: Record<string, Scene>
}

export const loader: LoaderFunction = async () => {
  const state = getFormState()
  invariant(StateHelpers.isStash(state!))
  const response = await fetch("/api/clips", {
    method: "POST",
    body: JSON.stringify({
      clipOrder: state.clipOrder,
      clipDuration: state.clipDuration,
      selectedMarkers: state.selectedMarkers,
      markers: state.markers,
      selectMode: state.selectMode,
      splitClips: state.splitClips,
    }),
    headers: {"content-type": "application/json"},
  })
  if (response.ok) {
    const data: ClipsResponse = await response.json()

    const scenes: Record<string, Scene> = {}
    data.scenes.forEach((s) => {
      scenes[s.id] = s
    })

    return {
      ...data,
      scenes,
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

function PreviewClips() {
  const loaderData = useLoaderData() as Data
  const streams = loaderData.streams
  const [clips, setClips] = useImmer<ClipState[]>(
    loaderData.clips.map((clip) => ({clip, included: true}))
  )

  const [currentClipIndex, setCurrentClipIndex] = useState(0)
  const [autoPlay, setAutoPlay] = useState(false)
  const currentClip = clips[currentClipIndex].clip
  const streamUrl = streams[currentClip.sceneId]
  const clipUrl = `${streamUrl}#t=${currentClip.range[0]},${currentClip.range[1]}`
  const {actions} = useStateMachine({updateForm})
  const navigate = useNavigate()
  const videoRef = useRef<HTMLVideoElement>(null)

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

    const sceneIds = Array.from(new Set(clips.map((c) => c.clip.sceneId)))
    sceneIds.sort()
    const sceneColors = new Map()
    sceneIds.forEach((id, index) => {
      sceneColors.set(id, getSegmentColor(index))
    })

    return [segments, sceneColors]
  }, [clips])

  const onTimeUpdate: React.ReactEventHandler<HTMLVideoElement> = (event) => {
    const endTimestamp = currentClip.range[1]
    const currentTime = event.currentTarget.currentTime
    if (Math.abs(endTimestamp - currentTime) <= 0.5 && autoPlay) {
      setCurrentClipIndex((c) => c + 1)
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
        <p className=" text-center text-xl">
          Showing clip{" "}
          <strong>
            {currentClipIndex + 1} / {clips.length}
          </strong>
        </p>

        <button
          type="button"
          onClick={onNextStage}
          className="btn btn-success place-self-end"
        >
          Next
          <HiChevronRight className="ml-1" />
        </button>
      </div>

      <video
        className="max-h-[75vh]"
        src={clipUrl}
        muted
        autoPlay={autoPlay}
        onTimeUpdate={onTimeUpdate}
        ref={videoRef}
      />

      <div className="w-full h-8 flex mt-2 gap-0.5">
        {segments.map((width, index) => {
          const clip = clips[index].clip
          const scene = loaderData.scenes[clip.sceneId]
          return (
            <div
              key={index}
              data-tip={`${scene.performers.join(", ")} - ${scene.title}`}
              className={clsx(
                "h-full tooltip transition-opacity",
                sceneColors.get(clip.sceneId),
                index !== currentClipIndex &&
                  "bg-opacity-25 hover:bg-opacity-50"
              )}
              style={{width}}
              onClick={() => setCurrentClipIndex(index)}
            />
          )
        })}
      </div>

      <div className="flex justify-between mt-4">
        <button
          className="btn"
          onClick={() => setCurrentClipIndex((i) => i - 1)}
          disabled={currentClipIndex === 0}
        >
          Previous clip
        </button>
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
          <div className="form-control">
            <label className="label cursor-pointer">
              <span className="label-text mr-2">Included in compilation</span>
              <input
                type="checkbox"
                className="toggle"
                checked={clips[currentClipIndex].included}
                onChange={(e) =>
                  setClips((draft) => {
                    draft[currentClipIndex].included = e.target.checked
                  })
                }
              />
            </label>
          </div>
        </div>
        <button
          className="btn"
          onClick={() => setCurrentClipIndex((i) => i + 1)}
          disabled={currentClipIndex >= clips.length - 1}
        >
          Next clip
        </button>
      </div>
    </>
  )
}

export default PreviewClips
