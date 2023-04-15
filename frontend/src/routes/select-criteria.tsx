import {useStateMachine} from "little-state-machine"
import {useMemo, useState} from "react"
import {json, useLoaderData, useNavigate} from "react-router-dom"
import useFuse from "../hooks/useFuse"
import {FormStage, Performer, Tag, Scene, SelectMode} from "../types/types"
import {updateForm} from "./actions"
import {HiChevronRight} from "react-icons/hi2"

interface Data {
  performers: Performer[]
  tags: Tag[]
  scenes: Scene[]
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
    console.error(error)
    throw json({error: error.toString()}, {status: 500})
  }
}

function TagIcon({className}: {className: string}) {
  return (
    <svg
      fill="none"
      stroke="currentColor"
      strokeWidth={1.5}
      viewBox="0 0 24 24"
      xmlns="http://www.w3.org/2000/svg"
      aria-hidden="true"
      className={className}
    >
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M9.568 3H5.25A2.25 2.25 0 003 5.25v4.318c0 .597.237 1.17.659 1.591l9.581 9.581c.699.699 1.78.872 2.607.33a18.095 18.095 0 005.223-5.223c.542-.827.369-1.908-.33-2.607L11.16 3.66A2.25 2.25 0 009.568 3z"
      />
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M6 6h.008v.008H6V6z"
      />
    </svg>
  )
}

function Scenes({
  scenes,
  selection,
  onCheckboxChange,
}: {
  scenes: Scene[]
  selection: string[]
  onCheckboxChange: (id: string, checked: boolean, name: string) => void
}) {
  return (
    <section className="grid grid-cols-4 gap-2 w-full">
      {scenes.map((scene) => (
        <article
          key={scene.id}
          className="card card-compact bg-base-100 shadow-xl"
        >
          <figure>
            <img
              src={scene.imageUrl}
              alt={scene.title}
              className="aspect-[16/9] object-cover object-top w-full"
            />
          </figure>
          <div className="card-body">
            <h2 className="card-title tooltip" data-tip={scene.title}>
              <span className="truncate">{scene.title}</span>
            </h2>
            <ul className="text-base">
              <li>{scene.performers.join(", ")}</li>
              <li>{scene.markerCount} markers</li>
              <li>
                <div className="tooltip" data-tip={scene.tags.join(", ")}>
                  <TagIcon className="w-4 h-4 mr-0.5 inline-block" />{" "}
                  {scene.tags.length}
                </div>
              </li>
            </ul>
            <div className="card-actions justify-end">
              <div className="form-control">
                <label className="label cursor-pointer">
                  <span className="label-text">Select</span>
                  <input
                    type="checkbox"
                    className="checkbox checkbox-primary ml-2"
                    checked={selection.includes(scene.id)}
                    onChange={(e) =>
                      onCheckboxChange(scene.id, e.target.checked, scene.title)
                    }
                  />
                </label>
              </div>
            </div>
          </div>
        </article>
      ))}
    </section>
  )
}

function Tags({
  tags,
  selection,
  onCheckboxChange,
}: {
  tags: Tag[]
  selection: string[]
  onCheckboxChange: (id: string, checked: boolean, name: string) => void
}) {
  return (
    <section className="grid grid-cols-6 gap-2 w-full">
      {tags.map((tag) => (
        <article key={tag.id} className="card bg-base-100 shadow-xl">
          <div className="card-body">
            <h2 className="card-title">{tag.name}</h2>
            <p>{tag.count} markers</p>
            <div className="card-actions justify-end">
              <div className="form-control">
                <label className="label cursor-pointer">
                  <span className="label-text">Select</span>
                  <input
                    type="checkbox"
                    className="checkbox checkbox-primary ml-2"
                    checked={selection.includes(tag.id)}
                    onChange={(e) =>
                      onCheckboxChange(tag.id, e.target.checked, tag.name)
                    }
                  />
                </label>
              </div>
            </div>
          </div>
        </article>
      ))}
    </section>
  )
}

function Performers({
  performers,
  selection,
  onCheckboxChange,
}: {
  performers: Performer[]
  selection: string[]
  onCheckboxChange: (id: string, checked: boolean, name: string) => void
}) {
  return (
    <section className="grid grid-cols-6 gap-2 w-full">
      {performers.map((performer) => (
        <article
          key={performer.id}
          className="card bg-base-100 shadow-xl card-compact"
        >
          <figure>
            <img
              src={performer.imageUrl}
              alt={performer.name}
              className="aspect-[2/3] object-cover object-top w-full"
            />
          </figure>
          <div className="card-body">
            <h2 className="card-title">{performer.name}</h2>
            <p>{performer.sceneCount} scenes</p>
            <div className="card-actions justify-end">
              <div className="form-control">
                <label className="label cursor-pointer">
                  <span className="label-text">Select</span>
                  <input
                    type="checkbox"
                    className="checkbox checkbox-primary ml-2"
                    checked={selection.includes(performer.id)}
                    onChange={(e) =>
                      onCheckboxChange(
                        performer.id,
                        e.target.checked,
                        performer.name
                      )
                    }
                  />
                </label>
              </div>
            </div>
          </div>
        </article>
      ))}
    </section>
  )
}

interface SearchItem {
  id: string
  tokens: string[]
}

function getSearchItems(data: Data, mode: SelectMode): SearchItem[] {
  switch (mode) {
    case "performers":
      return data.performers.map((p) => ({id: p.id, tokens: [p.name]}))
    case "scenes":
      return data.scenes.map((s) => ({
        id: s.id,
        tokens: [s.title, ...s.tags, ...s.performers],
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
      {state.data.selectMode === "performers" && (
        <Performers
          performers={results as Performer[]}
          onCheckboxChange={onCheckboxChange}
          selection={selection}
        />
      )}

      {state.data.selectMode === "tags" && (
        <Tags
          tags={results as Tag[]}
          onCheckboxChange={onCheckboxChange}
          selection={selection}
        />
      )}

      {state.data.selectMode === "scenes" && (
        <Scenes
          scenes={results as Scene[]}
          onCheckboxChange={onCheckboxChange}
          selection={selection}
        />
      )}
    </div>
  )
}

export default SelectCriteria
