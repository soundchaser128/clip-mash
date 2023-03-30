import {useStateMachine} from "little-state-machine"
import {useState} from "react"
import {json, useLoaderData, useNavigate} from "react-router-dom"
import {FormStage, Performer, Tag, Scene} from "../types/types"
import {updateForm} from "./actions"

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

function filterData(data: Data, filter?: string): Data {
  if (!filter || filter.trim().length === 0) {
    return data
  } else {
    return {
      performers: data.performers.filter((p) =>
        p.name.toLowerCase().includes(filter.toLowerCase())
      ),
      tags: data.tags.filter((t) =>
        t.name.toLowerCase().includes(filter.toLowerCase())
      ),
      scenes: data.scenes.filter((s) =>
        s.title.toLowerCase().includes(filter.toLowerCase())
      ),
    }
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

function SelectCriteria() {
  const data = useLoaderData() as Data
  const [filter, setFilter] = useState("")
  const {tags, performers, scenes} = filterData(data, filter)
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
          </button>
        </div>
      )}
      {state.data.selectMode === "performers" && (
        <section className="grid grid-cols-4 gap-2 w-full">
          {performers.map((performer) => (
            <article key={performer.id} className="card bg-base-100 shadow-xl">
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
      )}

      {state.data.selectMode === "tags" && (
        <section className="grid grid-cols-4 gap-2 w-full">
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
      )}

      {state.data.selectMode === "scenes" && (
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
                          onCheckboxChange(
                            scene.id,
                            e.target.checked,
                            scene.title
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
      )}
    </div>
  )
}

export default SelectCriteria
