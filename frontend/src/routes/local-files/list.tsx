import {useStateMachine} from "little-state-machine"
import invariant from "tiny-invariant"
import {LocalVideo, StateHelpers} from "../../types/types"
import {
  HiAdjustmentsVertical,
  HiCheck,
  HiClock,
  HiPlus,
  HiTag,
  HiXMark,
} from "react-icons/hi2"
import Modal from "../../components/Modal"
import React, {useRef, useState} from "react"
import {useForm, Controller} from "react-hook-form"
import {useImmer} from "use-immer"
import {format} from "date-fns"
import {parse} from "date-fns"
import {getMilliseconds} from "date-fns"

interface Marker {
  title: string
  start: number
  // end: number
}

interface Inputs {
  title: string
  start: number
}

const MarkerModalContent: React.FC<{video: LocalVideo}> = ({video}) => {
  const {register, setValue, handleSubmit, control} = useForm<Inputs>({})
  const [markers, setMarkers] = useImmer<Marker[]>([])
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
    setValue("start", videoRef.current!.currentTime)
  }

  const onSetCurrentTime = () => {
    setValue("start", videoRef.current!.currentTime)
  }

  return (
    <>
      <video
        className="w-full mb-4 max-h-[1024px]"
        muted
        controls
        src={`/api/video/${video.id}`}
        ref={videoRef}
      />
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
              {...register("title")}
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
                      value={format(field.value * 1000, "mm:ss")}
                      onChange={(e) =>
                        getMilliseconds(
                          parse(e.target.value, "mm:ss", new Date())
                        ) / 1000.0
                      }
                    />
                  )
                }}
              />

              <button onClick={onSetCurrentTime} className="btn btn-square">
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

      <button className="btn btn-success self-end mt-4">
        <HiCheck className="mr-2" />
        Done
      </button>
    </>
  )
}

export default function ListVideos() {
  const {state} = useStateMachine()
  invariant(StateHelpers.isLocalFiles(state.data))
  const videos = state.data.videos!
  const [modalVideo, setModalVideo] = useState<LocalVideo>()
  const modalOpen = typeof modalVideo !== "undefined"

  const onRemoveFile = (video: LocalVideo) => {
    // TODO
  }

  return (
    <>
      <Modal isOpen={modalOpen} onClose={() => setModalVideo(undefined)}>
        {modalVideo && <MarkerModalContent video={modalVideo} />}
      </Modal>

      <section className="grid grid-cols-1 lg:grid-cols-3 gap-2 w-full mt-4">
        {videos.map((file) => (
          <article
            className="card card-compact bg-base-100 shadow-xl"
            key={file.fileName}
          >
            <figure className="">
              <video className="w-full" muted src={`/api/video/${file.id}`} />
            </figure>
            <div className="card-body">
              <h2 className="card-title">
                <span className="truncate">{file.fileName}</span>
              </h2>
              <ul>
                <li
                  className="tooltip"
                  data-tip="Whether the scene has an associated .funscript file."
                >
                  <HiAdjustmentsVertical className="inline mr-2" />
                  Interactive:{" "}
                  <strong>
                    {file.interactive ? (
                      <HiCheck className="text-green-600 inline" />
                    ) : (
                      <HiXMark className="text-red-600 inline" />
                    )}
                  </strong>
                </li>
              </ul>
            </div>

            <div className="card-actions justify-end">
              <button
                className="btn btn-secondary btn-sm"
                onClick={() => setModalVideo(file)}
              >
                <HiPlus className="w-4 h-4 mr-2" />
                Add markers
              </button>
              <button
                onClick={() => onRemoveFile(file)}
                className="btn btn-error btn-sm"
              >
                <HiXMark className="w-4 h-4 mr-2" />
                Remove
              </button>
            </div>
          </article>
        ))}
      </section>
    </>
  )
}
