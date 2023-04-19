import {useStateMachine} from "little-state-machine"
import {useState} from "react"
import {HiChevronRight} from "react-icons/hi2"
import {useLoaderData, useNavigate, Outlet, json} from "react-router-dom"
import useFuse from "../../hooks/useFuse"
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
  results: Performer[] | Scene[] | Tag[]
}

interface SearchItem {
  id: string
  tokens: string[]
}

async function fetchTags(): Promise<Tag[]> {
  const response = await fetch("/api/tags")
  return await response.json()
}

async function fetchPerformers(): Promise<Performer[]> {
  const response = await fetch("/api/performers")
  return await response.json()
}

async function fetchScenes(): Promise<Scene[]> {
  const response = await fetch("/api/scenes")
  return await response.json()
}

export async function loader(): Promise<Data> {
  try {
    const [tags, performers, scenes] = await Promise.all([
      fetchTags(),
      fetchPerformers(),
      fetchScenes(),
    ])
    return {tags, performers, scenes}
  } catch (e) {
    const error = e as Error
    throw json({error: error.toString(), request: "todo"}, {status: 500})
  }
}

function getSearchItems(data: Data, mode: SelectMode): SearchItem[] {
  switch (mode) {
    case "performers":
      return data.performers.map((p) => ({
        id: p.id,
        tokens: [p.name, ...p.tags],
      }))
    case "scenes":
      return data.scenes.map((s) => ({
        id: s.id,
        tokens: [
          s.title,
          ...s.tags,
          ...s.performers,
          s.interactive ? "interactive" : "non-interactive",
        ],
      }))
    case "tags":
      return data.tags.map((t) => ({id: t.id, tokens: [t.name]}))
  }
}

function getResults(
  data: Data,
  mode: SelectMode,
  ids: string[]
): Performer[] | Tag[] | Scene[] {
  switch (mode) {
    case "performers":
      return ids.map((id) => data.performers.find((p) => p.id === id)!)
    case "scenes":
      return ids.map((id) => data.scenes.find((p) => p.id === id)!)
    case "tags":
      return ids.map((id) => data.tags.find((p) => p.id === id)!)
  }
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
  const data = useLoaderData() as Data
  const [filter, setFilter] = useState("")
  const {state, actions} = useStateMachine({updateForm})
  const mode = state.data.selectMode!
  const searchItems = getSearchItems(data, mode)

  const ids = useFuse({
    items: searchItems,
    keys: ["tokens"],
    query: filter,
  }).map((r) => r.id)

  const results = getResults(data, mode, ids)

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
            className="input input-bordered"
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
            results,
          } satisfies Context
        }
      />
    </div>
  )
}

export default SelectCriteria
