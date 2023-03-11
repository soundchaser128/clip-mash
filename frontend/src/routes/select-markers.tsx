import {useStateMachine} from "little-state-machine"
import {useState} from "react"
import {LoaderFunction, useLoaderData, useNavigate} from "react-router-dom"
import {FormStage, FormState} from "../types/types"
import {updateForm} from "./actions"
import {formatDistance} from "date-fns"

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

const formatDuration = (s: number) =>
  formatDistance(0, s * 1000, {includeSeconds: true})

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
  const [selection, setSelection] = useState(
    () => state.data.selectedMarkers || data.markers.dtos.map((m) => m.id)
  )
  const [filter, setFilter] = useState("")
  const navigate = useNavigate()
  const markers = filterMarkers(data.markers.dtos, filter)

  const totalDuration = formatDuration(
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
    actions.updateForm({
      stage: FormStage.VideoOptions,
      selectedMarkers: selection,
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
        {markers.map((marker) => (
          <article key={marker.id} className="card bg-base-100 shadow-xl">
            <figure>
              <img
                src={marker.screenshotUrl}
                className="aspect-[16/9] object-cover object-top w-full"
              />
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
                {formatDuration(getDuration(marker))}
              </p>
            </div>
            <div className="card-actions justify-end">
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
          </article>
        ))}
      </section>
    </div>
  )
}

export default SelectMarkers
