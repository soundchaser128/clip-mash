import React, {useCallback, useEffect, useState} from "react"
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
import debounce from "lodash.debounce"

const AddStashVideoPage: React.FC = () => {
  const [search, setSearchParams] = useSearchParams()
  const [query, setQuery] = useState<string>(search.get("query") || "")
  const data = useLoaderData() as StashLoaderData
  const revalidator = useRevalidator()
  const config = useConfig()
  const navigate = useNavigate()

  useEffect(() => {
    if (!config) {
      navigate("/stash/config")
    }
  }, [config, navigate])

  const setParams = useCallback(
    (query: string | undefined) => {
      if (query) {
        setSearchParams((prev) => {
          const params = new URLSearchParams(prev)
          params.set("query", query)
          return params
        })
      }
    },
    [setSearchParams],
  )

  useEffect(() => {
    setParams(query)
  }, [setParams, query])

  const debouncedSetQuery = debounce(setParams, 500)

  const onFilterChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setQuery(e.target.value)
    debouncedSetQuery(e.target.value.trim())
  }

  const onAddVideo = async (video: StashVideoDto) => {
    const body: AddVideosRequest = {
      type: "stash",
      scene_ids: [parseInt(video.id)],
    }

    await addNewVideos(body)
    revalidator.revalidate()
  }

  const onAddEntirePage = async () => {
    const body: AddVideosRequest = {
      type: "stash",
      scene_ids: data.content
        .filter((video) => !video.existsInDatabase)
        .map((video) => parseInt(video.id)),
    }

    await addNewVideos(body)
    revalidator.revalidate()
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
      <section className="flex justify-between w-full">
        <div className="form-control">
          <input
            type="text"
            className="input input-bordered input-primary w-96"
            placeholder="Filter..."
            value={query}
            onChange={onFilterChange}
          />
        </div>
        <button className="btn btn-success" onClick={onAddEntirePage}>
          <HiPlus /> Add entire page
        </button>
      </section>
      <section className="grid grid-cols-1 lg:grid-cols-3 gap-4 w-full my-4">
        {data.content.map((video) => (
          <VideoCard
            key={video.id}
            video={{video, markerCount: 0}}
            stashConfig={config}
            actionChildren={
              <>
                <span />
                <button
                  onClick={() => onAddVideo(video)}
                  className="btn btn-sm btn-success"
                  disabled={video.existsInDatabase}
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
