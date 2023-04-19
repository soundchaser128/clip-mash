import {useOutletContext} from "react-router-dom"
import {Context} from "./root"
import {Tag} from "../../types/types"

function Tags() {
  const {onCheckboxChange, selection, results} = useOutletContext<Context>()
  const tags = results as Tag[]

  return (
    <section className="grid grid-cols-1 lg:grid-cols-6 gap-2 w-full">
      {tags.map((tag) => (
        <article key={tag.id} className="card bg-base-100 shadow-xl">
          <div className="card-body">
            <h2 className="card-title">{tag.name}</h2>
            <p>
              <strong>{tag.count}</strong> markers
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
