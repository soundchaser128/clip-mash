import {useStateMachine} from "little-state-machine"
import {useEffect, useRef, useState} from "react"

interface Progress {
  finished: number
  total: number
}

function Progress() {
  const {state} = useStateMachine()
  const [progress, setProgress] = useState<Progress>()
  const [finished, setFinished] = useState(false)
  const [fileName, setFileName] = useState("")

  const onSubmit = async () => {
    const body = JSON.stringify(state.data)
    const response = await fetch("/api/create", {
      method: "POST",
      body,
      headers: {"content-type": "application/json"},
    })

    if (response.ok) {
      let fileName = await response.text()
      setFileName(fileName)
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

  return (
    <div className="mt-8 max-w-lg w-full self-center flex flex-col items-center">
      {!progress && !finished && (
        <button onClick={onSubmit} className="btn btn-lg btn-success">
          Create video
        </button>
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
        <div className="text-center flex flex-col text-xl gap-4 mt-8">
          <p>
            <strong>Success!</strong>
          </p>
          <p>You can download your finished video here:</p>
          <a
            href={`/api/download?fileName=${encodeURIComponent(fileName)}`}
            className="btn btn-success btn-lg"
            download
          >
            Download
          </a>
        </div>
      )}
    </div>
  )
}

export default Progress
