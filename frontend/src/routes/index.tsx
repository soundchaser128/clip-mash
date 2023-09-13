import {useStateMachine} from "little-state-machine"
import {useLoaderData, useNavigate} from "react-router-dom"
import {updateForm} from "./actions"
import {VideoSource} from "../types/types"
import Layout from "../components/Layout"
import {HiComputerDesktop} from "react-icons/hi2"
import {FormStage, LocalFilesFormStage} from "../types/form-state"

export default function InitialRoot() {
  const id = useLoaderData() as string

  const navigate = useNavigate()
  const {actions} = useStateMachine({updateForm})
  const onNextStage = (mode: VideoSource) => {
    const update = {
      source: mode,
      videoId: id,
      stage:
        mode === "localFile"
          ? LocalFilesFormStage.ListVideos
          : FormStage.SelectMode,
    }

    // @ts-expect-error meh
    actions.updateForm(update)

    navigate(mode === "localFile" ? "/local/videos" : "/stash/mode")
  }

  return (
    <Layout>
      <div className="hero">
        <div className="hero-content self-center">
          <div className="max-w-md flex flex-col">
            <img src="/logo.png" className="w-40 self-center" />
            <p className="mt-2 text-lg text-center opacity-60">
              ClipMash helps you create video compilations.
            </p>
            <div className="self-center btn-group">
              <button
                className="btn btn-lg btn-secondary w-52"
                onClick={() => onNextStage("localFile")}
              >
                <HiComputerDesktop className="mr-2 w-6 h-6" />
                Start
              </button>
            </div>
          </div>
        </div>
      </div>
    </Layout>
  )
}
