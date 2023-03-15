import {useStateMachine} from "little-state-machine"
import {useNavigate} from "react-router-dom"
import {FormStage} from "../types/types"
import {updateForm} from "./actions"

function SelectMode() {
  const {actions} = useStateMachine({updateForm})
  const navigate = useNavigate()

  const onNextStage = (mode: "performers" | "tags") => {
    actions.updateForm({
      stage: FormStage.SelectCriteria,
      selectMode: mode,
    })
    navigate("/select-criteria")
  }

  return (
    <section className="py-4 flex flex-col">
      <div className="flex flex-col items-start gap-4">
        <p className="text-center w-full">
          Choose how to filter markers: You can filter either by performers or
          by tags.
        </p>

        <div className="self-center flex gap-2">
          <button
            onClick={() => onNextStage("performers")}
            className="btn btn-lg btn-secondary w-48"
          >
            Performers
          </button>

          <button
            className="btn btn-lg btn-secondary w-48"
            onClick={() => onNextStage("tags")}
          >
            Tags
          </button>
        </div>
      </div>
    </section>
  )
}

export default SelectMode
