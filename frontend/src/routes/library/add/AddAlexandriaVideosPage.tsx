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
      source: "Alexandria",
      stashSceneId: null,
      tags: video.tags,
      title: video.title,
    },
  }
}

const AddAlexandriaVideosPage = () => {
  const data = useLoaderData() as {videos: AlexandriaVideoPage}

  return (
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
  )
}

export default AddAlexandriaVideosPage
