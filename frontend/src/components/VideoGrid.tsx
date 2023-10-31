import {ListVideoDto, ListVideoDtoPage} from "@/api"
import VideoCard from "./VideoCard"
import {useLoaderData} from "react-router-dom"
import {useForm} from "react-hook-form"
import {HiFolder, HiXMark} from "react-icons/hi2"
import Pagination from "./Pagination"

interface Props {}

interface FilterInputs {
  query: string
  sort: string
  hasMarkers?: boolean
  isInteractive?: boolean
}

const VideoGrid: React.FC<Props> = () => {
  const page = useLoaderData() as ListVideoDtoPage
  const videos = page.content
  const {handleSubmit} = useForm<FilterInputs>()
  const noVideos = false
  const noVideosForFilter = false

  return (
    <>
      {!noVideos && (
        <div className="w-full grid grid-cols-2">
          <input
            type="text"
            placeholder="Filter..."
            className="input input-primary w-full lg:w-96"
          />

          <div className="flex gap-2 place-self-end">
            <div className="flex gap-1">
              <label className="label">
                <span className="label-text">Video source</span>
              </label>
              <select className="select select-sm select-bordered">
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
              <select className="select select-sm select-bordered">
                <option disabled value="none">
                  Sort by...
                </option>
                <option value="markers">Number of markers</option>
                <option value="title">Video title</option>
                <option value="created">Created on</option>
                <option value="duration">Duration</option>
              </select>
            </div>

            <div className="flex gap-1">
              <label className="label">
                <span className="label-text">Has markers</span>
              </label>
              <select className="select select-sm select-bordered">
                <option disabled value="none">
                  Has markers
                </option>
                <option value="">All</option>
                <option value="true">Yes</option>
                <option value="false">No</option>
              </select>
            </div>
          </div>
        </div>
      )}

      {noVideos && (
        <div className="flex flex-col items-center justify-center mt-8">
          <HiFolder className="text-8xl" />
          <h1 className="text-xl">No videos found</h1>
          <p>Add some videos with the button above!</p>
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
            video={video}
            actionChildren={<></>}
          />
        ))}
      </section>

      <Pagination totalPages={page.totalPages} currentPage={page.pageNumber} />
    </>
  )
}
export default VideoGrid
