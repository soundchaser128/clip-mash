import {useStateMachine} from "little-state-machine"
import {useEffect, useRef, useState} from "react"
import {
  HiArrowDown,
  HiCodeBracket,
  HiHeart,
  HiRocketLaunch,
} from "react-icons/hi2"
import {formatSeconds} from "../helpers"
import {CreateVideoBody, Progress, createVideo} from "../api"
import useNotification from "../hooks/useNotification"
import {updateForm} from "./actions"
import {Link} from "react-router-dom"
import clsx from "clsx"
import ExternalLink from "../components/ExternalLink"

function Progress() {
  const {state, actions} = useStateMachine({updateForm})
  const [progress, setProgress] = useState<Progress>()

  const [finished, setFinished] = useState(false)
  const [finalFileName, setFinalFileName] = useState(
    state.data.finalFileName || "",
  )

  const fileName = `${state.data.fileName || "Compilation"} [${
    state.data.videoId
  }].mp4`
  const sendNotification = useNotification()
  const numSongs = state.data.songs?.length || 0
  const interactive = numSongs > 0 || state.data.interactive
  const eventSource = useRef<EventSource>()
  const {videoId} = state.data

  const handleProgress = (data: Progress) => {
    if (data.done) {
      setFinished(true)
      eventSource.current?.close()
      sendNotification("Success", "Video generation finished!")
    }
    setProgress(data)
  }

  const openEventSource = () => {
    const es = new EventSource(`/api/progress/${videoId}/stream`)
    es.onmessage = (event) => {
      const data = JSON.parse(event.data) as Progress | null
      data && handleProgress(data)
    }
    return es
  }

  useEffect(() => {
    fetch(`/api/progress/${videoId}/info`)
      .then((res) => {
        if (res.ok) {
          return res.json()
        } else {
          throw new Error("Failed to fetch progress info")
        }
      })
      .then((json) => {
        const progress = json as Progress | null
        if (progress && progress.itemsTotal > 0) {
          handleProgress(progress)
          eventSource.current = openEventSource()
        }
      })

    return () => {
      eventSource.current?.close()
    }
  }, [])

  const totalDuration = state.data.clips!.reduce(
    (duration, clip) => duration + (clip.range[1] - clip.range[0]),
    0,
  )

  const onSubmit = async (e: React.MouseEvent) => {
    e.preventDefault()

    const data = {
      clips: state.data.clips!,
      fileName,
      songIds: state.data.songs?.map((s) => s.songId) || [],
      videoId: state.data.videoId!,
      encodingEffort: state.data.encodingEffort!,
      outputFps: state.data.outputFps!,
      outputResolution: state.data.outputResolution!,
      selectedMarkers: state.data.selectedMarkers!,
      videoCodec: state.data.videoCodec!,
      videoQuality: state.data.videoQuality!,
      musicVolume: state.data.musicVolume,
    } satisfies CreateVideoBody

    const response = await createVideo(data)
    setFinalFileName(response.finalFileName)
    actions.updateForm({
      fileName: response.finalFileName,
    })

    eventSource.current = openEventSource()
    setProgress({
      itemsFinished: 0,
      etaSeconds: 0,
      done: false,
      itemsTotal: totalDuration,
      message: "Starting...",
      timestamp: Date.now().toString(),
      videoId: state.data.videoId!,
    })
  }

  return (
    <div className="mt-2 w-full self-center flex flex-col items-center">
      {!progress && !finished && (
        <>
          <div className="mb-8">
            <p>
              Generating video with <strong>{state.data.clips?.length}</strong>{" "}
              clips.
            </p>
            <p>
              Estimated final duration:{" "}
              <strong>{formatSeconds(totalDuration)}</strong>.
            </p>
            <p>
              File name: <strong>{fileName}</strong>
            </p>
          </div>
          <a onClick={onSubmit} className="btn btn-lg btn-success">
            <HiRocketLaunch className="mr-2 w-6 h-6" />
            Create video
          </a>
        </>
      )}

      {progress && !finished && (
        <div className="w-full">
          <progress
            className="progress h-6 progress-primary w-full"
            value={progress?.itemsFinished}
            max={progress?.itemsTotal}
          />

          <section className="text-center">
            <p>
              <strong>{formatSeconds(progress.itemsFinished, "short")}</strong>{" "}
              / <strong>{formatSeconds(progress.itemsTotal, "short")}</strong>{" "}
              of the compilation finished
            </p>
            <p>
              Estimated time remaining:{" "}
              <strong>{Math.round(progress.etaSeconds || 0)} seconds</strong>
            </p>
            <p>{progress.message}</p>
          </section>
        </div>
      )}

      {finished && (
        <div className="flex flex-col gap-6">
          <h1 className="text-5xl font-bold text-center">🎉 Success!</h1>
          <p>
            You can now download the finished compilation!{" "}
            {interactive && (
              <>
                You can also create a <code>.funscript</code> file for use with
                sex toys.
              </>
            )}
          </p>
          <div
            className={clsx(
              "grid gap-2 w-full",
              interactive && "grid-cols-3",
              !interactive && "grid-cols-2",
            )}
          >
            <div className="flex flex-col">
              <a
                href={`/api/project/download?fileName=${encodeURIComponent(
                  finalFileName,
                )}`}
                className="btn btn-success btn-lg"
                download
              >
                <HiArrowDown className="w-6 h-6 mr-2" />
                Download video
              </a>
            </div>
            {interactive && (
              <div className="flex flex-col">
                <Link className="btn btn-primary btn-lg" to="/stash/funscript">
                  <HiCodeBracket className="w-6 h-6 mr-2" />
                  Create funscript
                </Link>
              </div>
            )}
            <div className="flex flex-col">
              <ExternalLink
                href="https://ko-fi.com/soundchaser128"
                className="btn btn-lg btn-secondary"
              >
                <HiHeart className="w-6 h-6 mr-2" />
                Support the developer
              </ExternalLink>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}

export default Progress
