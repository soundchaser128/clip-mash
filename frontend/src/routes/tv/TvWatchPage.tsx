import {ClipsResponse, fetchClipsInteractive} from "@/api"
import {getClipUrl} from "@/helpers/clips"
import {useEffect, useRef, useState} from "react"
import {useMatch, useParams} from "react-router-dom"

const suggestions = [
  "Blowjob",
  "Doggy Style",
  "Missionary",
  "Reverse Cowgirl",
  "Cowgirl",
  "Anal",
  "Titty Fucking",
  "Masturbation",
]

function randomSuggestion(): string {
  const index = Math.floor(Math.random() * suggestions.length)
  return suggestions[index]
}

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
    <main className="w-full h-screen flex flex-row">
      <section className="grow">
        <video
          src={clipUrl}
          onTimeUpdate={onVideoTimeUpdate}
          className="w-full h-full"
          ref={videoRef}
          autoPlay
          controls
          muted
        />
      </section>
      <section className="p-2 bg-base-200">
        <h1 className="text-4xl font-bold">TV</h1>
        <p className="text-lg">Query: {query}</p>
      </section>
    </main>
  )
}

export default TvWatchPage
