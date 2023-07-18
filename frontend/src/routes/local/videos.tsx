import {useStateMachine} from "little-state-machine"
import invariant from "tiny-invariant"
import {VideoWithMarkers, Page} from "../../types/types"
import {
  HiAdjustmentsVertical,
  HiArrowDownTray,
  HiCheck,
  HiChevronRight,
  HiClock,
  HiFolder,
  HiPlus,
  HiTag,
  HiXMark,
} from "react-icons/hi2"
import {useEffect, useState} from "react"
import {useImmer} from "use-immer"
import {updateForm} from "../actions"
import {
  Link,
  LoaderFunction,
  Outlet,
  json,
  useLoaderData,
  useNavigate,
  useNavigation,
  useSearchParams,
} from "react-router-dom"
import {formatSeconds} from "../../helpers"
import clsx from "clsx"
import {createNewMarker} from "./api"
import {ListVideoDto} from "../../types/types.generated"
import Pagination from "../../components/Pagination"
import debounce from "lodash.debounce"
import {LocalFilesFormStage, StateHelpers} from "../../types/form-state"

export const loader: LoaderFunction = async ({request}) => {
  const url = new URL(request.url)
  const query = url.searchParams
  query.set("size", "18")
  const response = await fetch(`/api/local/video?${query.toString()}`)
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
  const initialVideos = useLoaderData() as Page<ListVideoDto>
  const [videos, setVideos] = useImmer<ListVideoDto[]>(initialVideos.content)
  const navigate = useNavigate()
  const navigation = useNavigation()
  const isLoading = navigation.state === "loading"
  const [params, setParams] = useSearchParams()
  const [filter, setFilter] = useState(params.get("query") ?? "")
  const noVideos = videos.length === 0 && !filter && !isLoading
  const noVideosForFilter = videos.length === 0 && filter && !isLoading

  const setQuery = (query: string) => {
    setParams({query})
  }
  const debouncedSetQuery = debounce(setQuery, 500)

  useEffect(() => {
    setVideos(initialVideos.content)
  }, [initialVideos, setVideos])

  const onOpenModal = ({video}: VideoWithMarkers) => {
    navigate(`/local/videos/${video.id.id}`)
  }

  const onFilterChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setFilter(e.target.value)
    debouncedSetQuery(e.target.value.trim())
  }

  const onAddFullVideo = async (video: VideoWithMarkers) => {
    const duration = video.video.duration
    const result = await createNewMarker(
      video.video,
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
      interactive,
      selectedMarkers: undefined,
    })
    navigate("/local/markers")
  }

  return (
    <>
      <Outlet />
      <div className="my-4 grid grid-cols-3 items-center">
        <div className="flex gap-2">
          <Link to="download" className="btn btn-primary">
            <HiArrowDownTray className="mr-2" />
            Download videos
          </Link>
          <Link to="/local/path" className="btn btn-primary">
            <HiFolder className="mr-2" />
            Add folder with videos
          </Link>
        </div>
        <span className="text-center">
          Found <strong>{initialVideos.totalItems}</strong> videos.
        </span>
        {videos.length > 0 && (
          <button
            className="btn btn-success place-self-end"
            onClick={onNextStage}
          >
            Next
            <HiChevronRight className="ml-1" />
          </button>
        )}
      </div>
      {!noVideos && (
        <div className="w-full flex justify-between">
          <div className="flex gap-2">
            <input
              type="text"
              placeholder="Filter..."
              className="input input-primary w-full lg:w-96"
              value={filter}
              onChange={onFilterChange}
            />
          </div>
        </div>
      )}

      {noVideos && (
        <div className="flex flex-col items-center justify-center mt-8">
          <HiFolder className="text-8xl" />
          <h1 className="text-xl">No videos found</h1>
          <p>Add some by either downloading them or adding a video folder.</p>
        </div>
      )}

      {noVideosForFilter && (
        <div className="flex flex-col items-center justify-center mt-8">
          <HiXMark className="text-8xl" />
          <h1 className="text-xl">No videos found</h1>
          <p>Try changing the filter.</p>
        </div>
      )}

      <section className="grid grid-cols-1 lg:grid-cols-3 gap-4 w-full my-4">
        {videos.map((video) => (
          <article
            className={clsx(
              "card card-compact shadow-xl bg-base-200",
              video.markers.length > 0 && "ring-4 ring-green-500",
            )}
            key={video.video.id.id}
          >
            <figure>
              <img
                className="aspect-[16/9] object-cover w-full"
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
                    video.markers.length > 0 || video.video.duration <= 0
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

      <Pagination
        totalPages={initialVideos.totalPages}
        currentPage={initialVideos.pageNumber}
        prevLink={{search: `?page=${initialVideos.pageNumber - 1}`}}
        nextLink={{search: `?page=${initialVideos.pageNumber + 1}`}}
      />
    </>
  )
}
