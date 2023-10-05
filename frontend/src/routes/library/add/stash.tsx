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
import {HiChevronLeft, HiPlus} from "react-icons/hi2"
import {AddVideosRequest, StashVideoDto, addNewVideos} from "../../../api"
import {useConfig} from "../../../hooks/useConfig"
import useDebouncedSetQuery from "../../../hooks/useDebouncedQuery"

const AddStashVideoPage: React.FC = () => {
  const [search] = useSearchParams()
  const [query, setQuery] = useState<string>(search.get("query") || "")
  const data = useLoaderData() as StashLoaderData
  const revalidator = useRevalidator()
  const config = useConfig()
  const navigate = useNavigate()
  const [addingVideo, setAddingVideo] = useState(false)
  const withMarkers = search.get("withMarkers") === "true"

  useEffect(() => {
    if (!config) {
      navigate("/stash/config")
    }
  }, [config, navigate])

  const {setQueryDebounced, addOrReplaceParam} = useDebouncedSetQuery()

  const onFilterChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setQuery(e.target.value)
    setQueryDebounced(e.target.value.trim())
  }

  const onAddVideo = async (video: StashVideoDto) => {
    setAddingVideo(true)
    const body: AddVideosRequest = {
      type: "stash",
      scene_ids: [parseInt(video.id)],
    }

    await addNewVideos(body)
    revalidator.revalidate()
    setAddingVideo(false)
  }

  const onAddEntirePage = async () => {
    setAddingVideo(true)
    const body: AddVideosRequest = {
      type: "stash",
      scene_ids: data.content
        .filter((video) => !video.existsInDatabase)
        .map((video) => parseInt(video.id)),
    }

    await addNewVideos(body)
    revalidator.revalidate()
    setAddingVideo(false)
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
          disabled={addingVideo}
          className="btn btn-success place-self-end"
          onClick={onAddEntirePage}
        >
          <HiPlus /> Add entire page
        </button>
      </section>
      <section className="grid grid-cols-1 lg:grid-cols-3 gap-4 w-full my-4">
        {data.content.map((video) => (
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
                  disabled={video.existsInDatabase || addingVideo}
                >
                  {video.existsInDatabase ? (
                    "Added"
                  ) : (
                    <>
                      <HiPlus /> Add
                    </>
                  )}
                </button>
              </>
            }
          />
        ))}
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
