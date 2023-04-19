import {useStateMachine} from "little-state-machine"
import {useNavigate} from "react-router-dom"
import {FormStage} from "../types/types"
import {updateForm} from "./actions"
import {HiFilm, HiTag, HiUser} from "react-icons/hi2"

function SelectMode() {
  const {actions} = useStateMachine({updateForm})
  const navigate = useNavigate()

  const onNextStage = (mode: "performers" | "tags" | "scenes") => {
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
          Choose how to filter markers: You can filter either by performers, by
          tags or by scenes.
        </p>

        <div className="self-center grid grid-cols-1 md:grid-cols-3 gap-2">
          <button
            onClick={() => onNextStage("performers")}
            className="btn btn-lg btn-secondary"
          >
            <HiUser className="mr-2 w-6 h-6" />
            Performers
          </button>

          <button
            className="btn btn-lg btn-secondary"
            onClick={() => onNextStage("tags")}
          >
            <HiTag className="mr-2 w-6 h-6" />
            Tags
          </button>

          <button
            className="btn btn-lg btn-secondary"
            onClick={() => onNextStage("scenes")}
          >
            <HiFilm className="mr-2 w-6 h-6" />
            Scenes
          </button>
        </div>
      </div>
    </section>
  )
}

export default SelectMode
