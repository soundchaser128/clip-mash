import {useStateMachine} from "little-state-machine"
import {useState} from "react"
import {LoaderFunction, useLoaderData, useNavigate} from "react-router-dom"
import {FormStage, FormState, SelectedMarker} from "../types/types"
import {updateForm} from "./actions"
import {formatDuration} from "date-fns"
import clsx from "clsx"
import {useImmer} from "use-immer"
import {HiCheck, HiChevronRight, HiXMark} from "react-icons/hi2"

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
  const date = new Date(s * 1000)
  return formatDuration(
    {
      hours: date.getUTCHours(),
      minutes: date.getUTCMinutes(),
      seconds: date.getUTCSeconds(),
    },
    {format: ["hours", "minutes", "seconds"]}
  )
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
  const {actions, state} = useStateMachine({updateForm})
  const data = useLoaderData() as Data

  const [selection, setSelection] = useImmer<Record<string, SelectedMarker>>(
    () => {
      const entries =
        state.data.selectedMarkers?.map((m) => [m.id, m]) ||
        data.markers.dtos.map((m) => [m.id, {...m, selected: true}])
      return Object.fromEntries(entries)
    }
  )
  const [filter, setFilter] = useState("")
  const [videoPreview, setVideoPreview] = useState<string>()
  const navigate = useNavigate()
  const markers = filterMarkers(data.markers.dtos, filter)
  const [maxMarkerLength, setMaxMarkerLength] = useState<number>()
  const allDisabled = Object.values(selection).every((m) => !m.selected)

  const onVideoPreviewChange = (id: string, checked: boolean) => {
    if (checked) {
      setVideoPreview(id)
    } else {
      setVideoPreview(undefined)
    }
  }

  const totalDuration = formatSeconds(
    Object.values(selection).reduce(
      (sum, next) => sum + (next.duration || 0),
      0
    )
  )

  const onCheckboxChange = (id: string, checked: boolean) => {
    setSelection((draft) => {
      draft[id].selected = checked
    })
  }

  const onDeselectAll = () => {
    setSelection((draft) => {
      Object.values(draft).forEach((e) => {
        e.selected = false
      })
    })
  }

  const onSelectAll = () => {
    setSelection((draft) => {
      Object.values(draft).forEach((e) => {
        e.selected = true
      })
    })
  }

  const onNextStage = () => {
    const selectedMarkers = Object.values(selection).filter((m) => m.selected)

    actions.updateForm({
      stage: FormStage.VideoOptions,
      selectedMarkers,
      markers: data.markers.dtos,
    })
    navigate("/video-options")
  }

  const onDurationBlur = () => {
    setSelection((draft) => {
      Object.values(draft).forEach((selectedMarker) => {
        const marker = markers.find((m) => m.id === selectedMarker.id)!
        const defaultDuration = getDuration(marker)
        const maxLen = maxMarkerLength || defaultDuration
        selectedMarker.duration =
          selectedMarker.duration >= maxLen ? maxLen : defaultDuration
      })
    })
  }

  return (
    <div>
      <div className="w-full flex justify-between items-center">
        <div className="text-center">
          Estimated total duration: <strong>{totalDuration}</strong>
        </div>
        <button
          type="button"
          onClick={onNextStage}
          className="btn btn-success place-self-end"
          disabled={allDisabled}
        >
          Next
          <HiChevronRight className="ml-1" />
        </button>
      </div>
      <div className="w-full flex justify-between my-4">
        <div className="flex gap-2">
          <input
            type="text"
            placeholder="Filter..."
            className="input input-bordered w-80"
            value={filter}
            onChange={(e) => setFilter(e.target.value)}
          />

          <input
            type="number"
            className="input input-bordered w-80"
            placeholder="Limit maximum marker length (in seconds)"
            value={maxMarkerLength || ""}
            onChange={(e) => {
              const num = e.target.valueAsNumber
              setMaxMarkerLength(num)
            }}
            onBlur={onDurationBlur}
          />
        </div>

        <div className="flex gap-2">
          <button onClick={onDeselectAll} className="btn btn-error">
            <HiXMark className="mr-1" />
            Deselect all
          </button>
          <button onClick={onSelectAll} className="btn btn-primary">
            <HiCheck className="mr-1" />
            Select all
          </button>
        </div>
      </div>
      <section className="grid grid-cols-4 gap-2 w-full">
        {markers.map((marker, index) => {
          const selectedMarker = selection[marker.id]!
          return (
            <article
              key={marker.id}
              className={clsx(
                "card card-compact bg-base-100 shadow-xl",
                !selectedMarker.selected && "opacity-50"
              )}
            >
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
                  <strong>Selected duration: </strong>
                  {formatSeconds(selectedMarker.duration)}
                </p>
                <div className="">
                  <div className="w-full">
                    <input
                      value={selectedMarker.duration}
                      onChange={(e) =>
                        setSelection((draft) => {
                          draft[marker.id].duration = e.target.valueAsNumber
                        })
                      }
                      disabled={!selectedMarker.selected}
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
                          disabled={!selectedMarker.selected}
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
                          checked={selectedMarker.selected}
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
          )
        })}
      </section>
    </div>
  )
}

export default SelectMarkers
