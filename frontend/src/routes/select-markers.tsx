import {useStateMachine} from "little-state-machine"
import {useState} from "react"
import {LoaderFunction, useLoaderData, useNavigate} from "react-router-dom"
import {FormStage, Marker, SelectedMarker, StateHelpers} from "../types/types"
import {updateForm} from "./actions"
import clsx from "clsx"
import {useImmer} from "use-immer"
import {
  HiCheck,
  HiChevronRight,
  HiClock,
  HiInformationCircle,
  HiUser,
  HiVideoCamera,
  HiXMark,
} from "react-icons/hi2"
import useFuse from "../hooks/useFuse"
import invariant from "tiny-invariant"
import {formatSeconds, getFormState} from "../helpers"

interface Data {
  markers: Marker[]
}

export const loader: LoaderFunction = async () => {
  const state = getFormState()
  if (state) {
    invariant(StateHelpers.isStash(state))
    const params = new URLSearchParams()
    params.set("selectedIds", state.selectedIds!.join(","))
    params.set("mode", state.selectMode!)
    params.set("includeAll", state.includeAll ? "true" : "false")
    const url = `/api/stash/markers?${params.toString()}`
    const response = await fetch(url)
    const markers = await response.json()
    return {markers} satisfies Data
  } else {
    return null
  }
}

function SelectMarkers() {
  const {actions, state} = useStateMachine({updateForm})
  invariant(StateHelpers.isStash(state.data))
  const data = useLoaderData() as Data

  const [selection, setSelection] = useImmer<Record<string, SelectedMarker>>(
    () => {
      invariant(StateHelpers.isStash(state.data))
      const entries =
        state.data.selectedMarkers?.map((m) => [m.id.id, m]) ||
        data.markers.map((m) => [
          m.id.id,
          {
            id: m.id,
            indexWithinVideo: m.indexWithinVideo,
            videoId: m.videoId,
            selected: true,
            selectedRange: [m.start, m.end],
          } satisfies SelectedMarker,
        ])
      return Object.fromEntries(entries)
    }
  )
  const [filter, setFilter] = useState("")
  const [videoPreview, setVideoPreview] = useState<number>()
  const navigate = useNavigate()
  const markers = useFuse({
    items: data.markers,
    query: filter,
    keys: ["performers", "primaryTag", "sceneTitle", "tags"],
  })

  const [maxMarkerLength, setMaxMarkerLength] = useState<number>()
  const allDisabled = Object.values(selection).every((m) => !m.selected)

  const onVideoPreviewChange = (id: number, checked: boolean) => {
    if (checked) {
      setVideoPreview(id)
    } else {
      setVideoPreview(undefined)
    }
  }

  const totalDuration = formatSeconds(
    Object.values(selection)
      .filter((m) => m.selected)
      .reduce(
        (sum, next) => sum + (next.selectedRange[1] - next.selectedRange[0]),
        0
      )
  )

  const onCheckboxChange = (id: number, checked: boolean) => {
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
    const selectedMarkers = Object.values(selection)
    const hasInteractiveScenes = data.markers
      .filter((m) => !!selection[m.id.id])
      .some((m) => m.sceneInteractive)
    console.log(selectedMarkers)

    actions.updateForm({
      stage: FormStage.VideoOptions,
      selectedMarkers,
      markers: data.markers,
      interactive: hasInteractiveScenes,
    })
    navigate("/stash/video-options")
  }

  const onLimitDuration = () => {
    setSelection((draft) => {
      for (const selectedMarker of Object.values(draft)) {
        const originalMarker = data.markers.find(
          (m) => m.id.id === selectedMarker.id.id
        )!
        const start = selectedMarker.selectedRange[0]
        const maxLen =
          maxMarkerLength || originalMarker.end - originalMarker.start
        selectedMarker.selectedRange = [
          start,
          Math.min(start + maxLen, originalMarker.end),
        ]
      }
    })
  }

  return (
    <div>
      <div className="w-full flex justify-between items-center">
        <div className="text-center">
          Estimated total duration of video: <strong>{totalDuration}</strong>
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
      <div className="w-full flex flex-col lg:flex-row justify-between gap-4 my-4">
        <div className="flex flex-col lg:flex-row gap-2">
          <input
            type="text"
            placeholder="Filter..."
            className="input input-bordered w-full lg:w-96"
            value={filter}
            onChange={(e) => setFilter(e.target.value)}
          />

          <div className="form-control">
            <div className="input-group">
              <input
                type="number"
                placeholder="Limit maximum marker length (in seconds)"
                className="input input-bordered w-full lg:w-96"
                value={maxMarkerLength || ""}
                onChange={(e) => setMaxMarkerLength(e.target.valueAsNumber)}
              />
              <button className="btn" type="button" onClick={onLimitDuration}>
                Apply
              </button>
            </div>
          </div>
        </div>

        <div className="flex gap-2 justify-center">
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
      {markers.length === 0 && (
        <div className="mt-4 alert alert-info w-fit">
          <HiInformationCircle className="stroke-current flex-shrink-0 h-6 w-6" />
          <span>
            No markers found for selection. Either create some scene markers in
            Stash or change your search criteria.
          </span>
        </div>
      )}
      <section className="grid grid-cols-1 lg:grid-cols-4 gap-2 w-full">
        {markers.map((marker) => {
          const selectedMarker = selection[marker.id.id]
          return (
            <article
              key={marker.id.id}
              className={clsx(
                "card card-compact bg-base-100 shadow-xl",
                !selectedMarker.selected && "opacity-50"
              )}
            >
              <figure>
                {videoPreview === marker.id.id && (
                  <video muted autoPlay src={marker.streamUrl} />
                )}
                {videoPreview !== marker.id.id && (
                  <img
                    src={marker.screenshotUrl}
                    className="aspect-[16/9] object-cover object-top w-full"
                  />
                )}
              </figure>
              <div className="card-body">
                <h2 className="card-title">
                  {[marker.primaryTag, ...marker.tags].join(", ")}
                </h2>
                <p>
                  <strong>
                    <HiVideoCamera className="mr-2 inline" />
                    Scene:{" "}
                  </strong>
                  {marker.sceneTitle || marker.fileName}
                </p>
                <p>
                  <strong>
                    <HiUser className="mr-2 inline" />
                    Performers:{" "}
                  </strong>
                  {marker.performers.join(", ") || "No performers found"}
                </p>
                <p>
                  <strong>
                    <HiClock className="mr-2 inline" />
                    Selected duration:{" "}
                  </strong>
                  {formatSeconds(selectedMarker.selectedRange, "short")} /{" "}
                  {formatSeconds(marker.end - marker.start, "short")}
                </p>
                <div className="">
                  <div className="w-full">
                    <input
                      value={
                        selectedMarker.selectedRange[1] -
                        selectedMarker.selectedRange[0]
                      }
                      onChange={(e) =>
                        setSelection((draft) => {
                          const start = draft[marker.id.id].selectedRange[0]
                          draft[marker.id.id].selectedRange[1] =
                            start + e.target.valueAsNumber
                        })
                      }
                      disabled={!selectedMarker.selected}
                      max={marker.end - marker.start}
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
                            onVideoPreviewChange(marker.id.id, e.target.checked)
                          }
                          checked={videoPreview === marker.id.id}
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
                            onCheckboxChange(marker.id.id, e.target.checked)
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
