import clsx from "clsx"
import {useStateMachine} from "little-state-machine"
import {Outlet} from "react-router-dom"
import {FormStage} from "../types/types"

export default function Root() {
  const {state} = useStateMachine()
  const stage = state.data.stage

  return (
    <main className="container ml-auto mr-auto w-screen h-screen">
      <section className="py-4 flex flex-col">
        <h1 className="text-4xl font-bold mb-4">Stash Compilation Generator</h1>
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
