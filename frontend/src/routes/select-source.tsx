import {useStateMachine} from "little-state-machine"
import {useNavigate} from "react-router-dom"
import {updateForm} from "./actions"
import {VideoSource} from "../types/types"
import {nanoid} from "nanoid"
import Layout from "../components/Layout"

export default function InitialRoot() {
  const navigate = useNavigate()
  const {actions} = useStateMachine({updateForm})
  const onNextStage = (mode: VideoSource) => {
    actions.updateForm({
      source: mode,
      id: nanoid(8),
    })
    navigate(mode === "local-files" ? "/local/path" : "/stash/mode")
  }

  return (
    <Layout>
      <h1 className="text-4xl font-bold my-4 text-center">ClipMash</h1>
      <section className="flex flex-col">
        <div className="flex flex-col items-start gap-4">
          <p className="text-center w-full">
            Choose from where to get videos. You can either use local files or
            connect to a Stash instance.
          </p>

          <div className="self-center grid grid-cols-1 md:grid-cols-2 gap-2">
            <button
              onClick={() => onNextStage("stash")}
              className="btn btn-lg btn-secondary"
            >
              Stash
            </button>

            <button
              className="btn btn-lg btn-secondary"
              onClick={() => onNextStage("local-files")}
            >
              Local videos
            </button>
          </div>
        </div>
      </section>
    </Layout>
  )
}
