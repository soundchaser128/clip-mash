import {VideoWithMarkers} from "../../types/types"
import React, {useRef, useState} from "react"
import {useForm, FieldErrors} from "react-hook-form"
import {
  HiClock,
  HiTrash,
  HiXMark,
  HiPlus,
  HiTag,
  HiCheck,
  HiPencilSquare,
  HiPlay,
  HiSquaresPlus,
} from "react-icons/hi2"
import {useImmer} from "use-immer"
import {formatSeconds, isBetween, parseTimestamp} from "../../helpers"
import Modal from "../../components/Modal"
import {useLoaderData, useNavigate, useRevalidator} from "react-router-dom"
import TimestampInput from "../../components/TimestampInput"
import {createMarker, updateMarker} from "./api"
import Timeline from "../../components/Timeline"
import Loader from "../../components/Loader"
import {
  StashConfig,
  MarkerDto,
  VideoDto,
  deleteMarker,
  splitMarker,
} from "../../api"
import {detectMarkers} from "../../api"
import {useConfig} from "../../hooks/useConfig"

function getVideoUrl(video: VideoDto, config?: StashConfig): string {
  if (video.source === "Stash" && config) {
    return `${config.stashUrl}/scene/${video.stashSceneId!}/stream?apikey=${
      config.apiKey
    }`
  } else {
    return `/api/library/video/${video.id}/file`
  }
}

const Box: React.FC<{children: React.ReactNode}> = ({children}) => (
  <div className="flex flex-col bg-base-200 py-4 px-6 rounded-lg w-2/3">
    {children}
  </div>
)

interface Inputs {
  id?: number
  title: string
  start: string
  end?: string
}

type FormMode = "hidden" | "create" | "edit"

function CreateMarkerButtons({
  onDetectMarkers,
  onAddFullVideo,
  threshold,
  setThreshold,
}: {
  onDetectMarkers: () => void
  onAddFullVideo: () => void
  threshold: number
  setThreshold: (value: number) => void
}) {
  return (
    <div className="flex flex-col h-full gap-6 items-center">
      <Box>
        <p className="">
          Detect markers by detecting scene changes (cuts in the video). Might
          not be fully accurate. It does not work very well for PoV videos.
        </p>
        <div className="form-control">
          <label className="label">
            <span className="label-text">
              Marker detection threshold (lower means more markers)
            </span>
          </label>
          <input
            type="range"
            min="0"
            max="100"
            className="range range-sm w-full"
            step="5"
            value={threshold}
            onChange={(e) => setThreshold(e.target.valueAsNumber)}
          />
          <div className="w-full flex justify-between text-xs px-2 mb-4">
            <span>0</span>
            <span className="font-bold">{Math.round(threshold)}</span>
            <span>100</span>
          </div>
        </div>
        <button onClick={onDetectMarkers} className="btn btn-secondary">
          <HiSquaresPlus className="mr-2" />
          Detect markers
        </button>
      </Box>
      <Box>
        <p className="mb-2">Add a single marker that spans the entire video.</p>
        <button className="btn btn-secondary" onClick={onAddFullVideo}>
          <HiPlus className="mr-2" />
          Add entire video
        </button>
      </Box>
    </div>
  )
}

export default function EditVideoModal() {
  const navigate = useNavigate()
  const {video, markers: videoMarkers} = useLoaderData() as VideoWithMarkers

  const revalidator = useRevalidator()
  const handleValidation = (values: Inputs) => {
    const {start, end, title} = values
    const errors: FieldErrors<Inputs> = {}
    if ((end || 0) <= start) {
      errors.end = {
        type: "required",
        message: "End must be after start",
      }
    }
    if (!title || !title.trim()) {
      errors.title = {
        type: "required",
        message: "Must enter a title",
      }
    }

    return {
      values,
      errors,
    }
  }

  const {
    register,
    setValue,
    handleSubmit,
    control,
    watch,
    setError,
    formState: {errors},
  } = useForm<Inputs>({
    resolver: handleValidation,
  })
  const [markers, setMarkers] = useImmer<MarkerDto[]>(videoMarkers)
  const videoRef = useRef<HTMLVideoElement>(null)
  const [formMode, setFormMode] = useState<FormMode>("hidden")
  const [videoDuration, setVideoDuration] = useState<number>()
  const [editedMarker, setEditedMarker] = useState<MarkerDto>()
  const [loading, setLoading] = useState(false)
  const [threshold, setThreshold] = useState(40)
  const [time, setTime] = useState(0)
  const config = useConfig()

  const markerStart = watch("start")

  const currentItemIndex = markers.findIndex((m) =>
    isBetween(time, m.start, m.end || videoDuration!),
  )

  const onTimeUpdate: React.ReactEventHandler<HTMLVideoElement> = (e) => {
    setTime(e.currentTarget.currentTime)
  }

  const onDetectMarkers = async () => {
    setLoading(true)

    const data = await detectMarkers(video.id, {
      threshold: threshold / 100,
    })
    setMarkers(markers.concat(data))
    setLoading(false)
  }

  const onSubmit = async (values: Inputs) => {
    const index =
      formMode === "create"
        ? markers.length
        : markers.findIndex((m) => m.id === editedMarker?.id)

    if (index === -1) {
      throw new Error("could not find edited marker's ID in marker array")
    }

    const result =
      formMode === "create"
        ? await createMarker(video, values, videoDuration!, index)
        : await updateMarker(editedMarker!.id, values)

    if (result.isOk) {
      const marker = result.unwrap()
      setMarkers((draft) => {
        if (formMode === "create") {
          draft.push(marker)
        } else if (formMode === "edit") {
          const idx = draft.findIndex((m) => m.id === marker.id)
          draft[idx] = marker
        }
      })
      setFormMode("hidden")
    } else {
      const err = result.error
      if (typeof err.error === "object") {
        for (const key in err.error) {
          setError(key as keyof Inputs, {
            message: err.error[key],
          })
        }
      }
    }
  }

  const onShowForm = (marker?: MarkerDto) => {
    setFormMode(marker ? "edit" : "create")
    const start = videoRef.current?.currentTime || 0
    setValue("start", formatSeconds(marker?.start || start, "short"))
    setValue("end", formatSeconds(marker?.end || undefined, "short"))
    setValue("title", marker?.primaryTag || "")

    if (marker) {
      setEditedMarker(marker)
    }
  }

  const onSetCurrentTime = (field: "start" | "end") => {
    setValue(field, formatSeconds(videoRef.current?.currentTime || 0, "short"))
  }

  const onRemoveMarker = async () => {
    const toRemove = editedMarker!.id
    setMarkers((draft) => {
      const idx = draft.findIndex((m) => m.id === toRemove)
      draft.splice(idx, 1)
    })
    await deleteMarker(toRemove)
    setFormMode("hidden")
  }

  const onDone = () => {
    onClose()
  }

  const onMetadataLoaded: React.ReactEventHandler<HTMLVideoElement> = (e) => {
    const duration = e.currentTarget.duration
    setVideoDuration(duration)
  }

  const onPlayMarker = (position: number) => {
    if (videoRef.current) {
      videoRef.current.currentTime = position
      videoRef.current.play()
    }
  }

  const onClose = () => {
    revalidator.revalidate()
    navigate(-1)
  }

  const onSplitMarker = async () => {
    const currentTime = videoRef.current?.currentTime || 0
    const currentMarker = markers.find((m) =>
      isBetween(currentTime, m.start, m.end),
    )
    if (currentMarker) {
      const data = await splitMarker(currentMarker.id, {time: currentTime})
      setMarkers(data)
    }
  }

  const onAddFullVideo = async () => {
    const duration = video.duration
    const result = await createMarker(
      video,
      {
        start: 0.0,
        end: duration,
        title: "Untitled",
      },
      duration,
      0,
    )

    if (result.isOk) {
      const marker = result.unwrap()
      setMarkers([marker])
    } else {
      const error = result.error
      console.error(error)
    }
  }

  return (
    <Modal isOpen onClose={onClose}>
      <div className="flex gap-2">
        <video
          className="w-2/3 max-h-[90vh]"
          muted
          controls
          src={getVideoUrl(video, config)}
          ref={videoRef}
          onLoadedMetadata={onMetadataLoaded}
          onTimeUpdate={onTimeUpdate}
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
              <div className="flex w-full items-baseline justify-between">
                <button
                  type="button"
                  className="btn btn-success"
                  onClick={() => onPlayMarker(parseTimestamp(markerStart))}
                >
                  <HiPlay className="mr-2" />
                  Play
                </button>
              </div>
              <div className="form-control">
                <label className="label">
                  <span className="label-text">Marker title</span>
                  <span className="label-text-alt text-error">
                    {errors.title?.message}
                  </span>
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
                  <TimestampInput
                    name="start"
                    control={control}
                    error={errors.start}
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
                  <span className="label-text-alt text-error">
                    {errors.end?.message}
                  </span>
                </label>
                <div className="input-group w-full">
                  <TimestampInput
                    name="end"
                    control={control}
                    error={errors.end}
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

              <div className="flex justify-between mt-2">
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
                <div className="btn-group">
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
              <h2 className="text-3xl font-bold mb-4">Markers</h2>
              <div className="overflow-x-auto">
                {markers.length === 0 && !loading && (
                  <CreateMarkerButtons
                    onDetectMarkers={onDetectMarkers}
                    onAddFullVideo={onAddFullVideo}
                    threshold={threshold}
                    setThreshold={setThreshold}
                  />
                )}
                {loading && (
                  <Loader className="h-full w-full justify-center">
                    Detecting markers...
                  </Loader>
                )}
                {markers.length > 0 && (
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
                        <tr key={marker.id}>
                          <td>{idx + 1}</td>
                          <td className="font-bold">{marker.primaryTag}</td>
                          <td>{formatSeconds(marker.start, "short")}</td>
                          <td>{formatSeconds(marker.end, "short")}</td>
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
                )}
              </div>
            </div>
          )}
          <div className="w-full flex justify-between">
            {formMode === "hidden" ? (
              <div className="flex gap-2">
                <button
                  onClick={() => onShowForm()}
                  className="btn btn-primary"
                >
                  <HiTag className="w-4 h-4 mr-2" />
                  Add new marker
                </button>
                <button
                  disabled={markers.length === 0}
                  onClick={onSplitMarker}
                  className="btn btn-secondary"
                >
                  <HiTag className="w-4 h-4 mr-2" />
                  Split marker
                </button>
              </div>
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
      <Timeline
        length={video.duration}
        items={markers.map((marker) => ({
          label: marker.primaryTag,
          length: marker.end - marker.start,
          offset: marker.start,
        }))}
        onItemClick={(item, index) => onShowForm(markers[index])}
        selectedIndex={
          editedMarker ? markers.indexOf(editedMarker) : currentItemIndex
        }
        fadeInactiveItems
        time={time}
      />
    </Modal>
  )
}
