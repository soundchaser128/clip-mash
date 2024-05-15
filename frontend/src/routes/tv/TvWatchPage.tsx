import {ClipsResponse, fetchClipsInteractive} from "@/api"
import {getClipUrl} from "@/helpers/clips"
import clsx from "clsx"
import {useRef, useState} from "react"
import {HiChevronLeft, HiChevronRight, HiPause, HiPlay} from "react-icons/hi2"
import useDebouncedSetQuery from "@/hooks/useDebouncedQuery"
import {LoaderFunction, useLoaderData} from "react-router-dom"
import DataList, {Data, Description} from "@/components/DataList"

export const interactiveClipsLoader: LoaderFunction = async (request) => {
  const url = new URL(request.request.url)
  const searchParams = url.searchParams

  const response = await fetchClipsInteractive({
    markerTitles: searchParams.getAll("query"),
    clipDuration: parseFloat(searchParams.get("clipDuration") || "5.0"),
  })

  return {
    clips: response,
  }
}

const TvWatchPage: React.FC = () => {
  const {clips} = useLoaderData() as {clips: ClipsResponse}
  const [collapsed, setCollapsed] = useState(false)
  const [clipDuration, setClipDuration] = useState(5.0)
  const setQuery = useDebouncedSetQuery({
    parameterName: "clipDuration",
    replaceAll: false,
  })

  const videoRef = useRef<HTMLVideoElement>(null)
  const [isPlaying, setIsPlaying] = useState(true)
  const [index, setIndex] = useState(0)
  const length = clips?.clips?.length || 0
  const currentClip = length > 0 ? clips!.clips[index] : undefined
  const currentVideo = clips?.videos.find((v) => v.id === currentClip?.videoId)
  const nextClip = length > 0 ? clips!.clips[(index + 1) % length] : undefined
  const clipUrl = getClipUrl(clips?.streams || {}, currentClip)
  const nextClipUrl = getClipUrl(clips?.streams || {}, nextClip)

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

  const onClipDurationChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setClipDuration(event.target.valueAsNumber)
    setQuery.setQueryDebounced(event.target.value)
  }

  const togglePlay = () => {
    if (videoRef.current) {
      if (isPlaying) {
        videoRef.current.pause()
      } else {
        videoRef.current.play()
      }
      setIsPlaying((p) => !p)
    }
  }

  const onChangeClip = (direction: "prev" | "next") => {
    if (direction === "prev") {
      setIndex((c) => (c - 1 + length) % length)
    } else {
      setIndex((c) => (c + 1) % length)
    }
  }

  return (
    <main className="w-full h-screen flex flex-row">
      <section className="grow flex flex-col">
        <video
          src={clipUrl}
          onTimeUpdate={onVideoTimeUpdate}
          className="w-full h-full"
          ref={videoRef}
          autoPlay
          muted
          preload="auto"
        />

        <video preload="auto" className="hidden" src={nextClipUrl} />
      </section>
      <section
        className={clsx(
          "hidden lg:flex flex-col bg-base-200 p-4 overflow-y-scroll overflow-x-hidden text-lg relative",
          collapsed ? "w-4" : "w-1/4",
        )}
      >
        <button
          onClick={() => setCollapsed((set) => !set)}
          className="absolute top-1/2 left-1 btn btn-sm btn-circle btn-outline"
        >
          {collapsed ? <HiChevronLeft /> : <HiChevronRight />}
        </button>
        {!collapsed && (
          <>
            <h1 className="text-4xl font-bold">TV</h1>
            <div className="form-control">
              <input
                className="range-primary"
                type="range"
                min="2"
                max="30"
                step="0.5"
                value={clipDuration}
                onChange={onClipDurationChange}
              />
              <label className="label">
                <span className="label-text">
                  Clip duration ({clipDuration}s)
                </span>
              </label>
            </div>
            <DataList>
              <Description>Current clip:</Description>
              <Data>
                {currentVideo?.title} -{" "}
                <strong>{currentClip?.markerTitle}</strong>
              </Data>
            </DataList>

            <div className="join self-center">
              <button
                onClick={() => onChangeClip("prev")}
                className="btn btn-square btn-outline join-item"
              >
                <HiChevronLeft />
              </button>

              <button
                className={clsx("btn btn-square join-item", {
                  "btn-success": !isPlaying,
                  "btn-neutral": isPlaying,
                })}
                type="button"
                onClick={togglePlay}
              >
                {isPlaying ? (
                  <HiPause className="w-5 h-5" />
                ) : (
                  <HiPlay className="w-5 h-5" />
                )}
              </button>
              <button
                onClick={() => onChangeClip("next")}
                className="btn btn-square btn-outline join-item"
              >
                <HiChevronRight />
              </button>
            </div>
          </>
        )}
      </section>
    </main>
  )
}

export default TvWatchPage
