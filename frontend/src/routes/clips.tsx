import {useStateMachine} from "little-state-machine"
import {useMemo, useState} from "react"
import {LoaderFunction, useLoaderData, useNavigate} from "react-router-dom"
import {Clip, FormStage, FormState, Scene} from "../types/types"
import {updateForm} from "./actions"
import {HiChevronRight} from "react-icons/hi2"
import clsx from "clsx"

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
  const json = sessionStorage.getItem("form-state")
  const state: {data: FormState} = JSON.parse(json!)
  const response = await fetch("/api/clips", {
    method: "POST",
    body: JSON.stringify({
      clipOrder: state.data.clipOrder,
      clipDuration: state.data.clipDuration,
      selectedMarkers: state.data.selectedMarkers,
      markers: state.data.markers,
      selectMode: state.data.selectMode,
    }),
    headers: {"content-type": "application/json"},
  })
  const data: ClipsResponse = await response.json()

  const scenes: Record<string, Scene> = {}
  data.scenes.forEach((s) => {
    scenes[s.id] = s
  })

  return {
    ...data,
    scenes,
  } satisfies Data
}

const segmentColors = [
  "bg-purple-400",
  "bg-green-400",
  "bg-yellow-400",
  "bg-red-400",
  "bg-teal-400",
  "bg-orange-600",
  "bg-rose-400",
]

function PreviewClips() {
  const data = useLoaderData() as Data
  const [currentClipIndex, setCurrentClipIndex] = useState(0)
  const currentClip = data.clips[currentClipIndex]
  const streamUrl = data.streams[currentClip.sceneId]
  const clipUrl = `${streamUrl}#t=${currentClip.range[0]},${currentClip.range[1]}`
  const {actions} = useStateMachine({updateForm})
  const navigate = useNavigate()

  const onNextStage = () => {
    actions.updateForm({
      stage: FormStage.Wait,
      clips: data.clips,
    })
    navigate("/progress")
  }

  const [segments, sceneColors] = useMemo(() => {
    const clipLengths = data.clips.map((clip) => clip.range[1] - clip.range[0])
    const total = clipLengths.reduce((total, len) => total + len, 0)
    const segments = clipLengths.map((len) => `${(len / total) * 100}%`)

    const sceneIds = Array.from(new Set(data.clips.map((c) => c.sceneId)))
    sceneIds.sort()
    const sceneColors = new Map()
    sceneIds.forEach((id, index) => {
      sceneColors.set(id, segmentColors[index % segmentColors.length])
    })

    return [segments, sceneColors]
  }, [data])

  return (
    <>
      <div className="mb-4 grid grid-cols-3 items-center">
        <div></div>
        <p className=" text-center text-xl">
          Showing clip{" "}
          <strong>
            {currentClipIndex + 1} / {data.clips.length}
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

      <video className="max-h-screen" src={clipUrl} muted controls autoPlay />

      <div className="w-full h-8 flex mt-2 gap-0.5">
        {segments.map((width, index) => {
          const clip = data.clips[index]
          const scene = data.scenes[clip.sceneId]
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
        <button
          className="btn"
          onClick={() => setCurrentClipIndex((i) => i + 1)}
          disabled={currentClipIndex >= data.clips.length - 1}
        >
          Next clip
        </button>
      </div>
    </>
  )
}

export default PreviewClips
