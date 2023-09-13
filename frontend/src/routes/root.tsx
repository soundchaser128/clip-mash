import {useStateMachine} from "little-state-machine"
import React, {useEffect} from "react"
import {
  Outlet,
  useLoaderData,
  useNavigate,
  useNavigation,
} from "react-router-dom"
import {HiXMark} from "react-icons/hi2"
import {resetForm} from "./actions"
import Layout from "../components/Layout"
import {getUrl} from "./stash/filter/root"
import Steps from "../components/Steps"
import {
  FormStage,
  LocalFilesFormStage,
  LocalVideosFormState,
  StashFormState,
  StateHelpers,
} from "../types/form-state"

const LocalFileSteps: React.FC<{state: LocalVideosFormState}> = ({state}) => {
  return (
    <Steps
      currentStage={state.stage}
      steps={[
        {
          stage: LocalFilesFormStage.ListVideos,
          link: "/local/videos",
          content: "Video library",
        },
        {
          stage: LocalFilesFormStage.SelectMarkers,
          link: "/local/markers",
          content: "Select markers",
        },
        {
          stage: LocalFilesFormStage.Music,
          link: "/stash/music",
          content: "Music options",
        },
        {
          stage: LocalFilesFormStage.VideoOptions,
          link: "/stash/video-options",
          content: "Video options",
        },
        {
          stage: LocalFilesFormStage.PreviewClips,
          link: "/stash/clips",
          content: "Preview clips",
        },
        {
          stage: LocalFilesFormStage.Wait,
          link: "/stash/progress",
          content: "Create video",
        },
      ]}
    />
  )
}

const AssistantLayout: React.FC = () => {
  const {actions, state} = useStateMachine({resetForm})
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
  const navigate = useNavigate()
  const navigation = useNavigation()
  const isLoading = navigation.state === "loading"
  const config = useLoaderData()
  const configExists = config !== null
  const isStashMode = StateHelpers.isStash(state.data)

  useEffect(() => {
    if (!configExists && isStashMode) {
      navigate("/stash/config")
    }
  }, [configExists, isStashMode, navigate])

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
        </div>
        {StateHelpers.isLocalFiles(state.data) && (
          <LocalFileSteps state={state.data} />
        )}
        <Outlet />
      </section>
    </Layout>
  )
}

export default AssistantLayout
