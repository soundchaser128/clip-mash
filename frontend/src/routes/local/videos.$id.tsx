import {VideoWithMarkers} from "../../types/types"
import {useRef, useState} from "react"
import {useForm, FieldErrors} from "react-hook-form"
import {
  HiClock,
  HiTrash,
  HiXMark,
  HiPlus,
  HiTag,
  HiCheck,
  HiPencilSquare,
  HiChevronLeft,
  HiChevronRight,
} from "react-icons/hi2"
import {useImmer} from "use-immer"
import {formatSeconds, parseTimestamp} from "../../helpers"
import Modal from "../../components/Modal"
import {useLoaderData, useNavigate, useRevalidator} from "react-router-dom"
import TimestampInput from "../../components/TimestampInput"
import {createNewMarker, updateMarker} from "./api"
import {SegmentedBar} from "../../components/SegmentedBar"
import Loader from "../../components/Loader"
import {MarkerDto} from "../../types/types.generated"

interface Inputs {
  id?: number
  title: string
  start: string
  end?: string
}

type FormMode = "hidden" | "create" | "edit"

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

  const markerStart = watch("start")
  const markerEnd = watch("end")

  const onDetectMarkers = async () => {
    setLoading(true)
    const result = await fetch(`/api/local/video/${video.id.id}/markers`, {
      method: "POST",
    })
    if (result.ok) {
      const data = (await result.json()) as MarkerDto[]
      setMarkers(markers.concat(data))
    }

    setLoading(false)
  }

  const onSubmit = async (values: Inputs) => {
    const index =
      formMode === "create"
        ? markers.length
        : markers.findIndex((m) => m.id.id === editedMarker?.id.id)

    if (index === -1) {
      throw new Error("could not find edited marker's ID in marker array")
    }

    const result =
      formMode === "create"
        ? await createNewMarker(video, values, videoDuration!, index)
        : await updateMarker(editedMarker!.id.id, values)

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
      const idx = draft.findIndex((m) => m.id.id === toRemove.id)
      draft.splice(idx, 1)
    })
    await fetch(`/api/local/video/marker/${toRemove.id}`, {method: "DELETE"})
    setFormMode("hidden")
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
    navigate(-1)
  }

  return (
    <Modal isOpen onClose={onClose}>
      <div className="flex gap-2">
        <video
          className="w-2/3 max-h-[90vh]"
          muted
          controls
          src={`/api/local/video/${video.id.id}/file`}
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
              <div className="flex w-full items-baseline justify-between">
                <button
                  type="button"
                  onClick={() => setVideoPosition(parseTimestamp(markerStart))}
                  className="btn btn-secondary"
                >
                  <HiChevronLeft className="mr-2" />
                  Go to start
                </button>
                Navigate
                <button
                  type="button"
                  onClick={() =>
                    typeof markerEnd !== "undefined" &&
                    setVideoPosition(parseTimestamp(markerEnd))
                  }
                  className="btn btn-secondary"
                  disabled={typeof markerEnd === "undefined"}
                >
                  Go to end
                  <HiChevronRight className="ml-2" />
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
              <h2 className="text-xl font-bold mb-2">Markers</h2>
              <div className="overflow-x-auto">
                {markers.length === 0 && !loading && (
                  <div className="flex flex-col gap-2 h-full w-full justify-center items-center">
                    <span className="text-lg">No markers yet.</span>
                    <p className="text-sm">
                      You can try letting ClipMash detect markers for you or add
                      them manually.
                    </p>
                    <button
                      onClick={onDetectMarkers}
                      className="btn btn-secondary"
                    >
                      <HiPlus className="mr-2" />
                      Detect markers
                    </button>
                  </div>
                )}
                {loading && (
                  <Loader className="h-full">Detecting markers...</Loader>
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
                        <tr key={marker.id.id}>
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
      <SegmentedBar
        length={video.duration}
        items={markers.map((marker) => ({
          label: marker.primaryTag,
          length: marker.end - marker.start,
          offset: marker.start,
        }))}
        onItemClick={(item, index) => onShowForm(markers[index])}
        selectedIndex={editedMarker ? markers.indexOf(editedMarker) : undefined}
      />
    </Modal>
  )
}
