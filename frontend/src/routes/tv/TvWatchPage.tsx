import {
  ClipsResponse,
  SongDto,
  fetchClipsInteractive,
  listSongs,
  pauseHandy,
} from "@/api"
import {getClipUrl} from "@/helpers/clips"
import clsx from "clsx"
import {useRef, useState} from "react"
import {
  HiBackward,
  HiChevronLeft,
  HiChevronRight,
  HiForward,
  HiPause,
  HiPlay,
  HiSpeakerWave,
  HiSpeakerXMark,
} from "react-icons/hi2"
import useDebouncedSetQuery from "@/hooks/useDebouncedQuery"
import {
  Link,
  LoaderFunction,
  useLoaderData,
  useSearchParams,
} from "react-router-dom"
import DataList, {Data, Description} from "@/components/DataList"
import {clamp} from "@/helpers/math"
import Heading from "@/components/Heading"
import {TvQueryType} from "./TvStartPage"
import {formatSeconds} from "@/helpers/time"

function removeExtension(fileName: string) {
  const index = fileName.indexOf(".")
  return index === -1 ? fileName : fileName.substring(0, index)
}

interface LoaderData {
  clips: ClipsResponse
  music: SongDto[]
}

export const interactiveClipsLoader: LoaderFunction = async (request) => {
  const url = new URL(request.request.url)
  const searchParams = url.searchParams
  let music: SongDto[] = []

  if (searchParams.has("withMusic")) {
    music = await listSongs({shuffle: true})
  }

  const queryType = searchParams.get("queryType") as TvQueryType

  const seed = searchParams.get("seed")

  const response = await fetchClipsInteractive({
    query: {
      type: queryType,
      data: searchParams.getAll("query"),
    },
    clipDuration: parseFloat(searchParams.get("clipDuration") || "5.0"),
    order: {
      type: "scene",
    },
    seed,
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
  const [params] = useSearchParams()
  const handyEnabled = params.has("handyEnabled")

  const setQuery = useDebouncedSetQuery({
    parameterName: "clipDuration",
    replaceAll: false,
  })

  const [musicAudioBalance, setMusicAudioBalance] = useState(0.5)
  const [volume, setVolume] = useState(1.0)

  const videoRef = useRef<HTMLVideoElement>(null)
  const nextVideoRef = useRef<HTMLVideoElement>(null)
  const [currentSong, setCurrentSong] = useState(0)
  const [timePlayed, setTimePlayed] = useState(0)
  const totalDuration = clips?.clips.reduce(
    (acc, clip) => acc + clip.range[1] - clip.range[0],
    0,
  )
  const audioRef = useRef<HTMLAudioElement>(null)

  const [isPlaying, setIsPlaying] = useState(true)
  const [muted, setMuted] = useState(music.length === 0)
  const [index, setIndex] = useState(0)
  const length = clips?.clips?.length || 0
  const currentClip = length > 0 ? clips!.clips[index] : undefined
  const nextClip = length > 0 ? clips!.clips[(index + 1) % length] : undefined
  const nextClipUrl = getClipUrl(clips?.streams || {}, nextClip)
  const currentVideo = clips?.videos.find((v) => v.id === currentClip?.videoId)
  const clipUrl = getClipUrl(clips?.streams || {}, currentClip)

  const onToggleMuted = () => {
    setMuted((m) => !m)
  }

  const onVideoTimeUpdate: React.ReactEventHandler<HTMLVideoElement> = (
    event,
  ) => {
    if (currentClip) {
      const endTimestamp = currentClip.range[1]
      const currentTime = event.currentTarget.currentTime
      if (Math.abs(endTimestamp - currentTime) <= 0.5) {
        const clipLength = endTimestamp - currentClip.range[0]
        setTimePlayed((t) => t + clipLength)
        setIndex((c) => (c + 1) % length)
        if (videoRef.current) {
          videoRef.current.load()
        }
        if (nextVideoRef.current) {
          nextVideoRef.current.load()
        }
      }
    }
  }

  const onClipDurationChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setClipDuration(event.target.valueAsNumber)
    setQuery.setQueryDebounced(event.target.value)
  }

  const onTogglePlay = async () => {
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

      // toggles paused state
      if (handyEnabled) {
        await pauseHandy()
      }

      setIsPlaying((p) => !p)
    }
  }

  const onChangeClip = (direction: "prev" | "next") => {
    if (!currentClip) {
      console.warn("No current clip found")
      return
    }
    const clipDuration = currentClip.range[1] - currentClip.range[0]

    if (direction === "prev") {
      setTimePlayed((t) => t - clipDuration)
      setIndex((c) => clamp(c - 1, 0, length - 1))
    } else {
      setTimePlayed((t) => t + clipDuration)
      setIndex((c) => (c + 1) % length)
    }
  }

  const onBalanceChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const value = event.target.valueAsNumber
    setMusicAudioBalance(value)
    const musicVolume = volume * value
    const videoVolume = volume * (1 - value)
    if (audioRef.current) {
      audioRef.current.volume = musicVolume
    }
    if (videoRef.current) {
      videoRef.current.volume = videoVolume
    }
  }

  const onVolumeChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const value = event.target.valueAsNumber
    setVolume(value)
    const musicVolume = value * musicAudioBalance
    const videoVolume = value * (1 - musicAudioBalance)
    if (audioRef.current) {
      audioRef.current.volume = musicVolume
    }
    if (videoRef.current) {
      videoRef.current.volume = videoVolume
    }
  }

  return (
    <main className="w-full flex flex-col lg:flex-row">
      <section className="grow flex flex-col relative h-screen">
        <Link
          to="/tv"
          className="btn btn-primary btn-outline btn-square btn-sm absolute top-4 left-4 z-10"
        >
          <HiChevronLeft />
        </Link>

        <video
          onTimeUpdate={onVideoTimeUpdate}
          className="w-full cursor-pointer"
          style={{height: "calc(100vh - 4rem)"}}
          ref={videoRef}
          autoPlay
          preload="auto"
          muted={muted}
          onClick={onTogglePlay}
        >
          {clipUrl?.map((url, index) => (
            <source key={index} src={url.src} type={url.type} />
          ))}
        </video>
        <div className="w-full px-2">
          <progress
            value={timePlayed}
            max={totalDuration}
            className="w-full progress progress-primary h-4"
          />
          <div className="text-center text-sm pb-2">
            <p>
              {formatSeconds(timePlayed, "short")} /{" "}
              {formatSeconds(totalDuration, "short")}
            </p>
            <p>
              {currentVideo?.title} -{" "}
              <strong>{currentClip?.markerTitle}</strong>
            </p>
          </div>
        </div>

        <video preload="auto" className="hidden" ref={nextVideoRef}>
          {nextClipUrl?.map((url, index) => (
            <source key={index} src={url.src} type={url.type} />
          ))}
        </video>

        {music.length > 0 && (
          <audio
            ref={audioRef}
            src={`/api/song/${music[currentSong]?.songId}/stream`}
            onEnded={() => setCurrentSong((s) => (s + 1) % music.length)}
            className="hidden"
            autoPlay
            muted={muted}
          />
        )}
      </section>
      <section
        className={clsx(
          "flex flex-col bg-base-200 p-4 overflow-y-scroll overflow-x-hidden text-lg relative",
          collapsed ? "lg:w-4" : "lg:w-1/4 w-full",
        )}
      >
        <button
          onClick={() => setCollapsed((set) => !set)}
          className="hidden lg:flex absolute top-1/2 left-1 btn btn-sm btn-circle btn-outline"
        >
          {collapsed ? <HiChevronLeft /> : <HiChevronRight />}
        </button>
        {!collapsed && (
          <>
            <Heading className="text-center">ClipMash TV</Heading>
            <div className="join self-center mb-4">
              <button
                onClick={() => onChangeClip("prev")}
                className="btn btn-square join-item"
              >
                <HiBackward className="w-5 h-5" />
              </button>

              <button
                className={clsx("btn btn-square join-item", {
                  "btn-success": !isPlaying,
                  "btn-warning": isPlaying,
                })}
                type="button"
                onClick={onTogglePlay}
              >
                {isPlaying ? (
                  <HiPause className="w-5 h-5" />
                ) : (
                  <HiPlay className="w-5 h-5" />
                )}
              </button>
              <button
                onClick={onToggleMuted}
                className="btn btn-square join-item"
                type="button"
              >
                {muted ? (
                  <HiSpeakerWave className="w-5 h-5" />
                ) : (
                  <HiSpeakerXMark className="w-5 h-5" />
                )}
              </button>

              <button
                onClick={() => onChangeClip("next")}
                className="btn btn-square join-item"
              >
                <HiForward className="w-5 h-5" />
              </button>
            </div>

            <div className="form-control">
              <input
                className="range range-primary"
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
            <div className="form-control">
              <input
                className="range range-primary"
                type="range"
                min="0"
                max="1"
                step="0.05"
                value={volume}
                onChange={onVolumeChange}
              />
              <label className="label">
                <span className="label-text mb-4">Volume</span>
              </label>
            </div>
            {music.length > 0 && (
              <div className="form-control">
                <input
                  className="range range-primary"
                  type="range"
                  min="0"
                  max="1"
                  step="0.05"
                  value={musicAudioBalance}
                  onChange={onBalanceChange}
                />
                <label className="label">
                  <span className="label-text mb-4">Audio balance</span>
                </label>
              </div>
            )}

            <DataList>
              {music.length > 0 && (
                <>
                  <Description>Current Song</Description>
                  <Data className="truncate">
                    {removeExtension(music[currentSong].fileName)}
                  </Data>
                </>
              )}
            </DataList>
          </>
        )}
      </section>
    </main>
  )
}

export default TvWatchPage
