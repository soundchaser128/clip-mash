import {useState} from "react"
import {HiCheck} from "react-icons/hi2"
import {StateHelpers} from "../../types/types"
import {useStateMachine} from "little-state-machine"
import {updateForm} from "../actions"
import invariant from "tiny-invariant"
import {useNavigate} from "react-router-dom"

export default function SelectVideos() {
  const {state, actions} = useStateMachine({updateForm})
  invariant(StateHelpers.isLocalFiles(state.data))
  const [path, setPath] = useState(state.data.localVideoPath || "")
  const [recurse, setRecurse] = useState(state.data.recurse || false)
  const navigate = useNavigate()

  const onSubmit: React.FormEventHandler = async (e) => {
    e.preventDefault()
    actions.updateForm({
      source: "local-files",
      localVideoPath: path,
      recurse,
    })
    navigate("/local/list")
  }

  return (
    <>
      <form onSubmit={onSubmit} className="flex gap-4 items-start flex-col">
        <div className="form-control">
          <label className="label">
            <span className="label-text">Local path for your videos</span>
          </label>
          <input
            type="text"
            className="input input-bordered w-96"
            value={path}
            onChange={(e) => setPath(e.target.value)}
            placeholder="C:\Users\CoolUser\Videos\DefinitelyNotPorn"
          />
        </div>
        <div className="form-control">
          <label className="label cursor-pointer">
            <span className="label-text mr-2">
              Look at all the subdirectories as well
            </span>
            <input
              type="checkbox"
              className="toggle"
              checked={recurse}
              onChange={(e) => setRecurse(e.target.checked)}
            />
          </label>
        </div>
        <button type="submit" className="btn btn-success">
          <HiCheck className="w-6 h-6 mr-2" />
          Submit
        </button>
      </form>
    </>
  )
}
