import {useStateMachine} from "little-state-machine"

function Progress() {
  const {state} = useStateMachine()

  const onSubmit = async () => {
    const body = JSON.stringify(state.data)
    const response = await fetch("/api/create", {
      method: "POST",
      body,
      headers: {"content-type": "application/json"},
    })
    console.log(response)
  }

  return (
    <div>
      <pre>{JSON.stringify(state.data, null, 2)}</pre>

      <button onClick={onSubmit} className="btn btn-success">
        Create video
      </button>
    </div>
  )
}

export default Progress
