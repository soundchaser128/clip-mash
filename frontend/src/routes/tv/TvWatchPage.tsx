import {ClipsResponse, fetchClipsInteractive} from "@/api"
import {
  Player,
  PlayerContextProvider,
  PlayerControls,
} from "@/components/VideoPlayer"
import {getClipUrl} from "@/helpers/clips"
import {useEffect, useRef, useState} from "react"
import {useMatch, useParams} from "react-router-dom"

const TvWatchPage: React.FC = () => {
  const {query} = useParams()

  const videoRef = useRef<HTMLVideoElement>(null)
  const [clips, setClips] = useState<ClipsResponse>()
  const [index, setIndex] = useState(0)
  const length = clips?.clips?.length || 0
  const currentClip = length > 0 ? clips!.clips[index] : undefined
  const clipUrl = getClipUrl(clips?.streams || {}, currentClip)

  useEffect(() => {
    fetchClipsInteractive({
      query: query!,
      clipDuration: 10.0,
    }).then((res) => setClips(res))
  }, [])

  const onVideoTimeUpdate: React.ReactEventHandler<HTMLVideoElement> = (
    event,
  ) => {
    if (currentClip) {
      const endTimestamp = currentClip.range[1]
      const currentTime = event.currentTarget.currentTime
      if (Math.abs(endTimestamp - currentTime) <= 0.5) {
        setIndex((c) => (c + 1) % length)
      }
    }
  }

  return (
    <PlayerContextProvider>
      <main className="w-full h-screen flex flex-row">
        <section className="grow flex flex-col">
          {/* <video
          src={clipUrl}
          onTimeUpdate={onVideoTimeUpdate}
          className="w-full h-full"
          ref={videoRef}
          autoPlay
          muted
        /> */}

          <Player src={clipUrl!} autoPlay />
          <PlayerControls />
        </section>
        <section className="p-2 bg-base-200">
          <h1 className="text-4xl font-bold">TV</h1>
          <p className="text-lg">Query: {query}</p>
        </section>
      </main>
    </PlayerContextProvider>
  )
}

export default TvWatchPage
