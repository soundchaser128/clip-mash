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

  const onAddEntirePage = async () => {
    const body: AddVideosRequest = {
      type: "stash",
      scene_ids: data.content.map((video) => parseInt(video.id.id)),
    }

    await addNewVideos(body)
    revalidator.revalidate()
  }

  return (
    <>
      <h1 className="text-3xl text-center font-bold mb-4">
        Add videos from Stash
      </h1>
      <section className="flex justify-between w-full">
        <div className="form-control">
          <input
            type="text"
            className="input input-bordered input-primary w-96"
            placeholder="Filter..."
            value={query}
            onChange={(e) => setQuery(e.target.value)}
          />
        </div>
        <button className="btn btn-success" onClick={onAddEntirePage}>
          <HiPlus /> Add entire page
        </button>
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
