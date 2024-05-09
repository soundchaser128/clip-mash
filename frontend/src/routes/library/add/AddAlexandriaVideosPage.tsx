import {AlexandriaVideo, AlexandriaVideoPage, ListVideoDto} from "@/api"
import PageSizeSelect from "@/components/PageSizeSelect"
import Pagination from "@/components/Pagination"
import VideoCard from "@/components/VideoCard"
import useDebouncedSetQuery from "@/hooks/useDebouncedQuery"
import {useState} from "react"
import {useLoaderData, useSearchParams} from "react-router-dom"

function toVideDto(video: AlexandriaVideo): ListVideoDto {
  return {
    markerCount: 0,
    video: {
      createdOn: new Date(video.createdOn).getTime(),
      duration: 0.0,
      fileName: "",
      filePath: null,
      id: video.id,
      interactive: false,
      performers: [],
      source: "Alexandria",
      stashSceneId: null,
      tags: video.tags.filter((t) => t.includes(":")),
      title: video.title,
    },
  }
}

const AddAlexandriaVideosPage = () => {
  const [search] = useSearchParams()
  const [query, setQuery] = useState<string>(search.get("query") || "")
  const data = useLoaderData() as {videos: AlexandriaVideoPage}
  const {setQueryDebounced, addOrReplaceParam} = useDebouncedSetQuery()

  const onFilterChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setQuery(e.target.value)
    setQueryDebounced(e.target.value.trim())
  }

  return (
    <>
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
      </section>
      <section className="py-2">
        <PageSizeSelect />
      </section>
      <section className="grid grid-cols-3 gap-2 w-full mb-4">
        {data.videos?.content?.map((video) => (
          <VideoCard
            key={video.id}
            video={toVideDto(video)}
            aspectRatio="wide"
            getVideoUrl={(video) =>
              `https://content-next.soundchaser128.xyz/content/${video.id}.mp4`
            }
            getThumbnailUrl={(video) =>
              `https://content-next.soundchaser128.xyz/thumbnail/500/500/${video.id}.webp`
            }
          />
        ))}
      </section>

      <Pagination
        totalPages={data.videos.totalPages}
        currentPage={data.videos.number}
        startIndex={1}
      />
    </>
  )
}

export default AddAlexandriaVideosPage
