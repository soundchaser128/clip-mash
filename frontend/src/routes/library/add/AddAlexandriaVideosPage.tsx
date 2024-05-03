import {AlexandriaVideo, AlexandriaVideoPage, ListVideoDto} from "@/api"
import VideoCard from "@/components/VideoCard"
import {useLoaderData} from "react-router-dom"

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
      source: "alexandria",
      stashSceneId: null,
      tags: video.tags,
      title: video.title,
    }
  }
}

const AddAlexandriaVideosPage = () => {
  const data = useLoaderData() as {videos: AlexandriaVideoPage}

  return (
    <section className="grid grid-cols-3 gap-2 w-full mb-4">
      {data.videos?.content?.map((video) => (
        <VideoCard key={video.id} video={toVideDto(video)} aspectRatio="wide" />
      ))}
    </section>
  )
}

export default AddAlexandriaVideosPage
