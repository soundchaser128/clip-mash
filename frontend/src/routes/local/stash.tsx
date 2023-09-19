import React, {useEffect, useState} from "react"
import {useLoaderData, useSearchParams} from "react-router-dom"
import VideoCard from "../../components/VideoCard"
import {StashLoaderData} from "../loaders"
import Pagination from "../../components/Pagination"
import {HiPlus} from "react-icons/hi2"
import {AddVideosRequest, VideoDto, addNewVideos} from "../../api"

const AddStashVideoPage: React.FC = () => {
  const [search, setSearchParams] = useSearchParams({query: ""})
  const [query, setQuery] = useState(search.get("query") || "")
  const {videos: data, config} = useLoaderData() as StashLoaderData

  useEffect(() => {
    setSearchParams({query})
  }, [query])

  const onAddVideo = async (video: VideoDto) => {
    const body: AddVideosRequest = {
      type: "stash",
      scene_ids: [parseInt(video.id.id)],
    }

    await addNewVideos(body)
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
            key={video.video.id.id}
            video={video}
            stashConfig={config}
            actionChildren={
              <>
                <span />
                <button
                  onClick={() => onAddVideo(video.video)}
                  className="btn btn-sm btn-success"
                >
                  <HiPlus />
                  Add to library
                </button>
              </>
            }
          />
        ))}
      </section>

      <Pagination totalPages={data.totalPages} currentPage={data.pageNumber} />
    </>
  )
}

export default AddStashVideoPage
