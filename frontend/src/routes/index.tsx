import {useStateMachine} from "little-state-machine"
import {useNavigate} from "react-router-dom"
import {updateForm} from "./actions"
import {FormStage, LocalFilesFormStage, VideoSource} from "../types/types"
import {nanoid} from "nanoid"
import Layout from "../components/Layout"
import {HiArchiveBox, HiComputerDesktop} from "react-icons/hi2"

export default function InitialRoot() {
  const navigate = useNavigate()
  const {actions} = useStateMachine({updateForm})
  const onNextStage = (mode: VideoSource) => {
    const update = {
      source: mode,
      id: nanoid(8),
      stage:
        mode === "localFile"
          ? LocalFilesFormStage.SelectPath
          : FormStage.SelectMode,
    }

    // @ts-expect-error meh
    actions.updateForm(update)

    navigate(mode === "localFile" ? "/local/path" : "/stash/mode")
  }

  return (
    <Layout>
      <div className="hero">
        <div className="hero-content self-center">
          <div className="max-w-md flex flex-col">
            <img src="/logo.png" className="w-40 self-center" />
            <p className="mt-4 text-lg">
              ClipMash helps you create video compilations.
            </p>

            <p className="pb-6 pt-3 text-lg">
              <strong>Choose from where to get videos:</strong> <br />
              You can either use files on your disk or connect to a{" "}
              <a href="https://stashapp.cc" className="link link-primary">
                Stash
              </a>{" "}
              instance.
            </p>

            <div className="self-center btn-group">
              <button
                onClick={() => onNextStage("stash")}
                className="btn btn-lg btn-primary w-40"
              >
                <HiArchiveBox className="mr-2 w-6 h-6" />
                Stash
              </button>

              <button
                className="btn btn-lg btn-secondary w-40"
                onClick={() => onNextStage("localFile")}
              >
                <HiComputerDesktop className="mr-2 w-6 h-6" />
                Local
              </button>
            </div>
          </div>
        </div>
      </div>
    </Layout>
  )
}
