import {MarkerCount, listMarkerTitles, listSongs} from "@/api"
import Heading from "@/components/Heading"
import clsx from "clsx"
import React, {useState} from "react"
import {HiRocketLaunch} from "react-icons/hi2"
import {LoaderFunction, useLoaderData, useNavigate} from "react-router-dom"

export const markerTitleLoader: LoaderFunction = async () => {
  const markerTitles = await listMarkerTitles({
    count: 1000,
  })

  return {
    markers: markerTitles,
  }
}

const TvStartPage: React.FC = () => {
  const navigate = useNavigate()
  const markers = useLoaderData() as {markers: MarkerCount[]}
  const [selection, setSelection] = useState<string[]>([])
  const [withMusic, setWithMusic] = useState<boolean>(false)

  const onSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    const query = new URLSearchParams()
    for (const title of selection) {
      query.append("query", title)
    }

    if (withMusic) {
      query.append("withMusic", "true")
    }

    navigate({
      pathname: "/tv/watch",
      search: query.toString(),
    })
  }

  const toggleSelected = (marker: MarkerCount) => {
    if (selection.includes(marker.title)) {
      setSelection(selection.filter((s) => s !== marker.title))
    } else {
      setSelection([...selection, marker.title])
    }
  }

  return (
    <main className="container pt-2 py-1 ml-auto mr-auto flex flex-col min-h-screen">
      <Heading className="text-center">ClipMash TV</Heading>
      <p className="text-lg mb-4 text-center">
        Click on marker titles to select them.
      </p>
      <form
        onSubmit={onSubmit}
        className="flex flex-col justify-center items-center max-w-xl self-center"
      >
        <ul className="flex gap-1 flex-wrap">
          {markers.markers.slice(0, 50).map((marker) => (
            <li key={marker.title}>
              <button
                type="button"
                className={clsx("btn btn-primary btn-sm", {
                  "btn-outline": !selection.includes(marker.title),
                })}
                onClick={() => toggleSelected(marker)}
              >
                {marker.title} ({marker.count})
              </button>
            </li>
          ))}
        </ul>

        <div className="form-control mt-4">
          <label className="label">
            <span className="label-text mr-2">With Music</span>

            <input
              type="checkbox"
              id="withMusic"
              className="toggle toggle-primary"
              checked={withMusic}
              onChange={(e) => setWithMusic(e.target.checked)}
            />
          </label>
        </div>

        <button
          disabled={!selection.length}
          type="submit"
          className="btn btn-success self-end"
        >
          <HiRocketLaunch className="mr-1" />
          Go
        </button>
      </form>
    </main>
  )
}

export default TvStartPage
