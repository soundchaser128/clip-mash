import clsx from "clsx"
import {useStateMachine} from "little-state-machine"
import {useEffect} from "react"
import {Outlet, useLocation, useNavigate} from "react-router-dom"
import {FormStage} from "../types/types"
import {resetForm} from "./actions"

const stageMap = {
  [FormStage.SelectMode]: "/",
  [FormStage.SelectCriteria]: "/select-criteria",
  [FormStage.SelectMarkers]: "/select-markers",
  [FormStage.VideoOptions]: "/video-options",
  [FormStage.Wait]: "/progress",
}

export default function Root() {
  const {state, actions} = useStateMachine({resetForm})
  const stage = state.data.stage
  const correctUrl = stageMap[stage]
  const location = useLocation()
  const navigate = useNavigate()

  useEffect(() => {
    if (location.pathname !== correctUrl) {
      navigate(correctUrl)
    }
  }, [, correctUrl, location])

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

  return (
    <main className="container ml-auto mr-auto w-screen h-screen">
      <section className="py-4 flex flex-col">
        <h1 className="text-4xl font-bold mb-4 text-center">
          Stash Compilation Generator
        </h1>
        <div className="self-center flex gap-2 mb-4">
          <button onClick={onReset} className="btn btn-sm btn-error">
            Reset
          </button>
        </div>
        <ul className="steps mb-4">
          <li className="step step-primary">Choose mode</li>
          <li
            className={clsx(
              "step",
              stage >= FormStage.SelectCriteria && "step-primary"
            )}
          >
            Select criteria
          </li>
          <li
            className={clsx(
              "step",
              stage >= FormStage.SelectMarkers && "step-primary"
            )}
          >
            Select markers
          </li>
          <li
            className={clsx(
              "step",
              stage >= FormStage.VideoOptions && "step-primary"
            )}
          >
            Select video options
          </li>
          <li
            className={clsx("step", stage >= FormStage.Wait && "step-primary")}
          >
            Wait for video
          </li>
        </ul>
        <Outlet />
      </section>
    </main>
  )
}
