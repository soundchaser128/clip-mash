import clsx from "clsx"
import {HiOutlineHeart, HiVideoCamera, HiStar, HiTag} from "react-icons/hi2"
import Rating from "../../../components/Rating"
import {Context} from "./root"
import {LoaderFunction, useOutletContext} from "react-router-dom"
import {Performer} from "../../../types/types"
import useFilteredData from "../../../hooks/useFilteredData"

export const loader: LoaderFunction = async () => {
  const response = await fetch("/api/performers")
  return await response.json()
}

function Performers() {
  const {onCheckboxChange, selection, query} = useOutletContext<Context>()
  const performers = useFilteredData<Performer>({
    query,
    keys: ["name", "tags"],
  })

  return (
    <section className="grid grid-cols-1 lg:grid-cols-4 gap-2 w-full">
      {performers.map((performer) => (
        <article
          key={performer.id}
          className="card card-compact bg-base-100 shadow-xl"
        >
          <figure className="relative">
            <HiOutlineHeart
              className={clsx(
                "w-12 h-12 absolute bottom-2 right-2 text-red-600",
                performer.favorite && "fill-red-600"
              )}
            />
            <img
              src={performer.imageUrl}
              alt={performer.name}
              className="aspect-[2/3] object-cover object-top w-full"
            />
          </figure>
          <div className="card-body">
            <h2 className="card-title">{performer.name}</h2>
            <ul className="text-base flex flex-col gap-2 grow">
              <li>
                <HiVideoCamera className="inline mr-2" />
                <strong>{performer.sceneCount}</strong> scenes
              </li>
              {performer.rating && (
                <li className="flex items-center">
                  <HiStar className="inline mr-2" />
                  Rating
                  <Rating
                    className="ml-1"
                    maxRating={5}
                    rating={performer.rating / 20}
                  />
                </li>
              )}
              <li>
                {performer.tags.length > 0 && (
                  <div className="inline-flex flex-wrap gap-x-1.5 gap-y-1.5">
                    {performer.tags.map((tag) => (
                      <span
                        className="bg-gray-200 px-1.5 py-0.5 rounded-lg"
                        key={tag}
                      >
                        <HiTag className="inline mr-2" />
                        {tag}
                      </span>
                    ))}
                  </div>
                )}
              </li>
            </ul>
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

export default Performers
