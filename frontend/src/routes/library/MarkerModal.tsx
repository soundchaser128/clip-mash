import React, {useEffect, useRef, useState} from "react"
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
  HiQuestionMarkCircle,
  HiMagnifyingGlass,
  HiChevronLeft,
  HiPause,
  HiSpeakerWave,
  HiSpeakerXMark,
} from "react-icons/hi2"
import {useImmer} from "use-immer"
import {formatSeconds, isBetween} from "@/helpers"
import Modal from "@/components/Modal"
import {useLoaderData, useNavigate, useRevalidator} from "react-router-dom"
import TimestampInput from "@/components/TimestampInput"
import {createMarker, updateMarker} from "./api"
import Timeline from "@/components/Timeline"
import Loader from "@/components/Loader"
import {
  StashConfig,
  MarkerDto,
  VideoDto,
  deleteMarker,
  splitMarker,
  VideoDetailsDto,
  detectMarkers,
  getVideo,
} from "@/api"
import {useConfig} from "@/hooks/useConfig"
import Kbd from "@/components/Kbd"
import useHotkeys from "@/hooks/useHotkeys"
import clsx from "clsx"

function getVideoUrl(video: VideoDto, config?: StashConfig): string {
  if (video?.source === "Stash" && config) {
    return `${config.stashUrl}/scene/${video.stashSceneId!}/stream?apikey=${
      config.apiKey
    }`
  } else {
    return `/api/library/video/${video?.id}/file`
  }
}

interface Inputs {
  id?: number
  title: string
  start: string
  end?: string
  createInStash?: boolean
}

type FormMode = "hidden" | "create" | "edit" | "help"

function HelpPanel({onBack}: {onBack: () => void}) {
  return (
    <div className="flex flex-col h-full text-sm">
      <p className="text-sm link mb-2" onClick={onBack}>
        <HiChevronLeft className="mr-1 inline" />
        Back
      </p>
      <h2 className="text-xl font-bold mb-2">Information</h2>
      <p className="mb-2">
        This panel allows you to create, update and delete markers. Markers are
        labelled sections of a video that will be used later on to create clips.
        Only parts of the video that are part of a marker will be used for the
        compilation.
      </p>
      <p className="mb-2">
        You can create markers manually, or use the &quot;Detect&quot; button to
        automatically create markers based on cuts in the video. You can also
        split markers into two parts, or add the entire video as a marker.
      </p>

      <p className="mb-4">
        Mark points can be used to mark interesting points (cuts, scene
        transitions) in a video and you can turn them into markers bu clicking
        on the &quot;From points&quot; button.
      </p>

      <h2 className="text-xl font-bold mb-2">Keyboard reference</h2>
      <ul className="flex flex-col gap-2">
        <li>
          <Kbd keys="I" /> Add mark point
        </li>
        <li>
          <Kbd keys="M F" separator=" " /> Add entire video
        </li>
        <li>
          <Kbd keys="M N" separator=" " /> Open marker creation form
        </li>
        <li>
          <Kbd keys="M S" separator=" " /> Split marker at current time
        </li>
        <li>
          <Kbd keys="M I" separator=" " /> Turn mark points into markers
        </li>
        <li>
          <Kbd keys="Space" /> Play/pause video
        </li>
        <li>
          <Kbd keys="V M" separator=" " /> Toggle mute
        </li>
      </ul>
    </div>
  )
}

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

const useVideoDetails = (id: string | undefined) => {
  const [video, setVideo] = useState<VideoDetailsDto>()
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<Error>()

  useEffect(() => {
    if (!id) {
      return
    }
    setLoading(true)
    getVideo(id)
      .then((data) => setVideo(data))
      .catch((error) => setError(error as Error))
      .finally(() => setLoading(false))
  }, [id])

  return {
    video,
    loading,
    error,
  }
}

interface Props {
  videoId?: string
  onClose: () => void
  isOpen: boolean
}

export default function EditVideoModal({videoId, onClose, isOpen}: Props) {
  const {
    register,
    handleSubmit,
    control,
    formState: {errors},
    setError,
    setValue,
  } = useForm<Inputs>({
    resolver: handleValidation,
  })

  const {video: data, loading: videoLoading, error} = useVideoDetails(videoId)
  const {video} = data || {}

  const revalidator = useRevalidator()

  const [markers, setMarkers] = useImmer<MarkerDto[]>(data?.markers || [])
  const videoRef = useRef<HTMLVideoElement>(null)
  const [formMode, setFormMode] = useState<FormMode>("hidden")
  const [videoDuration, setVideoDuration] = useState<number>()
  const [editedMarker, setEditedMarker] = useState<MarkerDto>()
  const [loading, setLoading] = useState(false)
  const threshold = 40
  const [time, setTime] = useState(0)
  const [markPoints, setMarkPoints] = useImmer<number[]>([])
  const config = useConfig()
  const showingForm = formMode === "create" || formMode === "edit"
  const isPlaying = videoRef.current?.paused === false
  const [isMuted, setIsMuted] = useState(videoRef.current?.muted)

  const onAddMark = () => {
    setMarkPoints((draft) => {
      draft.push(videoRef.current!.currentTime)
    })
  }

  const onAddFullVideo = async () => {
    if (markers.length > 0) {
      return
    }

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
      false,
    )

    if (result.isOk) {
      const marker = result.unwrap()
      setMarkers([marker])
      revalidator.revalidate()
    } else {
      const error = result.error
      console.error(error)
    }
  }

  const onSplitMarker = async () => {
    const currentTime = videoRef.current?.currentTime || 0
    const currentMarker = markers.find((m) =>
      isBetween(currentTime, m.start, m.end),
    )
    if (currentMarker) {
      const data = await splitMarker(currentMarker.id, {time: currentTime})
      setMarkers(data)
      revalidator.revalidate()
    }
  }

  const onConsumeMarkPoints = async () => {
    if (markPoints.length === 0) {
      return
    }

    const newMarkers: MarkerDto[] = []
    const points = [...markPoints]
    if (points[0] != 0.0) {
      points.unshift(0.0)
    }

    for (let i = 0; i < points.length; i++) {
      const current = points[i]
      const next = points[i + 1] || videoDuration!
      const marker = await createMarker(
        video,
        {
          start: current,
          end: next,
          title: "Untitled",
        },
        videoDuration!,
        i,
        false,
      )
      newMarkers.push(marker.unwrap())
    }
    // turn mark point timestamps into markers
    setMarkers((draft) => {
      draft.push(...newMarkers)
    })
    setMarkPoints([])
  }

  const onTogglePlay = () => {
    if (videoRef.current) {
      if (videoRef.current.paused) {
        videoRef.current.play()
      } else {
        videoRef.current.pause()
      }
    }
  }

  const onToggleMuted = () => {
    if (videoRef.current) {
      videoRef.current.muted = !videoRef.current.muted
      setIsMuted(videoRef.current.muted)
    }
  }

  useHotkeys("i", onAddMark)
  useHotkeys("m f", onAddFullVideo)
  useHotkeys("m n", () => onShowForm("create", undefined))
  useHotkeys("m s", onSplitMarker)
  useHotkeys("m i", onConsumeMarkPoints)
  useHotkeys("space", onTogglePlay)
  useHotkeys("v m", onToggleMuted)

  useEffect(() => {
    if (videoRef.current) {
      videoRef.current.focus()
    }
  }, [videoRef])

  if (!data || videoLoading) {
    return null
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
        ? await createMarker(
            video,
            values,
            videoDuration!,
            index,
            values.createInStash ?? false,
          )
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
      revalidator.revalidate()
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

  const onSetCurrentTime = (field: "start" | "end") => {
    setValue(
      field,
      formatSeconds(videoRef.current?.currentTime || 0, "short-with-ms"),
    )
  }
  const currentItemIndex = markers.findIndex((m) =>
    isBetween(time, m.start, m.end || videoDuration!),
  )

  const onTimeUpdate: React.ReactEventHandler<HTMLVideoElement> = (e) => {
    setTime(e.currentTarget.currentTime)
  }

  const onRemoveMark = (t: number, e: React.MouseEvent) => {
    e.stopPropagation()
    setMarkPoints((draft) => {
      const idx = draft.findIndex((m) => m === t)
      draft.splice(idx, 1)
    })
  }

  const onShowForm = (mode: FormMode, marker?: MarkerDto) => {
    setFormMode(mode)
    const start = mode === "create" ? videoRef.current?.currentTime : undefined
    setValue("start", formatSeconds(marker?.start || start, "short-with-ms"))
    setValue("end", formatSeconds(marker?.end || undefined, "short-with-ms"))
    setValue("title", marker?.primaryTag || "")

    if (marker) {
      setEditedMarker(marker)
    }
  }

  const onDetectMarkers = async () => {
    setLoading(true)

    const data = await detectMarkers(video.id, {
      threshold: threshold / 100,
    })
    setMarkers(markers.concat(data))
    setLoading(false)
    revalidator.revalidate()
  }

  const onRemoveMarker = async () => {
    const toRemove = editedMarker!.id
    setMarkers((draft) => {
      const idx = draft.findIndex((m) => m.id === toRemove)
      draft.splice(idx, 1)
    })
    await deleteMarker(toRemove)
    setFormMode("hidden")
    revalidator.revalidate()
  }

  const onDone = () => {
    onClose()
  }

  const onMetadataLoaded: React.ReactEventHandler<HTMLVideoElement> = (e) => {
    const duration = e.currentTarget.duration
    setVideoDuration(duration)
  }

  const onItemClick = (item: unknown, index: number) => {
    const marker = markers[index]
    onShowForm("edit", marker)
    if (videoRef.current) {
      videoRef.current.currentTime = marker.start
    }
  }

  const onTimelineClick = (time: number) => {
    if (videoRef.current) {
      videoRef.current.currentTime = time
    }
  }

  const onDeleteAll = async () => {
    if (
      confirm(
        "This will delete all markers for the current video. Are you sure?",
      )
    ) {
      for (const marker of markers) {
        await deleteMarker(marker.id)
      }
      setMarkers([])
      revalidator.revalidate()
    }
  }

  return (
    <Modal isOpen={isOpen} onClose={onClose}>
      <div className="flex gap-2">
        <video
          className="w-2/3 max-h-[82vh]"
          src={getVideoUrl(video, config)}
          ref={videoRef}
          onLoadedMetadata={onMetadataLoaded}
          onTimeUpdate={onTimeUpdate}
          muted
        />
        <div className="flex flex-col w-1/3 justify-between">
          {showingForm && (
            <form
              className="w-full flex flex-col gap-2"
              onSubmit={handleSubmit(onSubmit)}
            >
              <h2 className="text-xl font-bold">
                {formMode === "create" ? "Add new" : "Edit"} marker
              </h2>
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
                  className="input input-bordered w-full"
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
              {video?.source === "Stash" && (
                <div className="form-control">
                  <label className="label cursor-pointer">
                    <span className="label-text">
                      Create marker in Stash as well?
                    </span>

                    <input
                      type="checkbox"
                      className="checkbox checkbox-primary"
                      {...register("createInStash")}
                    />
                  </label>
                </div>
              )}

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
                    className="btn btn-outline"
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

          {!showingForm && (
            <div>
              <div className="overflow-x-auto p-2">
                {formMode === "help" && (
                  <HelpPanel onBack={() => setFormMode("hidden")} />
                )}
                {loading && (
                  <Loader className="h-full w-full justify-center">
                    Detecting markers...
                  </Loader>
                )}
                {formMode === "hidden" && !loading && (
                  <>
                    <div className="flex w-full justify-between">
                      <div className="grid grid-cols-2 gap-0.5">
                        <button
                          onClick={() => onShowForm("create")}
                          className="btn btn-sm btn-success"
                        >
                          <HiPlus className="w-4 h-4 mr-2" />
                          Add
                        </button>
                        <button
                          disabled={markers.length === 0}
                          onClick={onSplitMarker}
                          className="btn btn-sm btn-primary"
                        >
                          <HiTag className="w-4 h-4 mr-2" />
                          Split
                        </button>
                        <button
                          onClick={onDetectMarkers}
                          className="btn btn-sm btn-primary"
                          disabled={markers.length > 0}
                        >
                          <HiMagnifyingGlass className="w-4 h-4 mr-2" />
                          Detect
                        </button>
                        <button
                          onClick={onConsumeMarkPoints}
                          className="btn btn-sm btn-success"
                          type="button"
                          disabled={markPoints.length === 0}
                        >
                          <HiPlus /> From points
                        </button>
                      </div>

                      <button
                        onClick={() => setFormMode("help")}
                        className="btn btn-sm btn-secondary"
                      >
                        <HiQuestionMarkCircle className="w-4 h-4 mr-2" />
                        Help
                      </button>
                    </div>
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
                        {markers.length === 0 && (
                          <tr>
                            <td colSpan={5} className="text-center">
                              No markers yet. Click the Help button for
                              information on how to add markers.
                            </td>
                          </tr>
                        )}

                        {markers.map((marker, idx) => (
                          <tr key={marker.id}>
                            <td>{idx + 1}</td>
                            <td className="font-bold">{marker.primaryTag}</td>
                            <td>
                              {formatSeconds(marker.start, "short-with-ms")}
                            </td>
                            <td>
                              {formatSeconds(marker.end, "short-with-ms")}
                            </td>
                            <td>
                              <button
                                onClick={() => onShowForm("edit", marker)}
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
                  </>
                )}
              </div>
            </div>
          )}
          <div className="w-full flex justify-between">
            {formMode === "hidden" && (
              <>
                <button onClick={onDeleteAll} className="btn btn-error">
                  <HiTrash className="mr-2" />
                  Delete all
                </button>
                <button onClick={onDone} className="btn btn-success">
                  <HiCheck className="mr-2" />
                  Done
                </button>
              </>
            )}
          </div>
        </div>
      </div>
      <div className="flex gap-2 items-center w-full">
        <button
          onClick={onTogglePlay}
          className={clsx("btn btn-square", {
            "btn-success": !isPlaying,
            "btn-neutral": isPlaying,
          })}
          type="button"
        >
          {isPlaying ? (
            <HiPause className="w-5 h-5" />
          ) : (
            <HiPlay className="w-5 h-5" />
          )}
        </button>
        <button
          onClick={onToggleMuted}
          className="btn btn-square"
          type="button"
        >
          {isMuted ? (
            <HiSpeakerWave className="w-5 h-5" />
          ) : (
            <HiSpeakerXMark className="w-5 h-5" />
          )}
        </button>
        <Timeline
          length={video?.duration}
          items={markers.map((marker) => ({
            label: marker.primaryTag,
            length: marker.end - marker.start,
            offset: marker.start,
          }))}
          onItemClick={(item, index) => onItemClick(item, index)}
          selectedIndex={
            editedMarker ? markers.indexOf(editedMarker) : currentItemIndex
          }
          fadeInactiveItems
          time={time}
          markPoints={markPoints}
          onMarkerClick={onRemoveMark}
          onTimelineClick={onTimelineClick}
          className="py-4 flex-grow"
        />
      </div>
    </Modal>
  )
}
