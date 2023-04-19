import {useStateMachine} from "little-state-machine"
import {useState} from "react"
import {HiChevronRight} from "react-icons/hi2"
import {useNavigate, Outlet} from "react-router-dom"
import {FormStage, Performer, Scene, SelectMode, Tag} from "../../types/types"
import {updateForm} from "../actions"

export interface Data {
  performers: Performer[]
  tags: Tag[]
  scenes: Scene[]
}

export interface Context {
  onCheckboxChange: (id: string, checked: boolean, name: string) => void
  selection: string[]
  query: string
}

export function getUrl(mode: SelectMode): string {
  switch (mode) {
    case "performers":
      return "/select/performers"
    case "scenes":
      return "/select/scenes"
    case "tags":
      return "/select/tags"
  }
}

function SelectCriteria() {
  const [filter, setFilter] = useState("")
  const {state, actions} = useStateMachine({updateForm})
  const [selection, setSelection] = useState<string[]>(
    state.data.selectedIds || []
  )
  const queryType = state.data.selectMode
  const navigate = useNavigate()
  const [fileNameComponents, setFileNameComponents] = useState<string[]>([])

  const onCheckboxChange = (id: string, checked: boolean, name: string) => {
    if (checked) {
      setSelection((s) => [...s, id])
      setFileNameComponents((s) => [...s, name])
    } else {
      setSelection((s) => s.filter((string) => string !== id))
      setFileNameComponents((s) => s.filter((string) => string !== name))
    }
  }

  const onNextStage = () => {
    const fileName = Array.from(new Set(fileNameComponents).values()).join(", ")
    actions.updateForm({
      stage: FormStage.SelectMarkers,
      selectedIds: selection,
      selectedMarkers: undefined,
      fileName: `${fileName} [${state.data.id}].mp4`,
    })
    navigate("/select-markers")
  }

  return (
    <div>
      {queryType && (
        <div className="w-full flex justify-between mb-4">
          <input
            type="text"
            placeholder="Filter..."
            className="input input-bordered w-96"
            value={filter}
            onChange={(e) => setFilter(e.target.value)}
          />
          <button
            type="button"
            onClick={onNextStage}
            className="btn btn-success"
            disabled={selection.length === 0}
          >
            Next
            <HiChevronRight className="ml-1" />
          </button>
        </div>
      )}

      <Outlet
        context={
          {
            onCheckboxChange,
            selection,
            query: filter,
          } satisfies Context
        }
      />
    </div>
  )
}

export default SelectCriteria
