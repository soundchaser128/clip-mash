import {useStateMachine} from "little-state-machine"
import {useEffect, useRef, useState} from "react"

interface Progress {
  finished: number
  total: number
}

function Progress() {
  const {state} = useStateMachine()
  const eventSource = useRef<EventSource>()
  const [progress, setProgress] = useState<Progress>()

  const onSubmit = async () => {
    const body = JSON.stringify(state.data)
    const response = await fetch("/api/create", {
      method: "POST",
      body,
      headers: {"content-type": "application/json"},
    })

    if (response.ok) {
      const es = new EventSource("/api/progress")
      es.onmessage = (event) => {
        const data = JSON.parse(event.data) as Progress
        setProgress(data)
      }
      eventSource.current = es
    }
  }

  useEffect(() => {
    return () => eventSource.current && eventSource.current.close()
  }, [eventSource.current])

  return (
    <div className="mt-8 max-w-lg w-full self-center flex flex-col items-center">
      {!progress && (
        <button onClick={onSubmit} className="btn btn-lg btn-success">
          Create video
        </button>
      )}

      {progress && (
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
    </div>
  )
}

export default Progress
