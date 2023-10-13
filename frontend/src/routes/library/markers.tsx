import {useStateMachine} from "little-state-machine"
import {useState} from "react"
import {useLoaderData, useNavigate, useRevalidator} from "react-router-dom"
import clsx from "clsx"
import {useImmer} from "use-immer"
import {
  HiCalendar,
  HiCheck,
  HiChevronRight,
  HiClock,
  HiInformationCircle,
  HiMiniBars2,
  HiVideoCamera,
  HiXMark,
} from "react-icons/hi2"
import {MarkerDto, SelectedMarker} from "../../api"
import {updateForm} from "../actions"
import {dateTimeFormat, formatSeconds, sumDurations} from "../../helpers"
import {FormStage} from "../../types/form-state"
import JumpToTop from "../../components/JumpToTop"
import EditableText from "../../components/EditableText"
import {updateMarker} from "../../api"
import useFuse from "../../hooks/useFuse"

const SelectMarkers: React.FC = () => {
  const initialMarkers = useLoaderData() as MarkerDto[]
  const {actions, state} = useStateMachine({updateForm})
  const revalidator = useRevalidator()

  const [selection, setSelection] = useImmer<Record<string, SelectedMarker>>(
    () => {
      const entries =
        state.data.selectedMarkers?.map((m) => [m.id, m]) ||
        initialMarkers.map((m) => [
          m.id,
          {
            id: m.id,
            indexWithinVideo: m.indexWithinVideo,
            videoId: m.videoId,
            selected: true,
            selectedRange: [m.start, m.end],
            title: m.primaryTag,
            loops: 1,
            source: m.source,
          } satisfies SelectedMarker,
        ])
      return Object.fromEntries(entries)
    },
  )
  const [filter, setFilter] = useState("")
  const [videoPreview, setVideoPreview] = useState<number>()
  const navigate = useNavigate()
  const markers = useFuse({
    query: filter,
    keys: ["performers", "primaryTag", "sceneTitle", "tags"],
    items: initialMarkers,
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

  const totalDuration = formatSeconds(sumDurations(Object.values(selection)))

  const onCheckboxChange = (id: number, checked: boolean) => {
    setSelection((draft) => {
      draft[id].selected = checked
    })
  }

  const onCheckboxToggle = (id: number) => {
    setSelection((draft) => {
      draft[id].selected = !draft[id].selected
    })
  }

  const onDeselectAll = () => {
    setSelection((draft) => {
      for (const marker of markers) {
        draft[marker.id].selected = false
      }
    })
  }

  const onSelectAll = () => {
    setSelection((draft) => {
      for (const marker of markers) {
        draft[marker.id].selected = true
      }
    })
  }

  const onSetLoops = (id: number, loops: number) => {
    setSelection((draft) => {
      draft[id].loops = loops
    })
  }

  const onNextStage = () => {
    const selectedMarkers = Object.values(selection)
    const hasInteractiveScenes = initialMarkers
      .filter((m) => !!selection[m.id])
      .some((m) => m.sceneInteractive)

    actions.updateForm({
      stage: FormStage.Music,
      selectedMarkers,
      markers: initialMarkers,
      interactive: hasInteractiveScenes,
    })
    navigate("/music")
  }

  const onLimitDuration = () => {
    setSelection((draft) => {
      for (const selectedMarker of Object.values(draft)) {
        const originalMarker = initialMarkers.find(
          (m) => m.id === selectedMarker.id,
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
        if (!selectedMarker.selected) continue
        const [start, end] = selectedMarker.selectedRange
        selectedMarker.loops = Math.floor(longestSelectedMarker / (end - start))
      }
    })
  }

  const onUpdateTitle = async (id: number, title: string) => {
    await updateMarker(id, {title})
    setSelection((draft) => {
      const marker = draft[id]
      marker.title = title
    })
    revalidator.revalidate()
  }

  return (
    <div>
      <JumpToTop />
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
      <section className="grid grid-cols-1 lg:grid-cols-3 gap-4 w-full">
        {markers.map((marker) => {
          const selectedMarker = selection[marker.id]
          const streamUrl = `${marker.streamUrl}#t=${marker.start},${marker.end}`
          const date = new Date(marker.createdOn * 1000)
          const isoDate = date.toISOString()
          const humanDate = dateTimeFormat.format(date)

          return (
            <article
              key={marker.id}
              className={clsx(
                "card card-compact bg-base-200 shadow-xl",
                !selectedMarker.selected && "opacity-50",
              )}
            >
              <figure>
                {videoPreview === marker.id && (
                  <video
                    muted
                    autoPlay
                    src={streamUrl}
                    width={499}
                    height={281}
                  />
                )}
                {videoPreview !== marker.id && (
                  <img
                    src={marker.screenshotUrl}
                    className="aspect-[16/9] object-cover object-top w-full cursor-pointer"
                    onClick={() => onCheckboxToggle(marker.id)}
                    width={499}
                    height={281}
                  />
                )}
              </figure>

              <div className="card-body">
                <h2 className="card-title">
                  <EditableText
                    value={marker.primaryTag}
                    onSave={(title) => onUpdateTitle(marker.id, title)}
                  />
                </h2>
                <p className="truncate">
                  <strong>
                    <HiVideoCamera className="mr-2 inline" />
                    Scene:{" "}
                  </strong>
                  {marker.sceneTitle || marker.fileName}
                </p>
                <p>
                  <strong>
                    <HiClock className="mr-2 inline" />
                    Selected duration:{" "}
                  </strong>
                  {formatSeconds(selectedMarker.selectedRange, "short")} /{" "}
                  {formatSeconds(marker.end - marker.start, "short")}
                </p>
                <p>
                  <strong>
                    <HiCalendar className="mr-2 inline" />
                    Created:{" "}
                  </strong>
                  <time dateTime={isoDate}>{humanDate}</time>
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
                          const start = draft[marker.id].selectedRange[0]
                          draft[marker.id].selectedRange[1] =
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
                            onCheckboxChange(marker.id, e.target.checked)
                          }
                        />
                      </label>
                    </div>
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
                          className="toggle toggle-sm"
                        />
                      </label>
                    </div>

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
                          onSetLoops(marker.id, e.target.valueAsNumber)
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
