import {useStateMachine} from "little-state-machine"
import invariant from "tiny-invariant"
import {
  VideoWithMarkers,
  StateHelpers,
  FormStage,
  LocalFilesFormStage,
} from "../../types/types"
import {
  HiAdjustmentsVertical,
  HiCheck,
  HiChevronRight,
  HiInformationCircle,
  HiPlayPause,
  HiTag,
  HiXMark,
} from "react-icons/hi2"
import {useEffect, useState} from "react"
import {useImmer} from "use-immer"
import {updateForm} from "../actions"
import {
  LoaderFunction,
  Outlet,
  json,
  useLoaderData,
  useNavigate,
} from "react-router-dom"
import {getFormState} from "../../helpers"
import clsx from "clsx"

export const loader: LoaderFunction = async () => {
  const formState = getFormState()
  invariant(StateHelpers.isLocalFiles(formState!))

  const params = new URLSearchParams({
    path: formState.localVideoPath!,
    recurse: formState.recurse ? "true" : "false",
  })

  const response = await fetch(`/api/local/video?${params.toString()}`, {
    method: "POST",
  })
  const data = await response.json()
  return json(data)
}

export default function ListVideos() {
  const {state, actions} = useStateMachine({updateForm})
  invariant(StateHelpers.isLocalFiles(state.data))
  const initialVideos = useLoaderData() as VideoWithMarkers[]
  const [videos, setVideos] = useImmer<VideoWithMarkers[]>(initialVideos)
  const [videoPreview, setVideoPreview] = useState<string>()
  const navigate = useNavigate()

  useEffect(() => {
    setVideos(initialVideos)
  }, [initialVideos])

  const onOpenModal = ({video}: VideoWithMarkers) => {
    navigate(`/local/videos/${video.id.id}`)
  }

  const onNextStage = () => {
    actions.updateForm({
      stage: LocalFilesFormStage.VideoOptions,
      videos: videos.filter((v) => v.markers.length > 0),
      selectedMarkers: videos
        .flatMap((m) => m.markers)
        .map((marker) => ({
          duration: marker.end - marker.start,
          id: marker.id,
          indexWithinVideo: marker.indexWithinVideo,
          selected: true,
          selectedRange: [marker.start, marker.end],
          videoId: marker.videoId,
        })),
    })
    navigate("/local/options")
  }

  return (
    <>
      <Outlet />
      {videos.length > 0 && (
        <div className="w-full flex justify-between">
          <div>
            <p>
              Found <strong>{videos.length}</strong> videos in folder{" "}
              <code>{state.data.localVideoPath}</code>.
            </p>
            <p>
              <strong>Note:</strong> Only videos with markers will be added to
              the compilation. Others will be ignored.
            </p>
          </div>

          <button className="btn btn-success" onClick={onNextStage}>
            Next
            <HiChevronRight className="ml-1" />
          </button>
        </div>
      )}

      {videos.length === 0 && (
        <div className="mt-4 alert alert-info w-fit self-center">
          <HiInformationCircle className="stroke-current flex-shrink-0 h-6 w-6" />
          <span>
            No videos found at location &apos;{state.data.localVideoPath}&apos;.
            Currently only <code>.mp4</code>
            files are supported.
          </span>
        </div>
      )}

      <section className="grid grid-cols-1 lg:grid-cols-3 gap-2 w-full my-4">
        {videos.map((video) => (
          <article
            className="card card-compact bg-base-100 shadow-xl"
            key={video.video.id.id}
          >
            <figure className="">
              {videoPreview === video.video.id.id && (
                <video
                  controls
                  autoPlay
                  className="w-full aspect-video"
                  muted
                  src={`/api/local/video/${video.video.id.id}`}
                />
              )}
            </figure>
            <div className="card-body">
              <h2 className="card-title">
                <span className="truncate">{video.video.fileName}</span>
              </h2>
              <ul className="flex flex-col gap-2 self-start">
                <li>
                  <HiAdjustmentsVertical className="inline mr-2" />
                  Interactive:{" "}
                  <strong>
                    {video.video.interactive ? (
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
              <div className="card-actions justify-between">
                <button
                  onClick={() =>
                    setVideoPreview((id) =>
                      id ? undefined : video.video.id.id
                    )
                  }
                  className={clsx(
                    "btn btn-sm",
                    videoPreview === video.video.id.id
                      ? "btn-error"
                      : "btn-success"
                  )}
                >
                  <HiPlayPause className="mr-2" />
                  {videoPreview === video.video.id.id ? "Stop" : "Preview"}
                </button>

                <button
                  className="btn btn-sm btn-primary"
                  onClick={() => onOpenModal(video)}
                >
                  <HiTag className="w-4 h-4 mr-2" />
                  Markers
                </button>
              </div>
            </div>
          </article>
        ))}
      </section>
    </>
  )
}
