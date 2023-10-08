import {useStateMachine} from "little-state-machine"
import {HiCheck, HiChevronRight, HiXMark} from "react-icons/hi2"
import {useEffect, useState} from "react"
import {useImmer} from "use-immer"
import {updateForm} from "../actions"
import {
  useLoaderData,
  useNavigate,
  useNavigation,
  useSearchParams,
} from "react-router-dom"
import {ListVideoDto, ListVideoDtoPage} from "../../api"
import Pagination from "../../components/Pagination"
import {FormStage} from "../../types/form-state"
import VideoCard from "../../components/VideoCard"
import {useConfig} from "../../hooks/useConfig"
import useDebouncedSetQuery from "../../hooks/useDebouncedQuery"
import JumpToTop from "../../components/JumpToTop"
import {pluralize} from "../../helpers"

export default function ListVideos() {
  const {state, actions} = useStateMachine({updateForm})
  const initialVideos = useLoaderData() as ListVideoDtoPage
  const [videos, setVideos] = useImmer<ListVideoDto[]>(initialVideos.content)
  const navigate = useNavigate()
  const navigation = useNavigation()
  const config = useConfig()
  const isLoading = navigation.state === "loading"
  const [params] = useSearchParams()
  const [source, setSource] = useState(params.get("source") || undefined)
  const [filter, setFilter] = useState(params.get("query") ?? "")
  const noVideos = videos.length === 0 && !filter && !isLoading && !source
  const noVideosForFilter = videos.length === 0 && filter && !isLoading
  const [sort, setSort] = useState(params.get("sort") ?? "markers")
  const {setQueryDebounced, addOrReplaceParam} = useDebouncedSetQuery()

  useEffect(() => {
    setVideos(initialVideos.content)
  }, [initialVideos, setVideos])

  const onFilterChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setFilter(e.target.value)
    setQueryDebounced(e.target.value.trim())
  }

  const onChangeSort = (e: React.ChangeEvent<HTMLSelectElement>) => {
    setSort(e.target.value)
    addOrReplaceParam("sort", e.target.value)
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

  return (
    <>
      <JumpToTop />
      <div className="grid grid-cols-3 w-full">
        <span />
        <p className="place-self-center">
          <strong>{state.data.videoIds?.length || "All"}</strong>{" "}
          {pluralize("video", state.data.videoIds?.length)} selected.
        </p>
        <button
          className="btn btn-success place-self-end"
          onClick={onNextStage}
        >
          Next
          <HiChevronRight className="ml-1" />
        </button>
      </div>
      {!noVideos && (
        <div className="w-full grid grid-cols-3">
          <input
            type="text"
            placeholder="Filter..."
            className="input input-primary w-full lg:w-96"
            value={filter}
            onChange={onFilterChange}
          />

          <div className="flex gap-2 justify-center">
            <button onClick={onDeselectAll} className="btn btn-error">
              <HiXMark className="mr-1" />
              Deselect all
            </button>
            <button onClick={onSelectAll} className="btn btn-secondary">
              <HiCheck className="mr-1" />
              Select all
            </button>
          </div>

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
              </select>
            </div>
          </div>
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
            disabled={!state.data.videoIds?.includes(video.video.id) ?? false}
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
