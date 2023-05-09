import {useStateMachine} from "little-state-machine"
import {useNavigate} from "react-router-dom"
import {updateForm} from "./actions"
import {FormStage, LocalFilesFormStage, VideoSource} from "../types/types"
import {nanoid} from "nanoid"
import Layout from "../components/Layout"

export default function InitialRoot() {
  const navigate = useNavigate()
  const {actions} = useStateMachine({updateForm})
  const onNextStage = (mode: VideoSource) => {
    const update = {
      source: mode,
      id: nanoid(8),
      stage:
        mode === "local-files"
          ? LocalFilesFormStage.SelectPath
          : FormStage.SelectMode,
    }

    // @ts-expect-error meh
    actions.updateForm(update)

    navigate(mode === "local-files" ? "/local/path" : "/stash/mode")
  }

  return (
    <Layout>
      <div className="hero">
        <div className="hero-content text-center">
          <div className="max-w-md">
            <h1 className="text-5xl font-bold">ClipMash</h1>
            <p className="py-6">
              Choose from where to get videos. You can either use local files or
              connect to a Stash instance.
            </p>

            <div className="self-center btn-group">
              <button
                onClick={() => onNextStage("stash")}
                className="btn btn-lg btn-primary w-40"
              >
                Stash
              </button>

              <button
                className="btn btn-lg btn-secondary w-40"
                onClick={() => onNextStage("local-files")}
              >
                Local
              </button>
            </div>
          </div>
        </div>
      </div>
    </Layout>
  )
}
