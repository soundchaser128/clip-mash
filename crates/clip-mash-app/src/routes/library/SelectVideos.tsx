import {useStateMachine} from "little-state-machine"
import {HiCheck, HiChevronRight, HiXMark} from "react-icons/hi2"
import {updateForm} from "../actions"
import {useLoaderData, useNavigate} from "react-router-dom"
import {ListVideoDto, PageListVideoDto} from "@/api"
import {FormStage} from "@/types/form-state"
import JumpToTop from "@/components/JumpToTop"
import {pluralize} from "@/helpers/formatting"
import VideoGrid from "@/components/VideoGrid"
import PageInfo from "@/components/PageInfo"

export default function ListVideos() {
  const {state, actions} = useStateMachine({updateForm})
  const page = useLoaderData() as PageListVideoDto
  const videos = page.content
  const navigate = useNavigate()
  const count = state.data.videoIds?.length ?? 0
  const nextDisabled = count === 0

  const onNextStage = () => {
    const interactive = videos
      .filter(
        (v) => v.markerCount > 0 && state.data.videoIds?.includes(v.video.id),
      )
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

  const onDeselectAll = () => {
    const ids = videos.map((v) => v.video.id)
    const newIds = state.data.videoIds?.filter((v) => !ids.includes(v)) ?? []
    actions.updateForm({videoIds: newIds})
  }

  const onSelectAll = () => {
    const ids = videos.map((v) => v.video.id)
    const newIds = new Set([...(state.data.videoIds ?? []), ...ids])
    actions.updateForm({videoIds: Array.from(newIds)})
  }

  const isVideoDisabled = (video: ListVideoDto): boolean => {
    const videoIds = state.data.videoIds ?? []
    return !videoIds.includes(video.video.id)
  }

  return (
    <>
      <JumpToTop />
      <div className="grid grid-cols-3 w-full my-4 items-center">
        <div className="join">
          <button onClick={onSelectAll} className="btn btn-secondary join-item">
            <HiCheck className="mr-1" />
            Select all
          </button>
          <button onClick={onDeselectAll} className="btn btn-error join-item">
            <HiXMark className="mr-1" />
            Deselect all
          </button>
        </div>
        <div className="place-self-center text-center mb-4">
          <PageInfo page={page} />
          <p>
            <strong>{count}</strong>{" "}
            {pluralize("video", state.data.videoIds?.length)} selected.
          </p>
          <p>Click on videos to add them to the selection.</p>
        </div>
        <button
          className="btn btn-success place-self-end self-center"
          onClick={onNextStage}
          disabled={nextDisabled}
        >
          Next
          <HiChevronRight className="ml-1" />
        </button>
      </div>

      <VideoGrid
        hideMarkerCountFilter
        onVideoClick={onToggleCheckbox}
        isVideoDisabled={isVideoDisabled}
        noVideosFoundMessage="No videos with markers found. Try going back to the library and adding some markers."
        actionChildren={(video) => (
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
          </>
        )}
      />
    </>
  )
}
