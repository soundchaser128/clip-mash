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
import {FormStage, StateHelpers} from "../../types/types"
import invariant from "tiny-invariant"
import Layout from "../../components/Layout"
import {getUrl} from "./filter/root"
import Step from "../../components/Step"

const StashRoot: React.FC = () => {
  const {state, actions} = useStateMachine({resetForm})
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
  invariant(StateHelpers.isStash(state.data))

  const stage = state.data.stage
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

  let criteria = "criteria"
  if (state.data.selectMode) {
    criteria = state.data.selectMode
  }

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
        <ul className="steps steps-vertical lg:steps-horizontal self-center mb-4">
          <Step
            currentStage={stage}
            activeStage={FormStage.SelectMode}
            link="/stash/mode"
          >
            Choose mode
          </Step>
          <Step
            currentStage={stage}
            activeStage={FormStage.SelectCriteria}
            link={state.data.selectMode ? getUrl(state.data.selectMode) : ""}
          >
            Select {criteria}
          </Step>
          <Step
            currentStage={stage}
            activeStage={FormStage.SelectMarkers}
            link="/stash/markers"
          >
            Select markers
          </Step>
          <Step
            currentStage={stage}
            activeStage={FormStage.VideoOptions}
            link="/stash/video-options"
          >
            Select video options
          </Step>
          <Step
            currentStage={stage}
            activeStage={FormStage.PreviewClips}
            link="/stash/clips"
          >
            Preview clips
          </Step>
          <Step
            currentStage={stage}
            activeStage={FormStage.Wait}
            link="/stash/progress"
          >
            Wait for video
          </Step>
        </ul>
        <Outlet />
      </section>
    </Layout>
  )
}

export default StashRoot
