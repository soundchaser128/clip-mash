import {useStateMachine} from "little-state-machine"
import {useRef, useState} from "react"
import {HiArrowDown, HiCheckBadge, HiCodeBracket} from "react-icons/hi2"
import {StateHelpers} from "../types/types"
import invariant from "tiny-invariant"
import {formatSeconds} from "../helpers"

interface Progress {
  finished: number
  total: number
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
    const data = {...state.data}
    if (!data.fileName) {
      data.fileName = `${data.id}.mp4`
    }

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
        const isFinished = data.finished === data.total
        if (isFinished) {
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
            <HiCheckBadge className="mr-2 w-6 h-6" />
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
        <div className="flex flex-col">
          <h1 className="text-5xl font-bold mb-6">Success!</h1>
          <p className="font-light self-start mb-1">
            Download the finished compilation
          </p>
          <a
            href={`/api/download?fileName=${encodeURIComponent(finalFileName)}`}
            className="btn btn-success btn-lg mb-8"
            download
          >
            <HiArrowDown className="w-6 h-6 mr-2" />
            Download
          </a>

          {state.data.interactive && (
            <>
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
            </>
          )}
        </div>
      )}
    </div>
  )
}

export default Progress
