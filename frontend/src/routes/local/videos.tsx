import {useStateMachine} from "little-state-machine"
import {VideoWithMarkers, Page} from "../../types/types"
import {HiChevronRight, HiFolder, HiPlus, HiTag, HiXMark} from "react-icons/hi2"
import {useEffect, useState} from "react"
import {useImmer} from "use-immer"
import {updateForm} from "../actions"
import {
  Link,
  LoaderFunction,
  Outlet,
  useLoaderData,
  useNavigate,
  useNavigation,
  useSearchParams,
} from "react-router-dom"
import {ListVideoDto} from "../../api"
import Pagination from "../../components/Pagination"
import debounce from "lodash.debounce"
import {listVideos} from "../../api"
import {FormStage} from "../../types/form-state"
import VideoCard from "../../components/VideoCard"

export const loader: LoaderFunction = async ({request}) => {
  const url = new URL(request.url)
  const query = url.searchParams
  query.set("size", "18")
  const object = Object.fromEntries(query.entries())
  const videos = await listVideos(object)

  return videos
}

export default function ListVideos() {
  const {actions} = useStateMachine({updateForm})
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
    navigate(`/library/${video.id.id}/markers`)
  }

  const onFilterChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setFilter(e.target.value)
    debouncedSetQuery(e.target.value.trim())
  }

  const onNextStage = () => {
    const interactive = videos
      .filter((v) => v.markers.length > 0)
      .some((v) => v.video.interactive)

    actions.updateForm({
      stage: FormStage.SelectMarkers,
      interactive,
      selectedMarkers: undefined,
    })
    navigate("/markers")
  }

  return (
    <>
      <Outlet />
      <div className="my-4 grid grid-cols-3 items-center">
        <div className="flex gap-2">
          <Link to="add" className="btn btn-accent">
            <HiPlus className="mr-2" />
            Add videos
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
          <VideoCard
            key={video.video.id.id}
            video={video}
            actionChildren={
              <>
                <span />
                <button
                  onClick={() => onOpenModal(video)}
                  className="btn btn-sm btn-primary"
                >
                  <HiTag className="w-4 h-4 mr-2" />
                  Markers
                </button>
              </>
            }
          />
        ))}
      </section>

      <Pagination
        totalPages={initialVideos.totalPages}
        currentPage={initialVideos.pageNumber}
      />
    </>
  )
}
