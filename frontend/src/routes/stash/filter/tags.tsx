import {LoaderFunction, useOutletContext} from "react-router-dom"
import {Context} from "./root"
import {Tag} from "../../../types/types"
import useFilteredData from "../../../hooks/useFilteredData"

export const loader: LoaderFunction = async () => {
  const response = await fetch("/api/stash/tags")
  return await response.json()
}

function Tags() {
  const {onCheckboxChange, selection, query} = useOutletContext<Context>()
  const tags = useFilteredData<Tag>({
    query,
    keys: ["name"],
  })

  return (
    <section className="grid grid-cols-1 lg:grid-cols-6 gap-4 w-full">
      {tags.map((tag) => (
        <article
          key={tag.id}
          className="card card-compact bg-base-200 shadow-xl"
        >
          <div className="card-body">
            <h2 className="card-title">{tag.name}</h2>
            <p>
              <strong>{tag.markerCount}</strong> markers
            </p>
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

export default Tags
