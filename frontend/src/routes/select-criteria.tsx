import {useStateMachine} from "little-state-machine"
import {useState} from "react"
import {useLoaderData, useNavigate} from "react-router-dom"
import {FormStage, Performer, Tag} from "../types/types"
import { updateForm } from "./actions"

interface Data {
  performers: Performer[]
  tags: Tag[]
}

async function fetchTags(): Promise<Tag[]> {
  const response = await fetch("/api/tags")
  return await response.json()
}

async function fetchPerformers(): Promise<Performer[]> {
  const response = await fetch("/api/performers")
  return await response.json()
}

export async function loader(): Promise<Data> {
  const [tags, performers] = await Promise.all([fetchTags(), fetchPerformers()])

  return {tags, performers}
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
    }
  }
}

function SelectCriteria() {
  const data = useLoaderData() as Data
  const [filter, setFilter] = useState("")
  const {tags, performers} = filterData(data, filter)
  const {state, actions} = useStateMachine({updateForm})
  const [selection, setSelection] = useState<string[]>(state.data.selectedIds || [])
  const queryType = state.data.selectMode
  const navigate = useNavigate()

  const onCheckboxChange = (id: string, checked: boolean) => {
    if (checked) {
      setSelection((s) => [...s, id])
    } else {
      setSelection((s) => s.filter((string) => string !== id))
    }
  }

  const onNextStage = () => {
    actions.updateForm({
      stage: FormStage.SelectMarkers,
      selectedIds: selection
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
                          onCheckboxChange(performer.id, e.target.checked)
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
                          onCheckboxChange(tag.id, e.target.checked)
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
