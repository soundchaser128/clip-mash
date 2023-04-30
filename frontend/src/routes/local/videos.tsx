import {useStateMachine} from "little-state-machine"
import invariant from "tiny-invariant"
import {LocalVideoDto, StateHelpers} from "../../types/types"
import {
  HiAdjustmentsVertical,
  HiCheck,
  HiChevronRight,
  HiInformationCircle,
  HiTag,
  HiXMark,
} from "react-icons/hi2"
import {useEffect} from "react"
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

export default function ListVideos() {
  const {state, actions} = useStateMachine({updateForm})
  invariant(StateHelpers.isLocalFiles(state.data))
  const initialVideos = useLoaderData() as LocalVideoDto[]
  const [videos, setVideos] = useImmer<LocalVideoDto[]>(initialVideos)
  const navigate = useNavigate()

  useEffect(() => {
    setVideos(initialVideos)
  }, [initialVideos])

  const onOpenModal = (video: LocalVideoDto) => {
    navigate(`/local/videos/${video.id}`)
  }

  const onNextStage = () => {
    actions.updateForm({
      videos: videos.filter((v) => v.markers.length > 0),
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
              <div className="card-actions justify-end">
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
