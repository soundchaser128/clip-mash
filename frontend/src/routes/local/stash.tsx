import React, {useEffect, useState} from "react"
import {useLoaderData, useRevalidator, useSearchParams} from "react-router-dom"
import VideoCard from "../../components/VideoCard"
import {StashLoaderData} from "../loaders"
import Pagination from "../../components/Pagination"
import {HiPlus} from "react-icons/hi2"
import {AddVideosRequest, StashVideoDto, addNewVideos} from "../../api"
import {useConfig} from "../../hooks/useConfig"

const AddStashVideoPage: React.FC = () => {
  const [search, setSearchParams] = useSearchParams()
  const [query, setQuery] = useState<string>(search.get("query") || "")
  const data = useLoaderData() as StashLoaderData
  const revalidator = useRevalidator()
  const config = useConfig()

  useEffect(() => {
    if (query) {
      setSearchParams((prev) => {
        prev.set("query", query)
        return prev
      })
    }
  }, [query, setSearchParams])

  const onAddVideo = async (video: StashVideoDto) => {
    const body: AddVideosRequest = {
      type: "stash",
      scene_ids: [parseInt(video.id.id)],
    }

    await addNewVideos(body)
    revalidator.revalidate()
  }

  return (
    <>
      <h1 className="text-3xl text-center font-bold mb-4">
        Add videos from Stash
      </h1>
      <section className="flex max-w-xl self-center">
        <div className="form-control">
          <label className="label">
            <span className="label-text">Search for videos</span>
          </label>
          <input
            required
            type="text"
            className="input input-bordered w-96"
            placeholder="Enter query..."
            value={query}
            onChange={(e) => setQuery(e.target.value)}
          />
        </div>
      </section>
      <section className="grid grid-cols-1 lg:grid-cols-3 gap-4 w-full my-4">
        {data.content.map((video) => (
          <VideoCard
            key={video.id.id}
            video={{video, markers: []}}
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
