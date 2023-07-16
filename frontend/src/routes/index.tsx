import {useStateMachine} from "little-state-machine"
import {useLoaderData, useNavigate} from "react-router-dom"
import {updateForm} from "./actions"
import {VideoSource} from "../types/types"
import Layout from "../components/Layout"
import {HiArchiveBox, HiComputerDesktop} from "react-icons/hi2"
import {FormStage, LocalFilesFormStage} from "../types/form-state"

export default function InitialRoot() {
  const id = useLoaderData() as string

  const navigate = useNavigate()
  const {actions} = useStateMachine({updateForm})
  const onNextStage = (mode: VideoSource) => {
    const update = {
      source: mode,
      id,
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
            <p className="mt-2 text-lg text-center text-gray-500">
              ClipMash helps you create video compilations.
            </p>

            <div className="text-lg mb-4">
              <h2 className="font-bold mt-4">Choose video source</h2>
              <p className="text-gray-500">
                You can either use files on your disk or connect to a{" "}
                <a href="https://stashapp.cc" className="link link-primary">
                  Stash
                </a>{" "}
                instance.
              </p>
            </div>

            <div className="self-center btn-group">
              <button
                onClick={() => onNextStage("stash")}
                className="btn btn-lg btn-primary w-52"
              >
                <HiArchiveBox className="mr-2 w-6 h-6" />
                Stash
              </button>

              <button
                className="btn btn-lg btn-secondary w-52"
                onClick={() => onNextStage("localFile")}
              >
                <HiComputerDesktop className="mr-2 w-6 h-6" />
                Local files
              </button>
            </div>
          </div>
        </div>
      </div>
    </Layout>
  )
}
