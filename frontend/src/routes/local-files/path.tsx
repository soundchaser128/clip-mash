import {useState} from "react"
import {HiCheck} from "react-icons/hi2"
import {LocalVideo, StateHelpers} from "../../types/types"
import {useStateMachine} from "little-state-machine"
import {updateForm} from "../actions"
import invariant from "tiny-invariant"
import {useNavigate} from "react-router-dom"

export default function SelectVideos() {
  const {state, actions} = useStateMachine({updateForm})
  invariant(StateHelpers.isLocalFiles(state.data))
  const [path, setPath] = useState(state.data.localVideoPath || "")
  const navigate = useNavigate()

  const onSubmit: React.FormEventHandler = async (e) => {
    e.preventDefault()

    const response = await fetch(
      `/api/video?path=${encodeURIComponent(path)}`,
      {method: "POST"}
    )

    const json = (await response.json()) as LocalVideo[]
    actions.updateForm({
      source: "local-files",
      videos: json,
      localVideoPath: path,
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
        <button type="submit" className="btn btn-success">
          <HiCheck className="w-6 h-6 mr-2" />
          Submit
        </button>
      </form>
    </>
  )
}
