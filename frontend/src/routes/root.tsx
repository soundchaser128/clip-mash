import {useStateMachine} from "little-state-machine"
import React from "react"
import {
  Outlet,
  useNavigate,
  useNavigation,
  useRouteLoaderData,
} from "react-router-dom"
import {HiOutlineDocumentArrowDown, HiXMark} from "react-icons/hi2"
import {resetForm} from "./actions"
import Layout from "../components/Layout"
import Steps from "../components/Steps"
import {FormState, FormStage, SerializedFormState} from "../types/form-state"

const LocalFileSteps: React.FC<{state: FormState}> = ({state}) => {
  return (
    <Steps
      currentStage={state.stage}
      steps={[
        {
          stage: FormStage.ListVideos,
          link: "/library",
          content: "Video library",
        },
        {
          stage: FormStage.SelectVideos,
          link: "/library/select",
          content: "Select videos",
        },
        {
          stage: FormStage.SelectMarkers,
          link: "/markers",
          content: "Select markers",
        },
        {
          stage: FormStage.Music,
          link: "/music",
          content: "Music options",
        },
        {
          stage: FormStage.VideoOptions,
          link: "/video-options",
          content: "Video options",
        },
        {
          stage: FormStage.PreviewClips,
          link: "/clips",
          content: "Preview clips",
        },
        {
          stage: FormStage.CreateVideo,
          link: "/generate",
          content: "Create video",
        },
      ]}
    />
  )
}

function saveFileToDisk<T>(fileName: string, data: T) {
  const json = JSON.stringify(data)
  const blob = new Blob([json], {type: "application/json"})
  const href = URL.createObjectURL(blob)
  const link = document.createElement("a")
  link.href = href
  link.download = fileName
  document.body.appendChild(link)
  link.click()
  document.body.removeChild(link)
  URL.revokeObjectURL(href)
}

const AssistantLayout: React.FC = () => {
  const {actions, state} = useStateMachine({resetForm})
  const version = useRouteLoaderData("root") as string

  const onReset = () => {
    if (
      confirm(
        "Are you sure you want to reset the form and return to the start?",
      )
    ) {
      actions.resetForm()
      navigate("/")
    }
  }

  const onSaveToDisk = async () => {
    const projectName = `${state.data.fileName || "Compilation"} - ${
      state.data.videoId
    }.json`
    const data: SerializedFormState = {
      ...state.data,
      clipMashVersion: version,
    }
    saveFileToDisk(projectName, data)
  }

  const navigate = useNavigate()
  const navigation = useNavigation()
  const isLoading = navigation.state === "loading"

  return (
    <Layout isLoading={isLoading}>
      <section className="py-4 flex flex-col">
        <h1 className="text-5xl text-primary font-bold mb-4 text-center">
          ClipMash
        </h1>
        <div className="self-center flex gap-2 mb-4">
          <button onClick={onReset} className="btn btn-sm btn-error">
            <HiXMark className="w-5 h-5 mr-2" />
            Reset
          </button>
          <button onClick={onSaveToDisk} className="btn btn-sm btn-success">
            <HiOutlineDocumentArrowDown className="mr-2 w-6 h-6" />
            Save project
          </button>
        </div>

        <LocalFileSteps state={state.data} />
        <Outlet />
      </section>
    </Layout>
  )
}

export default AssistantLayout
