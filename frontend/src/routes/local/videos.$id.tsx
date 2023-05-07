import {format, formatDuration, getMilliseconds, parse} from "date-fns"
import {VideoWithMarkers, Marker} from "../../types/types"
import clsx from "clsx"
import {useRef, useState} from "react"
import {useForm, Controller} from "react-hook-form"
import {
  HiClock,
  HiTrash,
  HiXMark,
  HiPlus,
  HiTag,
  HiCheck,
  HiPencilSquare,
} from "react-icons/hi2"
import {useImmer} from "use-immer"
import {getSegmentColor} from "../../helpers"
import Modal from "../../components/Modal"
import {
  useNavigate,
  useParams,
  useRevalidator,
  useRouteLoaderData,
} from "react-router-dom"
interface Inputs {
  id?: number
  title: string
  start: number
  end: number
}

function formatSeconds(seconds?: number): string {
  return typeof seconds !== "undefined" ? format(seconds * 1000, "mm:ss") : ""
}

function parseSeconds(string: string): number {
  return getMilliseconds(parse(string, "mm:ss", new Date())) / 1000.0
}

interface Segment {
  offset: number
  width: number
}

function getSegments(
  duration: number | undefined,
  markers: Marker[]
): Segment[] {
  if (typeof duration !== "undefined" && !isNaN(duration)) {
    const totalDuration = duration
    const result = []
    for (const marker of markers) {
      const offset = (marker.start / totalDuration) * 100
      const seconds = marker.end - marker.start
      const width = (seconds / totalDuration) * 100
      result.push({
        offset,
        width,
      })
    }

    return result
  } else {
    return []
  }
}

type FormMode = "hidden" | "create" | "edit"

interface CreateMarker {
  videoId: string
  start: number
  end: number
  title: string
  indexWithinVideo: number
}

async function persistMarker(
  videoId: string,
  marker: Inputs,
  duration: number,
  index: number
): Promise<Marker> {
  const payload = {
    start: Math.max(marker.start, 0),
    end: Math.min(marker.end, duration),
    title: marker.title.trim(),
    videoId,
    indexWithinVideo: index,
  } satisfies CreateMarker

  const response = await fetch("/api/local/video/marker", {
    method: "POST",
    body: JSON.stringify(payload),
    headers: {"Content-Type": "application/json"},
  })

  // TODO error handling
  return await response.json()
}

export default function EditVideoModal() {
  const {id} = useParams()
  const navigate = useNavigate()
  const videos = useRouteLoaderData("video-list") as VideoWithMarkers[]
  const {video, markers: videoMarkers} = videos.find(
    ({video}) => video.id.id === id
  )!
  const revalidator = useRevalidator()
  const {register, setValue, handleSubmit, control, watch} = useForm<Inputs>({})
  const [markers, setMarkers] = useImmer<Marker[]>(videoMarkers)
  const videoRef = useRef<HTMLVideoElement>(null)
  const [formMode, setFormMode] = useState<FormMode>("hidden")
  const [videoDuration, setVideoDuration] = useState<number>()

  const segments = getSegments(videoDuration, markers)
  const markerStart = watch("start")
  const markerEnd = watch("end")

  console.log(markers)

  const onSubmit = async (values: Inputs) => {
    const index =
      formMode === "create"
        ? markers.length + 1
        : markers.findIndex((m) => m.id.id === values.id)
    const newMarker = await persistMarker(
      video.id.id,
      values,
      videoDuration!,
      index
    )
    setMarkers((draft) => {
      if (formMode === "create") {
        draft.push(newMarker)
      } else if (formMode === "edit") {
        const idx = draft.findIndex((m) => m.id === newMarker.id)
        draft[idx] = newMarker
      }
    })
    setFormMode("hidden")
  }

  const onShowForm = (marker?: Marker) => {
    setFormMode(marker ? "edit" : "create")
    const start = videoRef.current?.currentTime || 0
    setValue("start", marker?.start || start)
    setValue("end", marker?.end || start + 15)
    setValue("title", marker?.primaryTag || "")
  }

  const onSetCurrentTime = (field: "start" | "end") => {
    setValue(field, videoRef.current?.currentTime || 0)
  }

  const onRemoveMarker = () => {
    // TODO
  }

  const onDone = () => {
    onClose()
  }

  const onMetadataLoaded: React.ReactEventHandler<HTMLVideoElement> = (e) => {
    const duration = e.currentTarget.duration
    setVideoDuration(duration)
  }

  const setVideoPosition = (position: number) => {
    if (videoRef.current) {
      videoRef.current.currentTime = position
    }
  }

  const onClose = () => {
    revalidator.revalidate()
    navigate("/local/videos")
  }

  return (
    <Modal isOpen onClose={onClose}>
      <div className="flex gap-2">
        <video
          className="w-2/3 max-h-[90vh]"
          muted
          controls
          src={`/api/local/video/${video.id.id}`}
          ref={videoRef}
          onLoadedMetadata={onMetadataLoaded}
        />
        <div className="flex flex-col w-1/3 justify-between">
          {formMode !== "hidden" && (
            <form
              className="w-full flex flex-col gap-2"
              onSubmit={handleSubmit(onSubmit)}
            >
              <h2 className="text-xl font-bold">
                {formMode === "create" ? "Add new" : "Edit"} marker
              </h2>
              <div className="btn-group">
                <button
                  type="button"
                  onClick={() => setVideoPosition(markerStart)}
                  className="btn"
                >
                  Go to start
                </button>
                <button
                  type="button"
                  onClick={() => setVideoPosition(markerEnd)}
                  className="btn"
                >
                  Go to end
                </button>
              </div>
              <div className="form-control">
                <label className="label">
                  <span className="label-text">Marker title</span>
                </label>
                <input
                  type="text"
                  placeholder="Type here..."
                  className="input input-bordered"
                  {...register("title", {required: true})}
                />
              </div>
              <div className="form-control">
                <label className="label">
                  <span className="label-text">Start time</span>
                </label>
                <div className="input-group w-full">
                  <Controller
                    control={control}
                    name="start"
                    render={({field}) => {
                      return (
                        <input
                          type="text"
                          className="input grow input-bordered"
                          {...field}
                          required
                          value={formatSeconds(field.value)}
                          onChange={(e) => parseSeconds(e.target.value)}
                        />
                      )
                    }}
                  />

                  <button
                    onClick={() => onSetCurrentTime("start")}
                    className="btn"
                    type="button"
                  >
                    <HiClock className="mr-2" />
                    Set current time
                  </button>
                </div>
              </div>

              <div className="form-control">
                <label className="label">
                  <span className="label-text">End time</span>
                </label>
                <div className="input-group w-full">
                  <Controller
                    control={control}
                    name="end"
                    render={({field}) => {
                      return (
                        <input
                          type="text"
                          className="input grow input-bordered"
                          {...field}
                          required
                          value={formatSeconds(field.value)}
                          onChange={(e) => parseSeconds(e.target.value)}
                        />
                      )
                    }}
                  />

                  <button
                    onClick={() => onSetCurrentTime("end")}
                    className="btn"
                    type="button"
                  >
                    <HiClock className="mr-2" />
                    Set current time
                  </button>
                </div>
              </div>

              <div className="flex justify-between">
                {formMode === "edit" ? (
                  <button
                    onClick={onRemoveMarker}
                    className="btn btn-error"
                    type="button"
                  >
                    <HiTrash className="mr-2" />
                    Remove marker
                  </button>
                ) : (
                  <div />
                )}
                <div className="btn-group mt-4">
                  <button
                    onClick={() => setFormMode("hidden")}
                    className="btn btn-secondary"
                    type="button"
                  >
                    <HiXMark className="mr-2" />
                    Cancel
                  </button>
                  <button className="btn btn-primary" type="submit">
                    <HiPlus className="mr-2" />{" "}
                    {formMode === "create" ? "Add" : "Save"}
                  </button>
                </div>
              </div>
            </form>
          )}

          {formMode === "hidden" && (
            <div>
              <h2 className="text-xl font-bold mb-2">Markers</h2>
              <div className="overflow-x-auto">
                <table className="table table-compact w-full">
                  <thead>
                    <tr>
                      <th></th>
                      <th>Tag</th>
                      <th>Start</th>
                      <th>End</th>
                      <th>Edit</th>
                    </tr>
                  </thead>
                  <tbody>
                    {markers.map((marker, idx) => (
                      <tr key={marker.id.id}>
                        <td>{idx + 1}</td>
                        <td className="font-bold">{marker.primaryTag}</td>
                        <td>{formatSeconds(marker.start)}</td>
                        <td>{formatSeconds(marker.end)}</td>
                        <td className="">
                          <button
                            onClick={() => onShowForm(marker)}
                            type="button"
                            className="btn btn-sm btn-square btn-primary"
                          >
                            <HiPencilSquare className="inline" />
                          </button>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          )}
          <div className="w-full flex justify-between">
            {formMode === "hidden" ? (
              <button
                onClick={() => onShowForm()}
                className="btn btn-primary self-center"
              >
                <HiTag className="w-4 h-4 mr-2" />
                Add new marker
              </button>
            ) : (
              <span />
            )}

            <button onClick={onDone} className="btn">
              <HiCheck className="mr-2" />
              Close
            </button>
          </div>
        </div>
      </div>
      <div className="w-full h-8 flex mt-2 gap-0.5 bg-gray-100 relative">
        {segments.map(({width, offset}, index) => {
          const marker = markers[index]
          return (
            <div
              key={index}
              className={clsx(
                "absolute h-full tooltip transition-opacity flex items-center justify-center cursor-pointer",
                getSegmentColor(index)
              )}
              onClick={() => onShowForm(marker)}
              style={{
                width: `${width}%`,
                left: `${offset}%`,
              }}
            >
              <span className="truncate">{marker.primaryTag}</span>
            </div>
          )
        })}
      </div>
    </Modal>
  )
}
