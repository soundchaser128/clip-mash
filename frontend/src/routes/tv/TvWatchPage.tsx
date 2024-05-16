import {ClipsResponse, SongDto, fetchClipsInteractive, listSongs} from "@/api"
import {getClipUrl} from "@/helpers/clips"
import clsx from "clsx"
import {useRef, useState} from "react"
import {HiChevronLeft, HiChevronRight, HiPause, HiPlay} from "react-icons/hi2"
import useDebouncedSetQuery from "@/hooks/useDebouncedQuery"
import {LoaderFunction, useLoaderData} from "react-router-dom"
import DataList, {Data, Description} from "@/components/DataList"
import {clamp} from "@/helpers/math"
import Heading from "@/components/Heading"

interface LoaderData {
  clips: ClipsResponse
  music: SongDto[]
}

export const interactiveClipsLoader: LoaderFunction = async (request) => {
  const url = new URL(request.request.url)
  const searchParams = url.searchParams
  let music: SongDto[] = []

  if (searchParams.has("withMusic")) {
    music = await listSongs()
  }

  const response = await fetchClipsInteractive({
    markerTitles: searchParams.getAll("query"),
    clipDuration: parseFloat(searchParams.get("clipDuration") || "5.0"),
    order: {
      type: "random",
    },
  })

  return {
    clips: response,
    music,
  } satisfies LoaderData
}

const TvWatchPage: React.FC = () => {
  const {clips, music} = useLoaderData() as LoaderData
  const [collapsed, setCollapsed] = useState(false)
  const [clipDuration, setClipDuration] = useState(5.0)
  const setQuery = useDebouncedSetQuery({
    parameterName: "clipDuration",
    replaceAll: false,
  })

  const videoRef = useRef<HTMLVideoElement>(null)
  const [currentSong, setCurrentSong] = useState(0)
  const audioRef = useRef<HTMLAudioElement>(null)

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
      if (Math.abs(endTimestamp - currentTime) <= 0.1) {
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
        if (audioRef.current) {
          audioRef.current.pause()
        }
      } else {
        videoRef.current.play()
        if (audioRef.current) {
          audioRef.current.play()
        }
      }
      setIsPlaying((p) => !p)
    }
  }

  const onChangeClip = (direction: "prev" | "next") => {
    if (direction === "prev") {
      setIndex((c) => clamp(c - 1, 0, length - 1))
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

        {music.length > 0 && (
          <audio
            ref={audioRef}
            src={`/api/song/${music[currentSong]?.songId}/stream`}
            onEnded={() => setCurrentSong((s) => (s + 1) % music.length)}
            className="hidden"
            autoPlay
          />
        )}
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
            <Heading className="text-center">ClipMash TV</Heading>
            <div className="join self-center mb-4">
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
                <span className="label-text mb-4">
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
          </>
        )}
      </section>
    </main>
  )
}

export default TvWatchPage
