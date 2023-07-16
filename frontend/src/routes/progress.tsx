import {useStateMachine} from "little-state-machine"
import {useState} from "react"
import {HiArrowDown, HiCodeBracket, HiRocketLaunch} from "react-icons/hi2"
import {FormState} from "../types/form-state"
import {formatSeconds} from "../helpers"
import {Progress} from "../types/types.generated"
import useNotification from "../hooks/useNotification"
import {updateForm} from "./actions"
import {Link} from "react-router-dom"

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
  const {state, actions} = useStateMachine({updateForm})
  const [progress, setProgress] = useState<Progress>()
  const [times, setTimes] = useState<RingBuffer<number>>(new RingBuffer(5))

  const eta = times.buffer.reduce((sum, time) => sum + time, 0) / times.size
  const [finished, setFinished] = useState(false)
  const [finalFileName, setFinalFileName] = useState("")

  const fileName = state.data.fileName || `Compilation [${state.data.id}].mp4`
  const sendNotification = useNotification()
  const numSongs = state.data.songs?.length || 0
  const interactive = numSongs > 0 || state.data.interactive

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
      actions.updateForm({
        finalFileName: fileName,
      })
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

  const totalDuration = state.data.clips!.reduce(
    (duration, clip) => duration + (clip.range[1] - clip.range[0]),
    0,
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
          <h1 className="text-5xl font-bold text-center">🎉 Success!</h1>
          <p>
            You can now download the finished compilation.{" "}
            {interactive && (
              <>
                You can also create a <code>.funscript</code> file for use with
                sex toys.
              </>
            )}
          </p>
          <div className="flex flex-row gap-4 w-full">
            <div className="flex flex-col">
              <a
                href={`/api/download?fileName=${encodeURIComponent(
                  finalFileName,
                )}`}
                className="btn btn-success btn-lg w-64"
                download
              >
                <HiArrowDown className="w-6 h-6 mr-2" />
                Download video
              </a>
            </div>
            {interactive && (
              <div className="flex flex-col">
                <Link
                  className="btn btn-primary btn-lg w-64"
                  to="/stash/funscript"
                >
                  <HiCodeBracket className="w-6 h-6 mr-2" />
                  Create funscript
                </Link>
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  )
}

export default Progress
