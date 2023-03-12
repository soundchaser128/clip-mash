import {useStateMachine} from "little-state-machine"
import {useState} from "react"
import {LoaderFunction, useLoaderData, useNavigate} from "react-router-dom"
import {FormStage, FormState, SelectedMarker} from "../types/types"
import {updateForm} from "./actions"
import {formatDistance, formatDuration} from "date-fns"
import produce from "immer"

interface Marker {
  id: string
  primaryTag: string
  streamUrl: string
  screenshotUrl: string
  start: number
  end?: number
  sceneTitle?: string
  performers: string[]
  fileName: string
}

interface Data {
  markers: {
    dtos: Marker[]
    gql: unknown[]
  }
}

export const loader: LoaderFunction = async () => {
  const json = sessionStorage.getItem("form-state")
  if (json) {
    const state: {data: FormState} = JSON.parse(json)
    const params = new URLSearchParams()
    params.set("selectedIds", state.data.selectedIds!.join(","))
    params.set("mode", state.data.selectMode!)
    const url = `/api/markers?${params.toString()}`
    const response = await fetch(url)
    const markers = await response.json()
    return {markers} satisfies Data
  } else {
    return null
  }
}

function getDuration({start, end}: Marker): number {
  if (end) {
    return end - start
  } else {
    return 15
  }
}

function formatSeconds(s: number) {
  // const date =
  return formatDuration({seconds: s}, {format: ["hours", "minutes", "seconds"]})
}

function filterMarkers(markers: Marker[], filter?: string) {
  if (!filter || filter.trim().length === 0) {
    return markers
  } else {
    const regex = new RegExp(filter, "i")
    return markers.filter(
      (m) =>
        regex.test(m.fileName) ||
        regex.test(m.primaryTag) ||
        regex.test(m.performers.join(" "))
    )
  }
}

function SelectMarkers() {
  const {state, actions} = useStateMachine({updateForm})
  const data = useLoaderData() as Data
  const [selection, setSelection] = useState<string[]>(
    () =>
      state.data.selectedMarkers?.map((s) => s.id) ||
      data.markers.dtos.map((m) => m.id)
  )
  const [durations, setDurations] = useState<number[]>(
    data.markers.dtos.map((m) => getDuration(m))
  )
  const [filter, setFilter] = useState("")
  const [videoPreview, setVideoPreview] = useState<string>()
  const navigate = useNavigate()
  const markers = filterMarkers(data.markers.dtos, filter)

  const onVideoPreviewChange = (id: string, checked: boolean) => {
    if (checked) {
      setVideoPreview(id)
    } else {
      setVideoPreview(undefined)
    }
  }

  const totalDuration = formatSeconds(
    markers
      .filter((m) => selection.includes(m.id))
      .reduce((total, marker) => total + getDuration(marker), 0)
  )

  const onCheckboxChange = (id: string, checked: boolean) => {
    if (checked) {
      setSelection((s) => [...s, id])
    } else {
      setSelection((s) => s.filter((string) => string !== id))
    }
  }

  const onNextStage = () => {
    const selectedMarkers = []
    for (let i = 0; i < selection.length; i++) {
      const duration = durations[i]
      const selected = selection[i]
    }

    actions.updateForm({
      stage: FormStage.VideoOptions,
      selectedMarkers: [], // todo,
      markers: data.markers.gql,
    })
    navigate("/video-options")
  }

  return (
    <div>
      <div className="w-full grid grid-cols-3 mb-4 items-baseline">
        <input
          type="text"
          placeholder="Filter..."
          className="input input-bordered w-64"
          value={filter}
          onChange={(e) => setFilter(e.target.value)}
        />
        <div className="text-center">
          Estimated total duration: <strong>{totalDuration}</strong>
        </div>
        <button
          type="button"
          onClick={onNextStage}
          className="btn btn-success place-self-end"
          disabled={selection.length === 0}
        >
          Next
        </button>
      </div>
      <section className="grid grid-cols-4 gap-2 w-full">
        {markers.map((marker, index) => (
          <article key={marker.id} className="card bg-base-100 shadow-xl">
            <figure>
              {videoPreview === marker.id && (
                <video muted autoPlay src={marker.streamUrl} />
              )}
              {videoPreview !== marker.id && (
                <img
                  src={marker.screenshotUrl}
                  className="aspect-[16/9] object-cover object-top w-full"
                />
              )}
            </figure>
            <div className="card-body">
              <h2 className="card-title">{marker.primaryTag}</h2>
              <p>
                <strong>Scene: </strong>
                {marker.sceneTitle || marker.fileName}
              </p>
              <p>
                <strong>Performers: </strong>
                {marker.performers.join(", ") || "No performers found"}
              </p>
              <p>
                <strong>Duration: </strong>
                {formatSeconds(durations[index])}
              </p>
              <div className="">
                <div className="w-full">
                  <input
                    value={durations[index]}
                    onChange={(e) =>
                      setDurations((durations) =>
                        produce(durations, (draft) => {
                          draft[index] = e.target.valueAsNumber
                        })
                      )
                    }
                    max={getDuration(marker)}
                    min={15}
                    type="range"
                    className="range range-primary w-full"
                  />
                </div>

                <div className="card-actions justify-between">
                  <div className="form-control">
                    <label className="label cursor-pointer">
                      <span className="label-text">Video preview</span>
                      <input
                        onChange={(e) =>
                          onVideoPreviewChange(marker.id, e.target.checked)
                        }
                        checked={videoPreview === marker.id}
                        type="checkbox"
                        className="toggle ml-2"
                      />
                    </label>
                  </div>
                  <div className="form-control">
                    <label className="label cursor-pointer">
                      <span className="label-text">Include</span>
                      <input
                        type="checkbox"
                        className="checkbox checkbox-primary ml-2"
                        checked={selection.includes(marker.id)}
                        onChange={(e) =>
                          onCheckboxChange(marker.id, e.target.checked)
                        }
                      />
                    </label>
                  </div>
                </div>
              </div>
            </div>
          </article>
        ))}
      </section>
    </div>
  )
}

export default SelectMarkers
