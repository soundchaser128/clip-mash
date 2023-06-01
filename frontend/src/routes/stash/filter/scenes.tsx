import {
  HiUser,
  HiCamera,
  HiVideoCamera,
  HiTag,
  HiAdjustmentsVertical,
  HiCheck,
  HiXMark,
  HiStar,
} from "react-icons/hi2"
import Rating from "../../../components/Rating"
import {LoaderFunction, useOutletContext} from "react-router-dom"
import {Context} from "./root"
import useFilteredData from "../../../hooks/useFilteredData"
import {StashScene} from "../../../types.generated"

export const loader: LoaderFunction = async () => {
  const response = await fetch("/api/stash/scenes")
  return await response.json()
}

function Scenes() {
  const {onCheckboxChange, selection, query} = useOutletContext<Context>()
  const scenes = useFilteredData<StashScene>({
    query,
    keys: ["interactive", "performers", "tags", "title"],
  })

  return (
    <section className="grid grid-cols-1 lg:grid-cols-4 gap-2 w-full">
      {scenes.map((scene) => (
        <article
          key={scene.id}
          className="card card-compact bg-base-100 shadow-xl"
        >
          <figure>
            <img
              src={scene.imageUrl || undefined}
              alt={scene.title}
              className="aspect-[16/9] object-cover object-top w-full"
            />
          </figure>
          <div className="card-body">
            <h2 className="card-title tooltip" data-tip={scene.title}>
              <span className="truncate">{scene.title}</span>
            </h2>
            <ul className="text-base grow flex flex-col items-start">
              <li className="tooltip" data-tip="Performers">
                <HiUser className="inline mr-2" />
                {scene.performers.join(", ")}
              </li>
              {scene.studio && (
                <li className="tooltip" data-tip="Studio">
                  <HiCamera className="inline mr-2" />
                  {scene.studio}
                </li>
              )}
              <li
                className="tooltip"
                data-tip="Number of scene markers in this scene"
              >
                <HiVideoCamera className="inline mr-2" />
                {scene.markerCount} marker(s)
              </li>
              <li>
                <div className="tooltip" data-tip={scene.tags.join(", ")}>
                  <HiTag className="inline mr-2" />
                  {scene.tags.length} tags
                </div>
              </li>
              <li
                className="tooltip"
                data-tip="Whether the scene has an associated .funscript file."
              >
                <HiAdjustmentsVertical className="inline mr-2" />
                Interactive:{" "}
                <strong>
                  {scene.interactive ? (
                    <HiCheck className="text-green-600 inline" />
                  ) : (
                    <HiXMark className="text-red-600 inline" />
                  )}
                </strong>
              </li>
              {scene.rating && (
                <li className="flex items-center">
                  <HiStar className="inline mr-2" />
                  Rating
                  <Rating
                    className="ml-1"
                    maxRating={5}
                    rating={scene.rating / 20}
                  />
                </li>
              )}
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

export default Scenes
