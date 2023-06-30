import {useStateMachine} from "little-state-machine"
import {useRef, useState} from "react"
import {
  HiArrowDown,
  HiCodeBracket,
  HiOutlineFolder,
  HiRocketLaunch,
} from "react-icons/hi2"
import {FormState} from "../types/types"
import {formatSeconds} from "../helpers"
import {Progress} from "../types.generated"
import useNotification from "../hooks/useNotification"

class RingBuffer<T> {
  buffer: T[]
  size: number

  constructor(size: number) {
    this.buffer = []
    this.size = size
  }

  get(index: number) {
    return this.buffer[index]
  }

  push(item: T): RingBuffer<T> {
    const buffer = [item, ...this.buffer].slice(0, this.size)
    const ringBuffer = new RingBuffer<T>(this.size)
    ringBuffer.buffer = buffer
    return ringBuffer
  }
}

type CreateVideoBody = Omit<FormState, "songs"> & {
  songIds: number[]
}

function Progress() {
  const {state} = useStateMachine()

  const [progress, setProgress] = useState<Progress>()
  const [times, setTimes] = useState<RingBuffer<number>>(new RingBuffer(5))

  const eta = times.buffer.reduce((sum, time) => sum + time, 0) / times.size
  const [finished, setFinished] = useState(false)
  const [finalFileName, setFinalFileName] = useState("")
  const downloadLink = useRef<HTMLAnchorElement>(null)
  const [creatingScript, setCreatingScript] = useState(false)
  const fileName = state.data.fileName || `Compilation [${state.data.id}].mp4`
  const sendNotification = useNotification()

  const onSubmit = async (e: React.MouseEvent) => {
    e.preventDefault()

    const songIds = state.data.songs?.map((s) => s.songId) || []
    const data = {
      ...state.data,
      fileName,
      songIds,
    } satisfies CreateVideoBody

    const body = JSON.stringify(data)
    const response = await fetch("/api/create", {
      method: "POST",
      body,
      headers: {"content-type": "application/json"},
    })

    if (response.ok) {
      const fileName = await response.text()
      setFinalFileName(fileName)
      const es = new EventSource("/api/progress")
      es.onmessage = (event) => {
        const data = JSON.parse(event.data) as Progress
        if (data.done) {
          setFinished(true)
          es.close()
          sendNotification("Success", "Video generation finished!")
        }
        setProgress(data)
        setTimes((buf) => buf.push(data.etaSeconds))
      }
    }
  }

  const onDownloadFunscript = async (
    e: React.MouseEvent<HTMLButtonElement>
  ) => {
    e.preventDefault()
    setCreatingScript(true)
    const body = JSON.stringify(state.data)
    const response = await fetch("/api/funscript", {
      method: "POST",
      body,
      headers: {"content-type": "application/json"},
    })

    const script = await response.blob()
    const file = finalFileName.replace(".mp4", ".funscript")
    const downloadUrl = URL.createObjectURL(script)
    if (downloadLink.current) {
      downloadLink.current.href = downloadUrl
      downloadLink.current.download = file
      downloadLink.current.click()
    }
    setCreatingScript(false)
  }

  const onOpenVideosFolder = async () => {
    await fetch("/api/directory/open?folder=videos")
  }

  const totalDuration = state.data.clips!.reduce(
    (duration, clip) => duration + (clip.range[1] - clip.range[0]),
    0
  )

  return (
    <div className="mt-8 max-w-lg w-full self-center flex flex-col items-center">
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
          <p>
            <strong>{formatSeconds(progress.itemsFinished, "short")}</strong> /{" "}
            <strong>{formatSeconds(progress.itemsTotal, "short")}</strong> of
            the compilation finished
          </p>
          <p>
            Estimated time remaining: <strong>{formatSeconds(eta)}</strong>
          </p>
          <p>{progress.message}</p>
        </div>
      )}

      {finished && (
        <div className="flex flex-col gap-6">
          <h1 className="text-5xl font-bold">🎉 Success!</h1>
          <div className="flex flex-col">
            <p className="font-light self-start mb-1">
              Download the finished compilation
            </p>
            <a
              href={`/api/download?fileName=${encodeURIComponent(
                finalFileName
              )}`}
              className="btn btn-success btn-lg"
              download
            >
              <HiArrowDown className="w-6 h-6 mr-2" />
              Download
            </a>
          </div>

          {state.data.interactive && (
            <div className="flex flex-col">
              <p className="font-light self-start mb-1">
                Download the generated .funscript file
              </p>
              <button
                onClick={onDownloadFunscript}
                className="btn btn-success btn-lg"
                disabled={creatingScript}
              >
                <HiCodeBracket className="w-6 h-6 mr-2" />
                Funscript
              </button>

              <a className="hidden" ref={downloadLink} />
            </div>
          )}

          <div className="flex flex-col">
            <p className="font-light self-start mb-1">Open the videos folder</p>
            <button
              className="btn btn-success btn-lg"
              onClick={onOpenVideosFolder}
            >
              <HiOutlineFolder className="w-6 h-6 mr-2" />
              Open
            </button>
          </div>
        </div>
      )}
    </div>
  )
}

export default Progress
