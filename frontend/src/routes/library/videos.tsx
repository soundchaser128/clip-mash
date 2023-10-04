import {useStateMachine} from "little-state-machine"
import {Page} from "../../types/types"
import {
  HiChevronRight,
  HiFolder,
  HiPlus,
  HiTag,
  HiTrash,
  HiXMark,
} from "react-icons/hi2"
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
  useRevalidator,
  useSearchParams,
} from "react-router-dom"
import {ListVideoDto, cleanupVideos, deleteVideo, updateVideo} from "../../api"
import Pagination from "../../components/Pagination"
import {listVideos} from "../../api"
import {FormStage} from "../../types/form-state"
import VideoCard from "../../components/VideoCard"
import {useConfig} from "../../hooks/useConfig"
import {DEFAULT_PAGE_LENGTH} from "../loaders"
import useDebouncedSetQuery from "../../hooks/useDebouncedQuery"

export const loader: LoaderFunction = async ({request}) => {
  const url = new URL(request.url)
  const query = url.searchParams
  query.set("size", DEFAULT_PAGE_LENGTH.toString())
  const object = Object.fromEntries(query.entries())
  const videos = await listVideos(object)

  return videos
}

export default function ListVideos() {
  const {state, actions} = useStateMachine({updateForm})
  const initialVideos = useLoaderData() as Page<ListVideoDto>
  const [videos, setVideos] = useImmer<ListVideoDto[]>(initialVideos.content)
  const navigate = useNavigate()
  const navigation = useNavigation()
  const config = useConfig()
  const isLoading = navigation.state === "loading"
  const [params] = useSearchParams()
  const [filter, setFilter] = useState(params.get("query") ?? "")
  const noVideos = videos.length === 0 && !filter && !isLoading
  const noVideosForFilter = videos.length === 0 && filter && !isLoading
  const revalidator = useRevalidator()

  const debouncedSetQuery = useDebouncedSetQuery()

  useEffect(() => {
    setVideos(initialVideos.content)
  }, [initialVideos, setVideos])

  const onOpenModal = ({video}: ListVideoDto) => {
    navigate(`/library/${video.id}/markers`)
  }

  const onFilterChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setFilter(e.target.value)
    debouncedSetQuery(e.target.value.trim())
  }

  const onNextStage = () => {
    const interactive = videos
      .filter((v) => v.markerCount > 0)
      .some((v) => v.video.interactive)

    actions.updateForm({
      stage: FormStage.SelectMarkers,
      interactive,
      selectedMarkers: undefined,
    })
    navigate("/markers")
  }

  const onCheckboxChange = (id: string, selected: boolean) => {
    const existingIds = state.data.videoIds ?? []
    const newIds = selected
      ? [...existingIds, id]
      : existingIds.filter((v) => v !== id)

    actions.updateForm({videoIds: newIds})
  }

  const onToggleCheckbox = (id: string) => {
    const selected = state.data.videoIds?.includes(id) ?? false
    onCheckboxChange(id, !selected)
  }

  const onRemoveVideo = async (id: string) => {
    if (confirm("Are you sure you want to remove this video?")) {
      await deleteVideo(id)
      revalidator.revalidate()
    }
  }

  const onCleanupVideos = async () => {
    if (
      confirm(
        "This will delete all videos from the database that can no longer be found on disk. Are you sure?",
      )
    ) {
      const {deletedCount} = await cleanupVideos()
      alert(`${deletedCount} videos deleted.`)
      revalidator.revalidate()
    }
  }

  const onEditTitle = async (id: string, title: string) => {
    await updateVideo(id, {title})
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
          {videos.length > 0 && (
            <button onClick={onCleanupVideos} className="btn btn-error">
              <HiTrash className="mr-2" />
              Clean up
            </button>
          )}
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
        <div className="w-full grid grid-cols-3">
          <div className="flex gap-2">
            <input
              type="text"
              placeholder="Filter..."
              className="input input-primary w-full lg:w-96"
              value={filter}
              onChange={onFilterChange}
            />
          </div>

          <p className="place-self-center opacity-80">
            You can either select videos by clicking on them or just continue,
            this will automatically select all videos.
          </p>
        </div>
      )}

      {noVideos && (
        <div className="flex flex-col items-center justify-center mt-8">
          <HiFolder className="text-8xl" />
          <h1 className="text-xl">No videos found</h1>
          <p>Add some videos with the button above!</p>
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
            key={video.video.id}
            onImageClick={onToggleCheckbox}
            video={video}
            stashConfig={config}
            onEditTitle={(title) => onEditTitle(video.video.id, title)}
            actionChildren={
              <>
                <div className="form-control">
                  <label className="label cursor-pointer">
                    <span className="label-text">Include</span>
                    <input
                      type="checkbox"
                      className="toggle toggle-sm toggle-primary ml-2"
                      checked={state.data.videoIds?.includes(video.video.id)}
                      onChange={(e) =>
                        onCheckboxChange(video.video.id, e.target.checked)
                      }
                    />
                  </label>
                </div>
                <div className="flex gap-1">
                  <button
                    onClick={() => onRemoveVideo(video.video.id)}
                    className="btn btn-error btn-sm btn-square"
                  >
                    <HiTrash className="" />
                  </button>
                  <button
                    onClick={() => onOpenModal(video)}
                    className="btn btn-sm btn-primary"
                  >
                    <HiTag className="w-4 h-4 mr-2" />
                    Markers
                  </button>
                </div>
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
