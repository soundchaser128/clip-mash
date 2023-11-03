import {useStateMachine} from "little-state-machine"
import {
  HiChevronRight,
  HiOutlineArrowPath,
  HiPlus,
  HiTag,
  HiTrash,
} from "react-icons/hi2"
import {useState} from "react"
import {updateForm} from "../actions"
import {
  Link,
  Outlet,
  useLoaderData,
  useNavigate,
  useRevalidator,
  useSearchParams,
} from "react-router-dom"
import {
  ListVideoDtoPage,
  cleanupVideos,
  deleteVideo,
  mergeStashVideo,
} from "../../api"
import {FormStage} from "../../types/form-state"

import JumpToTop from "../../components/JumpToTop"
import VideoGrid from "@/components/VideoGrid"

export default function ListVideos() {
  const {actions} = useStateMachine({updateForm})
  const navigate = useNavigate()
  const page = useLoaderData() as ListVideoDtoPage
  const revalidator = useRevalidator()
  const [syncingVideo, setSyncingVideo] = useState<string>()
  const videos = page.content
  const [query] = useSearchParams()

  const onOpenModal = (videoId: string) => {
    const queryString = query.toString()
    navigate(`/library/${videoId}/markers?${queryString}`)
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
    if (
      confirm("Are you sure you want to remove this video and all its markers?")
    ) {
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

  const onSyncVideo = async (id: string) => {
    setSyncingVideo(id)
    await mergeStashVideo(id)
    revalidator.revalidate()
    setSyncingVideo(undefined)
  }

  const startRange = page.pageNumber * page.pageSize + 1
  const endRange = startRange + page.content.length - 1

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
        <div className="">
          {videos.length > 0 && (
            <>
              <p className="mb-1 text-center">
                Showing videos{" "}
                <strong>
                  {startRange}-{endRange}
                </strong>{" "}
                of <strong>{page.totalItems}</strong>.
              </p>
              <p className="text-sm">
                Click on a video to add or edit its markers. Use the search
                query, filters and pagination buttons to find the videos you are
                interested in. Once you are done, click Next.
              </p>
            </>
          )}
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

      <VideoGrid
        editableTitles
        onVideoClick={onOpenModal}
        actionChildren={(video) => (
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
        )}
      />
    </>
  )
}
