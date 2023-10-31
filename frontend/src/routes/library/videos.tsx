import {useStateMachine} from "little-state-machine"
import {
  HiChevronRight,
  HiFolder,
  HiOutlineArrowPath,
  HiPlus,
  HiTag,
  HiTrash,
  HiXMark,
} from "react-icons/hi2"
import {useState} from "react"
import {updateForm} from "../actions"
import {
  Link,
  Outlet,
  useLoaderData,
  useNavigate,
  useNavigation,
  useRevalidator,
  useSearchParams,
} from "react-router-dom"
import {
  ListVideoDtoPage,
  cleanupVideos,
  deleteVideo,
  mergeStashVideo,
  updateVideo,
} from "../../api"
import Pagination from "../../components/Pagination"
import {FormStage} from "../../types/form-state"
import VideoCard from "../../components/VideoCard"
import {useConfig} from "../../hooks/useConfig"
import useDebouncedSetQuery from "../../hooks/useDebouncedQuery"
import JumpToTop from "../../components/JumpToTop"

export default function ListVideos() {
  const {actions} = useStateMachine({updateForm})
  const initialVideos = useLoaderData() as ListVideoDtoPage
  const videos = initialVideos.content
  const navigate = useNavigate()
  const navigation = useNavigation()
  const config = useConfig()
  const isLoading = navigation.state === "loading"
  const [params] = useSearchParams()
  const [source, setSource] = useState(params.get("source") || undefined)
  const [filter, setFilter] = useState(params.get("query") ?? "")
  const noVideos = videos.length === 0 && !filter && !isLoading && !source
  const revalidator = useRevalidator()
  const [sort, setSort] = useState(params.get("sort") ?? "markers")
  const {setQueryDebounced, addOrReplaceParam} = useDebouncedSetQuery()
  const [syncingVideo, setSyncingVideo] = useState<string>()
  const [hasMarkers, setHasMarkers] = useState<string | undefined>(
    params.get("hasMarkers") ?? "",
  )
  const [query] = useSearchParams()
  const anyFilterSet = filter || source || hasMarkers

  const noVideosForFilter = videos.length === 0 && anyFilterSet && !isLoading

  const onOpenModal = (videoId: string) => {
    const queryString = query.toString()
    navigate(`/library/${videoId}/markers?${queryString}`)
  }

  const onFilterChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setFilter(e.target.value)
    setQueryDebounced(e.target.value.trim())
  }

  const onChangeSort = (e: React.ChangeEvent<HTMLSelectElement>) => {
    setSort(e.target.value)
    addOrReplaceParam("sort", e.target.value)
  }

  const onChangeHasMarkers = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const value = e.target.value
    setHasMarkers(value)
    if (value === "") {
      addOrReplaceParam("hasMarkers", undefined)
    } else {
      addOrReplaceParam("hasMarkers", value)
    }
  }

  const onChangeSource = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const value = e.target.value
    if (value === "All") {
      setSource(undefined)
      addOrReplaceParam("source", undefined)
    } else {
      setSource(value)
      addOrReplaceParam("source", value)
    }
  }

  const onNextStage = () => {
    const interactive = videos
      .filter((v) => v.markerCount > 0)
      .some((v) => v.video.interactive)

    actions.updateForm({
      stage: FormStage.SelectVideos,
      interactive,
      selectedMarkers: undefined,
    })
    navigate("/library/select")
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

  const onSyncVideo = async (id: string) => {
    setSyncingVideo(id)
    await mergeStashVideo(id)
    revalidator.revalidate()
    setSyncingVideo(undefined)
  }

  const startRange = initialVideos.pageNumber * initialVideos.pageSize + 1
  const endRange = startRange + initialVideos.content.length - 1

  return (
    <>
      <JumpToTop />
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
        <div className="text-center">
          <p>
            Showing videos{" "}
            <strong>
              {startRange}-{endRange}
            </strong>{" "}
            of <strong>{initialVideos.totalItems}</strong>.
          </p>
        </div>
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
        <div className="w-full grid grid-cols-2">
          <input
            type="text"
            placeholder="Filter..."
            className="input input-primary w-full lg:w-96"
            value={filter}
            onChange={onFilterChange}
          />

          <div className="flex gap-2 place-self-end">
            <div className="flex gap-1">
              <label className="label">
                <span className="label-text">Video source</span>
              </label>
              <select
                value={source}
                onChange={onChangeSource}
                className="select select-sm select-bordered"
              >
                <option disabled value="none">
                  Filter video source
                </option>
                <option value="All">All</option>
                <option value="Folder">Local folder</option>
                <option value="Stash">Stash</option>
                <option value="Download">Downloaded</option>
              </select>
            </div>

            <div className="flex gap-1">
              <label className="label">
                <span className="label-text">Sort by</span>
              </label>
              <select
                value={sort}
                onChange={onChangeSort}
                className="select select-sm select-bordered"
              >
                <option disabled value="none">
                  Sort by...
                </option>
                <option value="markers">Number of markers</option>
                <option value="title">Video title</option>
                <option value="created">Created on</option>
                <option value="duration">Duration</option>
              </select>
            </div>

            <div className="flex gap-1">
              <label className="label">
                <span className="label-text">Has markers</span>
              </label>
              <select
                value={hasMarkers}
                onChange={onChangeHasMarkers}
                className="select select-sm select-bordered"
              >
                <option disabled value="none">
                  Has markers
                </option>
                <option value="">All</option>
                <option value="true">Yes</option>
                <option value="false">No</option>
              </select>
            </div>
          </div>
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
            onImageClick={onOpenModal}
            video={video}
            stashConfig={config}
            onEditTitle={
              video.video.source === "Stash"
                ? undefined
                : (title) => onEditTitle(video.video.id, title)
            }
            actionChildren={
              <>
                <div className="form-control"></div>
                <div className="flex gap-1">
                  {video.video.source === "Stash" && (
                    <button
                      onClick={() => onSyncVideo(video.video.id)}
                      className="btn btn-sm btn-secondary"
                      disabled={syncingVideo === video.video.id}
                    >
                      <HiOutlineArrowPath />
                      Sync
                    </button>
                  )}
                  <button
                    onClick={() => onRemoveVideo(video.video.id)}
                    className="btn btn-error btn-sm btn-square"
                  >
                    <HiTrash />
                  </button>
                  <button
                    onClick={() => onOpenModal(video.video.id)}
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
