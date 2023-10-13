import {useStateMachine} from "little-state-machine"
import {useCallback, useEffect, useRef, useState} from "react"
import {HiArchiveBox, HiRocketLaunch} from "react-icons/hi2"
import {formatSeconds, pluralize} from "../helpers"
import {CreateVideoBody, Progress, createVideo, getProgressInfo} from "../api"
import useNotification from "../hooks/useNotification"
import {useNavigate} from "react-router-dom"
import {FormState} from "../types/form-state"

const TWO_MINUTES = 120
const ONE_HOUR = 3600

const saveProjectToDisk = async (fileName: string, data: FormState) => {
  const json = JSON.stringify(data)
  const blob = new Blob([json], {type: "application/json"})
  const href = URL.createObjectURL(blob)
  const link = document.createElement("a")
  link.href = href
  link.download = fileName
  document.body.appendChild(link)
  link.click()
  document.body.removeChild(link)
  URL.revokeObjectURL(href)
}

const formatEta = (seconds: number): string => {
  if (seconds > ONE_HOUR) {
    const hours = Math.round(seconds / ONE_HOUR)
    const word = pluralize("hour", hours)
    return `${hours} ${word}`
  } else if (seconds > TWO_MINUTES) {
    const minutes = Math.round(seconds / 60)
    const word = pluralize("minute", minutes)
    return `${minutes} ${word}`
  } else {
    return `${Math.round(seconds)} seconds`
  }
}

function Progress() {
  const {state} = useStateMachine()
  const [progress, setProgress] = useState<Progress>()
  const fileName = `${state.data.fileName || "Compilation"} [${
    state.data.videoId
  }].mp4`
  const sendNotification = useNotification()
  const eventSource = useRef<EventSource>()
  const {videoId} = state.data
  const navigate = useNavigate()

  const handleProgress = useCallback(
    (data: Progress) => {
      if (data.done) {
        eventSource.current?.close()
        sendNotification("Success", "Video generation finished!")
        navigate(`/${videoId}/download`)
      }
      setProgress(data)
    },
    [navigate, sendNotification, videoId],
  )

  const openEventSource = useCallback(() => {
    const es = new EventSource(`/api/progress/${videoId}/stream`)
    es.onmessage = (event) => {
      const data = JSON.parse(event.data) as Progress | null
      data && handleProgress(data)
    }
    return es
  }, [handleProgress, videoId])

  useEffect(() => {
    getProgressInfo(videoId!).then((progress) => {
      if (progress && progress.itemsTotal > 0) {
        handleProgress(progress)
        eventSource.current = openEventSource()
      }
    })

    return () => {
      eventSource.current?.close()
    }
  }, [handleProgress, openEventSource, videoId])

  const totalDuration = state.data.clips!.reduce(
    (duration, clip) => duration + (clip.range[1] - clip.range[0]),
    0,
  )

  const onSaveToDisk = async () => {
    const projectName = `${state.data.videoId}.json`
    await saveProjectToDisk(projectName, state.data)
  }

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

    await createVideo(data)
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
      {!progress && (
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
          <div className="flex gap-2">
            <button onClick={onSubmit} className="btn btn-lg btn-success">
              <HiRocketLaunch className="mr-2 w-6 h-6" />
              Create video
            </button>
            <button onClick={onSaveToDisk} className="btn btn-lg btn-success">
              <HiArchiveBox className="mr-2 w-6 h-6" />
              Save project data
            </button>
          </div>
        </>
      )}

      {progress && (
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
            {progress.etaSeconds != undefined && (
              <p>
                Estimated time remaining:{" "}
                <strong
                  className="tooltip"
                  data-tip={formatSeconds(progress.etaSeconds)}
                >
                  {formatEta(progress.etaSeconds)}
                </strong>
              </p>
            )}

            <p>{progress.message}</p>
          </section>
        </div>
      )}
    </div>
  )
}

export default Progress
