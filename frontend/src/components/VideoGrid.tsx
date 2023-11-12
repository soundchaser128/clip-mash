import {ListVideoDto, ListVideoDtoPage, updateVideo} from "@/api"
import VideoCard from "./VideoCard"
import {useLoaderData, useNavigation, useSearchParams} from "react-router-dom"
import {useForm} from "react-hook-form"
import {HiFolder, HiXMark} from "react-icons/hi2"
import Pagination from "./Pagination"
import useDebouncedSetQuery, {QueryPairs} from "@/hooks/useDebouncedQuery"
import {useEffect, useState} from "react"
import {useConfig} from "@/hooks/useConfig"
import AddTagModal from "./AddTagModal"
import clsx from "clsx"

interface Props {
  editableTitles?: boolean
  editableTags?: boolean
  actionChildren?: (video: ListVideoDto) => React.ReactNode
  onVideoClick: (id: string) => void
  hideMarkerCountFilter?: boolean
  isVideoDisabled?: (video: ListVideoDto) => boolean
  noVideosFoundMessage?: string
}

interface FilterInputs {
  query: string
  sort: string
  hasMarkers?: string
  isInteractive?: string
  source?: string
}

const EMPTY_VALUES: FilterInputs = {
  query: "",
  sort: "title",
}

const VideoGrid: React.FC<Props> = ({
  editableTitles,
  editableTags,
  actionChildren,
  onVideoClick,
  hideMarkerCountFilter,
  isVideoDisabled,
  noVideosFoundMessage,
}) => {
  const page = useLoaderData() as ListVideoDtoPage
  const [params] = useSearchParams()
  const [editingTags, setEditingTags] = useState<ListVideoDto | undefined>(
    undefined,
  )
  const [showingDetails, setShowingDetails] = useState(false)

  const config = useConfig()
  const {addOrReplaceParams, setQueryDebounced} = useDebouncedSetQuery()
  const videos = page.content
  const {register, handleSubmit, watch, reset} = useForm<FilterInputs>({
    mode: "onChange",
    defaultValues: Object.fromEntries(params.entries()),
  })

  const navigation = useNavigation()
  const isLoading = navigation.state === "loading"
  const values = watch()
  const formEmpty =
    !values.query?.trim() &&
    !values.sort &&
    !values.hasMarkers &&
    !values.isInteractive &&
    !values.source
  const noVideos = videos.length === 0 && formEmpty && !isLoading
  const noVideosForFilter = videos.length === 0 && !formEmpty && !isLoading

  const onSubmit = (values: FilterInputs) => {
    const hasQuery = !!values.query?.trim()
    const update: QueryPairs = [
      ["sort", values.sort],
      ["hasMarkers", values.hasMarkers],
      ["isInteractive", values.isInteractive],
      ["source", values.source === "All" ? undefined : values.source],
    ]
    if (hasQuery) {
      setQueryDebounced(values.query.trim())
    } else {
      update.push(["query", undefined])
    }
    addOrReplaceParams(update)
  }

  const onEditTitle = async (id: string, title: string) => {
    await updateVideo(id, {title})
  }

  const onShowTagModal = (video: ListVideoDto) => {
    setEditingTags(video)
  }

  useEffect(() => {
    const subscription = watch(() => handleSubmit(onSubmit)())
    return () => subscription.unsubscribe()
  }, [handleSubmit, watch])

  return (
    <>
      {!noVideos && (
        <form
          onSubmit={handleSubmit(onSubmit)}
          className="flex items-center w-full justify-between"
        >
          <input
            type="text grow"
            placeholder="Filter..."
            className="input input-primary w-full lg:w-96"
            {...register("query")}
          />

          <div className="flex gap-1">
            <div className="flex gap-1">
              <label className="label">
                <span className="label-text">Video source</span>
              </label>
              <select
                className="select select-sm select-bordered"
                {...register("source")}
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
                className="select select-sm select-bordered"
                {...register("sort")}
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
            {!hideMarkerCountFilter && (
              <div className="flex gap-1">
                <label className="label">
                  <span className="label-text">Has markers</span>
                </label>
                <select
                  className="select select-sm select-bordered"
                  {...register("hasMarkers")}
                >
                  <option disabled value="none">
                    Has markers
                  </option>
                  <option value="">All</option>
                  <option value="true">Yes</option>
                  <option value="false">No</option>
                </select>
              </div>
            )}

            <div className="flex gap-1">
              <label className="label">
                <span className="label-text">Interactive</span>
              </label>
              <select
                className="select select-sm select-bordered"
                {...register("isInteractive")}
              >
                <option disabled value="none">
                  Has markers
                </option>
                <option value="">All</option>
                <option value="true">Yes</option>
                <option value="false">No</option>
              </select>
            </div>
            <button
              type="button"
              onClick={() => reset(EMPTY_VALUES)}
              className="tooltip flex items-center hover:bg-gray-200 rounded-full p-2"
              data-tip="Reset values"
            >
              <HiXMark />
            </button>
          </div>
        </form>
      )}

      <div className="w-full flex justify-end">
        <span />
        <div className="flex items-center gap-1">
          <label className="label">
            <span className="label-text">Show details</span>
          </label>
          <input
            type="checkbox"
            className="toggle toggle-secondary"
            checked={showingDetails}
            onChange={(e) => setShowingDetails(e.target.checked)}
          />
        </div>
      </div>

      <AddTagModal
        video={editingTags}
        onClose={() => setEditingTags(undefined)}
      />

      {noVideos && (
        <div className="flex flex-col items-center justify-center mt-8">
          <HiFolder className="text-8xl" />
          <h1 className="text-xl">No videos found</h1>
          <p>
            {noVideosFoundMessage || "Add some videos with the button above!"}
          </p>
        </div>
      )}

      {noVideosForFilter && (
        <div className="flex flex-col items-center justify-center mt-8">
          <HiXMark className="text-8xl" />
          <h1 className="text-xl">No videos found</h1>
          <p>Try changing the filter.</p>
        </div>
      )}

      <section
        className={clsx("grid grid-cols-1 lg:grid-cols-3 w-full my-4", {
          "gap-3": showingDetails,
          "gap-1": !showingDetails,
        })}
      >
        {videos.map((video) => (
          <VideoCard
            key={video.video.id}
            video={video}
            actionChildren={actionChildren && actionChildren(video)}
            stashConfig={config}
            onImageClick={onVideoClick}
            disabled={isVideoDisabled ? isVideoDisabled(video) : false}
            onEditTitle={
              editableTitles && video.video.source !== "Stash"
                ? (title) => onEditTitle(video.video.id, title)
                : undefined
            }
            onAddTag={
              editableTags && video.video.source !== "Stash"
                ? (video) => onShowTagModal(video)
                : undefined
            }
            hideDetails={!showingDetails}
          />
        ))}
      </section>

      <Pagination totalPages={page.totalPages} currentPage={page.pageNumber} />
    </>
  )
}
export default VideoGrid
