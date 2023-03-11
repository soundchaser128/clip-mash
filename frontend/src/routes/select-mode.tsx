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
        <div className="flex w-full items-center justify-between">
          <span>You can filter markers either by performers or by tags.</span>
        </div>
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
