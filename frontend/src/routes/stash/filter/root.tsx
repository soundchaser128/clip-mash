import {useStateMachine} from "little-state-machine"
import {useState} from "react"
import {HiChevronRight} from "react-icons/hi2"
import {useNavigate, Outlet} from "react-router-dom"
import {updateForm} from "../../actions"
import invariant from "tiny-invariant"
import {PerformerDto, StashScene, TagDto} from "../../../types/types.generated"
import {SelectMode} from "../../../types/types"
import {FormStage, StateHelpers} from "../../../types/form-state"

export interface Data {
  performers: PerformerDto[]
  tags: TagDto[]
  scenes: StashScene[]
}

export interface Context {
  onCheckboxChange: (id: string, checked: boolean, name: string) => void
  selection: string[]
  query: string
  includeAll: boolean
}

export function getUrl(mode: SelectMode): string {
  switch (mode) {
    case "performers":
      return "/stash/filter/performers"
    case "scenes":
      return "/stash/filter/scenes"
    case "tags":
      return "/stash/filter/tags"
  }
}

function SelectCriteria() {
  const [filter, setFilter] = useState("")
  const {state, actions} = useStateMachine({updateForm})
  invariant(StateHelpers.isStash(state.data))
  const [selection, setSelection] = useState<string[]>(
    state.data.selectedIds || [],
  )
  const queryType = state.data.selectMode
  const navigate = useNavigate()
  const [fileNameComponents, setFileNameComponents] = useState<string[]>([])
  const [includeAll, setIncludeAll] = useState(false)

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
      includeAll,
    })
    navigate("/stash/markers")
  }

  return (
    <div>
      {queryType && (
        <div className="w-full grid grid-cols-3 mb-4">
          <input
            type="text"
            placeholder="Filter..."
            className="input input-bordered w-96 justify-self-start"
            value={filter}
            onChange={(e) => setFilter(e.target.value)}
          />
          {state.data.selectMode !== "scenes" && (
            <div
              className="form-control justify-self-center tooltip"
              data-tip="Whether to include every marker that matches any of the criteria, or only markers that match all of them at once."
            >
              <label className="label cursor-pointer gap-4">
                <span className="label-text">Include any</span>
                <input
                  type="checkbox"
                  className="toggle"
                  checked={includeAll}
                  onChange={(e) => setIncludeAll(e.target.checked)}
                />
                <span className="label-text">Include all</span>
              </label>
            </div>
          )}
          {state.data.selectMode === "scenes" && <div />}

          <button
            type="button"
            onClick={onNextStage}
            className="btn btn-success justify-self-end"
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
            includeAll,
          } satisfies Context
        }
      />
    </div>
  )
}

export default SelectCriteria
