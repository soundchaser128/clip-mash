import {useStateMachine} from "little-state-machine"
import React, {useEffect} from "react"
import {
  Outlet,
  useLoaderData,
  useNavigate,
  useNavigation,
} from "react-router-dom"
import {HiXMark} from "react-icons/hi2"
import {resetForm} from "../actions"
import {
  FormStage,
  LocalFilesFormStage,
  LocalVideosFormState,
  StashFormState,
  StateHelpers,
} from "../../types/types"
import Layout from "../../components/Layout"
import {getUrl} from "./filter/root"
import Steps from "../../components/Steps"

const StashSteps: React.FC<{state: StashFormState}> = ({state}) => {
  return (
    <Steps
      currentStage={state.stage}
      steps={[
        {
          stage: FormStage.SelectMode,
          link: "/stash/mode",
          content: "Choose mode",
        },
        {
          stage: FormStage.SelectCriteria,
          link: state.selectMode ? getUrl(state.selectMode) : "",
          content: "Choose mode",
        },
        {
          stage: FormStage.SelectMarkers,
          link: "/stash/markers",
          content: "Select markers",
        },
        {
          stage: FormStage.VideoOptions,
          link: "/stash/video-options",
          content: "Video options",
        },
        {
          stage: FormStage.PreviewClips,
          link: "/stash/clips",
          content: "Preview clips",
        },
        {
          stage: FormStage.Wait,
          link: "/stash/progress",
          content: "Create video",
        },
      ]}
    />
  )
}

const LocalFileSteps: React.FC<{state: LocalVideosFormState}> = ({state}) => {
  return (
    <Steps
      currentStage={state.stage}
      steps={[
        {
          stage: LocalFilesFormStage.SelectPath,
          link: "/local/path",
          content: "Select file path",
        },
        {
          stage: LocalFilesFormStage.ListVideos,
          link: "/local/videos",
          content: "Create markers",
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

const StashRoot: React.FC = () => {
  const {actions, state} = useStateMachine({resetForm})
  const onReset = () => {
    if (
      confirm(
        "Are you sure you want to reset the form and return to the start?"
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

  useEffect(() => {
    if (!configExists) {
      navigate("/config")
    }
  }, [configExists])

  return (
    <Layout isLoading={isLoading}>
      <section className="py-4 flex flex-col">
        <h1 className="text-5xl font-bold mb-4 text-center">ClipMash</h1>
        <div className="self-center flex gap-2 mb-4">
          <button onClick={onReset} className="btn btn-sm btn-error">
            <HiXMark className="w-5 h-5 mr-2" />
            Reset
          </button>
        </div>
        {StateHelpers.isStash(state.data) && <StashSteps state={state.data} />}
        {StateHelpers.isLocalFiles(state.data) && (
          <LocalFileSteps state={state.data} />
        )}
        <Outlet />
      </section>
    </Layout>
  )
}

export default StashRoot
