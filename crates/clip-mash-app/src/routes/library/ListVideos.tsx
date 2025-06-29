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
  useLoaderData,
  useNavigate,
  useRevalidator,
  useSearchParams,
} from "react-router-dom"
import {
  PageListVideoDto,
  cleanupVideos,
  deleteVideo,
  mergeStashVideo,
} from "../../api"
import {FormStage} from "../../types/form-state"

import JumpToTop from "../../components/JumpToTop"
import VideoGrid from "@/components/VideoGrid"
import PageInfo from "@/components/PageInfo"
import {useCreateToast} from "@/hooks/useToast"
import clsx from "clsx"

export default function ListVideos() {
  const {actions} = useStateMachine({updateForm})
  const navigate = useNavigate()
  const page = useLoaderData() as PageListVideoDto
  const revalidator = useRevalidator()
  const [syncingVideo, setSyncingVideo] = useState<string>()
  const videos = page.content
  const [query] = useSearchParams()
  const createToast = useCreateToast()

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
      createToast({
        type: "success",
        message: `${deletedCount} videos deleted.`,
      })
      revalidator.revalidate()
    }
  }

  const onSyncVideo = async (id: string) => {
    setSyncingVideo(id)
    await mergeStashVideo(id)
    revalidator.revalidate()
    setSyncingVideo(undefined)
  }

  return (
    <>
      <JumpToTop />
      <div className="my-4 grid grid-cols-3 items-center">
        <div className="flex gap-2">
          <Link to="add" className="btn btn-success">
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
            <PageInfo page={page} className="mb-1 text-center" />
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
        editableTags
        onVideoClick={onOpenModal}
        actionChildren={(video, aspectRatio) => (
          <>
            <div
              className={clsx("flex gap-1 items-center", {
                "flex-col": aspectRatio === "tall",
              })}
            >
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
                className="btn btn-error btn-sm"
              >
                <HiTrash /> Delete
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
