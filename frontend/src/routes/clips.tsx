import {useState} from "react"
import {LoaderFunction, useLoaderData} from "react-router-dom"
import {Clip, FormState} from "../types/types"

interface ClipsResponse {
  clips: Clip[]
  streams: Record<string, string>
}

export const loader: LoaderFunction = async () => {
  const json = sessionStorage.getItem("form-state")
  const state: {data: FormState} = JSON.parse(json!)
  const response = await fetch("/api/clips", {
    method: "POST",
    body: JSON.stringify({
      clipOrder: "scene-order",
      clipDuration: 15,
      selectedMarkers: state.data.selectedMarkers,
      markers: state.data.markers,
    }),
    headers: {"content-type": "application/json"},
  })
  return await response.json()
}

function PreviewClips() {
  const data = useLoaderData() as ClipsResponse
  const [currentClipIndex, setCurrentClipIndex] = useState(0)
  const currentClip = data.clips[currentClipIndex]
  const streamUrl = data.streams[currentClip.sceneId]
  const clipUrl = `${streamUrl}#t=${currentClip.range[0]},${currentClip.range[1]}`

  return (
    <>
      <video src={clipUrl} muted controls autoPlay />

      <div className="flex justify-between">
        <button
          className="btn"
          onClick={() => setCurrentClipIndex((i) => i - 1)}
          disabled={currentClipIndex === 0}
        >
          Previous clip
        </button>
        <button
          className="btn"
          onClick={() => setCurrentClipIndex((i) => i + 1)}
          disabled={currentClipIndex >= data.clips.length - 1}
        >
          Next clip
        </button>
      </div>
    </>
  )
}

export default PreviewClips
