import {useStateMachine} from "little-state-machine"
import invariant from "tiny-invariant"
import {
  VideoWithMarkers,
  StateHelpers,
  LocalFilesFormStage,
} from "../../types/types"
import {
  HiAdjustmentsVertical,
  HiArrowDownTray,
  HiCheck,
  HiChevronRight,
  HiClock,
  HiInformationCircle,
  HiPlus,
  HiTag,
  HiXMark,
} from "react-icons/hi2"
import {useEffect} from "react"
import {useImmer} from "use-immer"
import {updateForm} from "../actions"
import {
  Link,
  LoaderFunction,
  Outlet,
  json,
  useLoaderData,
  useNavigate,
} from "react-router-dom"
import {formatSeconds, getFormState} from "../../helpers"
import clsx from "clsx"
import {persistMarker} from "./api"

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
  if (response.ok) {
    const data = await response.json()
    return json(data)
  } else {
    const text = await response.text()
    throw json({error: text, request: "/api/local/video"}, {status: 500})
  }
}

export default function ListVideos() {
  const {state, actions} = useStateMachine({updateForm})
  invariant(StateHelpers.isLocalFiles(state.data))
  const initialVideos = useLoaderData() as VideoWithMarkers[]
  const [videos, setVideos] = useImmer<VideoWithMarkers[]>(initialVideos)
  const navigate = useNavigate()

  useEffect(() => {
    setVideos(initialVideos)
  }, [initialVideos])

  const onOpenModal = ({video}: VideoWithMarkers) => {
    navigate(`/local/videos/${video.id.id}`)
  }

  const onAddFullVideo = async (video: VideoWithMarkers) => {
    const duration = video.video.duration
    const result = await persistMarker(
      video.video.id.id,
      {
        start: 0.0,
        end: duration,
        title: "Untitled",
      },
      duration,
      0
    )

    if (result.isOk) {
      const marker = result.unwrap()
      setVideos((draft) => {
        const video = draft.find((v) => v.video.id.id === marker.videoId.id)
        invariant(video)
        video.markers.push(marker)
      })
    } else {
      const error = result.error
      console.error(error)
    }
  }

  const onNextStage = () => {
    const interactive = videos
      .filter((v) => v.markers.length > 0)
      .some((v) => v.video.interactive)

    actions.updateForm({
      stage: LocalFilesFormStage.SelectMarkers,
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
          title: marker.primaryTag,
        })),
      interactive,
    })
    navigate("/local/markers")
  }

  return (
    <>
      <Outlet />
      {videos.length > 0 && (
        <div className="w-full flex justify-between">
          <Link to="download" className="btn btn-primary">
            <HiArrowDownTray className="mr-2" />
            Download videos
          </Link>

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
            Currently only <code>.mp4</code> files are supported.
          </span>
        </div>
      )}

      <section className="grid grid-cols-1 lg:grid-cols-3 gap-4 w-full my-4">
        {videos.map((video) => (
          <article
            className={clsx(
              "card card-compact shadow-xl bg-base-200",
              video.markers.length > 0 && "border-2 border-green-600"
            )}
            key={video.video.id.id}
          >
            <figure>
              <img
                className="aspect-[16/9] object-cover"
                src={`/api/local/video/${video.video.id.id}/preview`}
              />
            </figure>
            <div className="card-body">
              <h2 className="card-title">{video.video.fileName}</h2>
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
                <li>
                  <HiArrowDownTray className="inline mr-2" />
                  Source:{" "}
                  {video.video.source === "downloadedLocalFile"
                    ? "Downloaded"
                    : "Folder"}
                </li>
                <li>
                  <HiClock className="inline mr-2" />
                  Duration:{" "}
                  <strong>{formatSeconds(video.video.duration)}</strong>
                </li>
              </ul>
              <div className="card-actions justify-between grow items-end">
                <button
                  disabled={
                    video.markers.length > 0 && video.video.duration <= 0
                  }
                  onClick={() => onAddFullVideo(video)}
                  className="btn btn-sm btn-secondary"
                >
                  <HiPlus className="w-4 h-4" />
                  Add entire video
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
