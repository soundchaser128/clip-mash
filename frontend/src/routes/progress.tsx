import {useStateMachine} from "little-state-machine"
import {useRef, useState} from "react"
import {HiArrowDown, HiCodeBracket, HiOutlineFolder, HiPlay} from "react-icons/hi2"
import {
  LocalVideosFormState,
  StashFormState,
  StateHelpers,
} from "../types/types"
import invariant from "tiny-invariant"
import {formatSeconds} from "../helpers"

interface Progress {
  finished: number
  total: number
  done: boolean
}

type CreateVideoBody = Omit<LocalVideosFormState | StashFormState, "songs"> & {
  songIds: number[]
}

function Progress() {
  const {state} = useStateMachine()
  invariant(StateHelpers.isNotInitial(state.data))

  const [progress, setProgress] = useState<Progress>()
  const [finished, setFinished] = useState(false)
  const [finalFileName, setFinalFileName] = useState("")
  const downloadLink = useRef<HTMLAnchorElement>(null)
  const [creatingScript, setCreatingScript] = useState(false)

  const onSubmit = async (e: React.MouseEvent) => {
    invariant(StateHelpers.isNotInitial(state.data))
    e.preventDefault()

    const songIds = state.data.songs?.map((s) => s.songId) || []
    const data = {
      ...state.data,
      fileName: state.data.fileName || `${state.data.id}.mp4`,
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
        }
        setProgress(data)
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
    await fetch("/api/open-directory?folder=videos")
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
              File name: <strong>{state.data.fileName}</strong>
            </p>
          </div>
          <a onClick={onSubmit} className="btn btn-lg btn-success">
            <HiPlay className="mr-2 w-6 h-6" />
            Create video
          </a>
        </>
      )}

      {progress && !finished && (
        <div className="text-center w-full">
          <progress
            className="progress h-6 progress-primary w-full"
            value={progress?.finished}
            max={progress?.total}
          />
          <p>
            {progress.finished} / {progress.total} clips finished
          </p>
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
