import {useStateMachine} from "little-state-machine"
import {useState} from "react"
import {useNavigate} from "react-router-dom"
import {FormStage, LocalFilesFormStage} from "../types/types"
import {updateForm} from "../routes/actions"
import clsx from "clsx"
import {useImmer} from "use-immer"
import {
  HiCheck,
  HiChevronRight,
  HiClock,
  HiInformationCircle,
  HiMiniBars2,
  HiUser,
  HiVideoCamera,
  HiXMark,
} from "react-icons/hi2"
import useFuse from "../hooks/useFuse"
import {formatSeconds} from "../helpers"
import {MarkerDto, SelectedMarker} from "../types.generated"

interface Props {
  data: {
    markers: MarkerDto[]
  }
  withImages?: boolean
  withPerformers?: boolean
  nextStage: FormStage | LocalFilesFormStage
}

function round(n: number, places: number): number {
  const factor = Math.pow(10, places)
  return Math.round(n * factor) / factor
}

const SelectMarkers: React.FC<Props> = ({
  data,
  withImages,
  withPerformers,
  nextStage,
}) => {
  const {actions, state} = useStateMachine({updateForm})

  const [selection, setSelection] = useImmer<Record<string, SelectedMarker>>(
    () => {
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
            title: m.primaryTag,
            loops: 1,
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
      .reduce((sum, next) => {
        const duration =
          (next.selectedRange[1] - next.selectedRange[0]) * (next.loops || 1)
        return sum + duration
      }, 0)
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

  const onSetLoops = (id: number, loops: number) => {
    setSelection((draft) => {
      draft[id].loops = loops
    })
  }

  const onNextStage = () => {
    const selectedMarkers = Object.values(selection)
    const hasInteractiveScenes = data.markers
      .filter((m) => !!selection[m.id.id])
      .some((m) => m.sceneInteractive)

    actions.updateForm({
      stage: nextStage,
      selectedMarkers,
      markers: data.markers,
      interactive: hasInteractiveScenes,
    })
    navigate("/stash/music")
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

  const onEqualizeLengths = () => {
    setSelection((draft) => {
      // Find the longest selected marker
      const longestSelectedMarker = Object.values(draft)
        .filter((m) => m.selected)
        .reduce((longest, next) => {
          const len = next.selectedRange[1] - next.selectedRange[0]
          return len > longest ? len : longest
        }, 0)

      // Set loops to match the longest selected marker
      for (const selectedMarker of Object.values(draft)) {
        selectedMarker.loops = round(
          longestSelectedMarker /
            (selectedMarker.selectedRange[1] - selectedMarker.selectedRange[0]),
          2
        )
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
              <button
                className="btn btn-success"
                type="button"
                onClick={onLimitDuration}
              >
                <HiCheck className="mr-1" />
                Apply
              </button>
            </div>
          </div>

          <div
            className="tooltip"
            data-tip="Makes the markers loop to match the duration of the longest selected marker."
          >
            <button className="btn" type="button" onClick={onEqualizeLengths}>
              <HiMiniBars2 className="mr-1" />
              Equalize lengths
            </button>
          </div>
        </div>

        <div className="flex gap-2 justify-center">
          <button onClick={onDeselectAll} className="btn btn-error">
            <HiXMark className="mr-1" />
            Deselect all
          </button>
          <button onClick={onSelectAll} className="btn btn-secondary">
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
      <section className="grid grid-cols-1 lg:grid-cols-4 gap-4 w-full">
        {markers.map((marker) => {
          const selectedMarker = selection[marker.id.id]
          return (
            <article
              key={marker.id.id}
              className={clsx(
                "card card-compact bg-base-200 shadow-xl",
                !selectedMarker.selected && "opacity-50"
              )}
            >
              {withImages && (
                <figure>
                  {videoPreview === marker.id.id && (
                    <video muted autoPlay src={marker.streamUrl} />
                  )}
                  {videoPreview !== marker.id.id && (
                    <img
                      src={marker.screenshotUrl || undefined}
                      className="aspect-[16/9] object-cover object-top w-full"
                    />
                  )}
                </figure>
              )}

              <div className="card-body">
                <h2 className="card-title">
                  {[marker.primaryTag, ...marker.tags].join(", ")}
                </h2>
                <p className="truncate">
                  <strong>
                    <HiVideoCamera className="mr-2 inline" />
                    Scene:{" "}
                  </strong>
                  {marker.sceneTitle || marker.fileName}
                </p>
                {withPerformers && (
                  <p className="truncate">
                    <strong>
                      <HiUser className="mr-2 inline" />
                      Performers:{" "}
                    </strong>
                    {marker.performers.join(", ") || "No performers found"}
                  </p>
                )}
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
                      min={0}
                      type="range"
                      className="range w-full"
                    />
                  </div>

                  <div className="grid grid-rows-2">
                    <div className="form-control">
                      <label className="label cursor-pointer">
                        <span className="label-text">Include</span>
                        <input
                          type="checkbox"
                          className="toggle toggle-sm toggle-primary"
                          checked={!!selectedMarker.selected}
                          onChange={(e) =>
                            onCheckboxChange(marker.id.id, e.target.checked)
                          }
                        />
                      </label>
                    </div>
                    {withImages && (
                      <div className="form-control">
                        <label className="label cursor-pointer">
                          <span className="label-text">Video preview</span>
                          <input
                            onChange={(e) =>
                              onVideoPreviewChange(
                                marker.id.id,
                                e.target.checked
                              )
                            }
                            checked={videoPreview === marker.id.id}
                            disabled={!selectedMarker.selected}
                            type="checkbox"
                            className="toggle toggle-sm"
                          />
                        </label>
                      </div>
                    )}

                    <div className="flex flex-row justify-between">
                      <label className="label">
                        <span className="label-text grow">Loops</span>
                      </label>
                      <input
                        type="number"
                        className="input input-sm input-bordered w-24"
                        value={selectedMarker.loops || 1}
                        disabled={!selectedMarker.selected}
                        onChange={(e) =>
                          onSetLoops(marker.id.id, e.target.valueAsNumber)
                        }
                      />
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
