import {useStateMachine} from "little-state-machine"
import {useState} from "react"
import {useNavigate} from "react-router-dom"
import {FormStage} from "../types/types"
import {updateForm} from "./actions"

type Mode = "none" | "performers" | "tags"

function SelectMode() {
  const {actions, state} = useStateMachine({updateForm})
  const [selectedMode, setSelectedMode] = useState<Mode>(
    state.data.selectMode || "none"
  )
  const navigate = useNavigate()

  const onNextStage = () => {
    if (selectedMode !== "none") {
      actions.updateForm({
        stage: FormStage.SelectCriteria,
        selectMode: selectedMode,
      })
      navigate("/select-criteria")
    }
  }

  const onModeChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    setSelectedMode(e.target.value as Mode)
  }

  return (
    <section className="py-4 flex flex-col">
      <div className="flex flex-col items-start gap-4">
        <div className="flex w-full items-center justify-between">
          <span>You can filter markers either by performers or by tags.</span>
          <button
            onClick={onNextStage}
            className="btn btn-success"
            disabled={selectedMode === "none"}
          >
            Next
          </button>
        </div>
        <div className="w-full flex justify-between">
          <select
            onChange={onModeChange}
            value={selectedMode}
            className="select select-primary"
          >
            <option disabled value="none">
              Select query type...
            </option>
            <option value="tags">Tags</option>
            <option value="performers">Performers</option>
          </select>
        </div>
      </div>
    </section>
  )
}

export default SelectMode
