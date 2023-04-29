import {useStateMachine} from "little-state-machine"
import invariant from "tiny-invariant"
import {LocalVideoDto, StateHelpers} from "../../types/types"
import {
  HiAdjustmentsVertical,
  HiCheck,
  HiClock,
  HiPlus,
  HiTag,
  HiXMark,
} from "react-icons/hi2"
import Modal from "../../components/Modal"
import React, {useMemo, useRef, useState} from "react"
import {useForm, Controller} from "react-hook-form"
import {useImmer} from "use-immer"
import {format} from "date-fns"
import {parse} from "date-fns"
import {getMilliseconds} from "date-fns"
import {updateForm} from "../actions"
import {LoaderFunction, json, useLoaderData} from "react-router-dom"
import {getFormState} from "../../helpers"

interface Marker {
  title: string
  startTime: number
  endTime: number
}

interface Inputs {
  title: string
  startTime: number
  endTime: number
}

export const loader: LoaderFunction = async () => {
  const formState = getFormState()
  invariant(StateHelpers.isLocalFiles(formState!))

  const params = new URLSearchParams({
    path: formState.localVideoPath!,
    recurse: formState.recurse ? "true" : "false",
  })

  const response = await fetch(`/api/video?${params.toString()}`, {
    method: "POST",
  })
  const data = await response.json()
  return json(data)
}

function formatSeconds(seconds?: number): string {
  return typeof seconds !== "undefined" ? format(seconds * 1000, "mm:ss") : ""
}

function parseSeconds(string: string): number {
  return getMilliseconds(parse(string, "mm:ss", new Date())) / 1000.0
}

const MarkerModalContent: React.FC<{
  video: LocalVideoDto
  onFinished: (markers: Marker[]) => void
}> = ({video, onFinished}) => {
  const {register, setValue, handleSubmit, control} = useForm<Inputs>({})
  const [markers, setMarkers] = useImmer<Marker[]>(video.markers)
  const videoRef = useRef<HTMLVideoElement>(null)
  const [formVisible, setFormVisible] = useState(false)

  const onSubmit = (values: Inputs) => {
    setMarkers((draft) => {
      draft.push(values)
    })
    setFormVisible(false)
  }

  const onShowForm = () => {
    setFormVisible(true)
    const start = videoRef.current?.currentTime || 0
    setValue("startTime", start)
    setValue("endTime", start + 15)
    setValue("title", "")
  }

  const onSetCurrentTime = (field: "startTime" | "endTime") => {
    setValue(field, videoRef.current?.currentTime || 0)
  }

  const segments = useMemo(() => {
    if (videoRef.current) {
      const totalDuration = videoRef.current.duration
      const result = []
      for (const marker of markers) {
        const offset = (marker.startTime / totalDuration) * 100
        const seconds = marker.endTime - marker.startTime
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
  }, [markers, videoRef.current?.duration])

  const onDone = () => {
    onFinished(markers)
  }

  return (
    <>
      <video
        className="w-full max-h-[800px]"
        muted
        controls
        src={`/api/video/${video.id}`}
        ref={videoRef}
      />
      <div className="w-full h-8 flex my-4 gap-0.5 bg-gray-100 relative">
        {segments.map(({width, offset}, index) => {
          const marker = markers[index]
          return (
            <div
              key={index}
              className="absolute h-full tooltip transition-opacity bg-gray-400 flex items-center justify-center"
              style={{
                width: `${width}%`,
                left: `${offset}%`,
              }}
            >
              <span className="truncate">{marker.title}</span>
            </div>
          )
        })}
      </div>

      {formVisible && (
        <form
          className="max-w-lg flex flex-col gap-2"
          onSubmit={handleSubmit(onSubmit)}
        >
          <h2 className="text-xl font-bold">Add new marker</h2>
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
                name="startTime"
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
                onClick={() => onSetCurrentTime("startTime")}
                className="btn btn-square"
                type="button"
              >
                <HiClock />
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
                name="endTime"
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
                onClick={() => onSetCurrentTime("endTime")}
                className="btn btn-square"
                type="button"
              >
                <HiClock />
              </button>
            </div>
          </div>

          <button className="btn btn-primary self-end" type="submit">
            <HiPlus className="mr-2" /> Add
          </button>
        </form>
      )}

      {!formVisible && (
        <button onClick={onShowForm} className="btn btn-primary self-start">
          <HiTag className="w-4 h-4 mr-2" />
          Add new marker
        </button>
      )}

      <button onClick={onDone} className="btn btn-success self-end mt-4">
        <HiCheck className="mr-2" />
        Done
      </button>
    </>
  )
}

async function persistMarkers(videoId: string, markers: Marker[]) {
  const payload = markers.map((m) => ({
    ...m,
    videoId,
  }))

  await fetch("/api/video/markers", {
    method: "POST",
    body: JSON.stringify(payload),
    headers: {"Content-Type": "application/json"},
  })
}

export default function ListVideos() {
  const {state, actions} = useStateMachine({updateForm})
  invariant(StateHelpers.isLocalFiles(state.data))
  const initialVideos = useLoaderData() as LocalVideoDto[]
  const [videos, setVideos] = useImmer<LocalVideoDto[]>(initialVideos || [])
  const [modalVideo, setModalVideo] = useState<LocalVideoDto>()
  const modalOpen = typeof modalVideo !== "undefined"

  const onRemoveFile = (video: LocalVideoDto) => {
    // TODO
  }

  const onMarkersAdded = async (markers: Marker[]) => {
    console.log("markers added", markers)
    const id = modalVideo?.id
    if (id) {
      await persistMarkers(id, markers)
    }

    setVideos((draft) => {
      const idx = draft.findIndex((v) => v.id === id)
      if (idx !== -1) {
        draft[idx].markers = markers
        actions.updateForm({
          videos: draft,
        })
      }
    })
    setModalVideo(undefined)
  }

  return (
    <>
      <Modal isOpen={modalOpen} onClose={() => setModalVideo(undefined)}>
        {modalVideo && (
          <MarkerModalContent video={modalVideo} onFinished={onMarkersAdded} />
        )}
      </Modal>

      <section className="grid grid-cols-1 lg:grid-cols-3 gap-2 w-full mt-4">
        {videos.map((video) => (
          <article
            className="card card-compact bg-base-100 shadow-xl"
            key={video.fileName}
          >
            <figure className="">
              <video
                className="w-full aspect-video"
                muted
                src={`/api/video/${video.id}`}
              />
            </figure>
            <div className="card-body">
              <h2 className="card-title">
                <span className="truncate">{video.fileName}</span>
              </h2>
              <ul className="flex flex-col gap-2 self-start">
                <li>
                  <HiAdjustmentsVertical className="inline mr-2" />
                  Interactive:{" "}
                  <strong>
                    {video.interactive ? (
                      <HiCheck className="text-green-600 inline" />
                    ) : (
                      <HiXMark className="text-red-600 inline" />
                    )}
                  </strong>
                </li>
                <li>
                  <HiTag className="inline mr-2" />
                  Markers: <strong>{video.markers.length}</strong>
                </li>
              </ul>
            </div>

            <div className="card-actions justify-end">
              <div className="btn-group">
                <button
                  className="btn btn-secondary btn-sm btn-outline"
                  onClick={() => setModalVideo(video)}
                >
                  <HiPlus className="w-4 h-4 mr-2" />
                  Add markers
                </button>
                <button
                  onClick={() => onRemoveFile(video)}
                  className="btn btn-error btn-sm"
                >
                  <HiXMark className="w-4 h-4 mr-2" />
                  Remove
                </button>
              </div>
            </div>
          </article>
        ))}
      </section>
    </>
  )
}
