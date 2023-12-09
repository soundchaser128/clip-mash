import React, {useEffect, useState} from "react"
import {
  Link,
  useLoaderData,
  useNavigate,
  useRevalidator,
  useSearchParams,
} from "react-router-dom"
import VideoCard from "../../../components/VideoCard"
import {StashLoaderData} from "../../loaders"
import Pagination from "../../../components/Pagination"
import {HiCheck, HiChevronLeft, HiPlus} from "react-icons/hi2"
import {AddVideosRequest, StashVideoDto, addNewVideos} from "../../../api"
import {useConfig} from "../../../hooks/useConfig"
import useDebouncedSetQuery from "../../../hooks/useDebouncedQuery"
import {useCreateToast} from "@/hooks/useToast"

const AddStashVideoPage: React.FC = () => {
  const [search] = useSearchParams()
  const [query, setQuery] = useState<string>(search.get("query") || "")
  const data = useLoaderData() as StashLoaderData
  const revalidator = useRevalidator()
  const config = useConfig()
  const navigate = useNavigate()
  const [addingVideo, setAddingVideo] = useState<string | boolean>()
  const withMarkers = search.get("withMarkers") === "true"
  const createToast = useCreateToast()

  useEffect(() => {
    if (!config) {
      navigate("/settings")
      createToast({
        message: "Please configure your Stash settings first.",
        type: "info",
      })
    }
  }, [config, navigate])

  const {setQueryDebounced, addOrReplaceParam} = useDebouncedSetQuery()

  const onFilterChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setQuery(e.target.value)
    setQueryDebounced(e.target.value.trim())
  }

  const onAddVideo = async (video: StashVideoDto) => {
    setAddingVideo(video.id)
    const body: AddVideosRequest = {
      type: "stash",
      sceneIds: [parseInt(video.id)],
    }

    await addNewVideos(body)
    revalidator.revalidate()
    setAddingVideo(undefined)
  }

  const onAddEntirePage = async () => {
    setAddingVideo(true)
    const body: AddVideosRequest = {
      type: "stash",
      sceneIds: data.content
        .filter((video) => !video.existsInDatabase)
        .map((video) => parseInt(video.id)),
    }

    await addNewVideos(body)
    revalidator.revalidate()
    setAddingVideo(undefined)
  }

  return (
    <>
      <div className="grid grid-cols-3 items-baseline">
        <Link to="/library" className="flex items-center text-sm link">
          <HiChevronLeft className="mr-1 inline" />
          Back
        </Link>

        <h1 className="text-3xl text-center font-bold mb-4">
          Add videos from Stash
        </h1>
      </div>
      <section className="grid grid-cols-3 w-full">
        <div className="form-control">
          <input
            type="text"
            className="input input-bordered input-primary w-96"
            placeholder="Filter..."
            value={query}
            onChange={onFilterChange}
          />
        </div>
        <div className="form-control place-self-center">
          <label className="label cursor-pointer">
            <span className="label-text mr-3">Show videos with markers</span>
            <input
              type="checkbox"
              className="checkbox checkbox-primary"
              checked={withMarkers}
              onChange={(e) =>
                addOrReplaceParam(
                  "withMarkers",
                  e.target.checked ? "true" : undefined,
                )
              }
            />
          </label>
        </div>
        <button
          disabled={addingVideo === true}
          className="btn btn-success place-self-end"
          onClick={onAddEntirePage}
        >
          <HiPlus /> Add entire page
        </button>
      </section>
      <section className="grid grid-cols-1 lg:grid-cols-3 gap-4 w-full my-4">
        {data.content.map((video) => {
          const videoBeingAdded =
            addingVideo === video.id || addingVideo === true
          const {existsInDatabase} = video

          return (
            <VideoCard
              key={video.id}
              video={{video, markerCount: video.markerCount}}
              stashConfig={config}
              actionChildren={
                <>
                  <span />
                  <button
                    onClick={() => onAddVideo(video)}
                    className="btn btn-sm btn-success"
                    disabled={existsInDatabase || videoBeingAdded}
                  >
                    {!existsInDatabase && !videoBeingAdded && (
                      <>
                        <HiPlus /> Add
                      </>
                    )}

                    {videoBeingAdded && (
                      <>
                        <span className="loading loading-spinner loading-xs" />{" "}
                        Adding...
                      </>
                    )}

                    {existsInDatabase && (
                      <>
                        <HiCheck /> Added
                      </>
                    )}
                  </button>
                </>
              }
            />
          )
        })}
      </section>

      <Pagination
        totalPages={data.totalPages}
        currentPage={data.pageNumber}
        startIndex={1}
      />
    </>
  )
}

export default AddStashVideoPage
