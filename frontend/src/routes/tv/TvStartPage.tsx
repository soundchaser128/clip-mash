import {MarkerCount, listMarkerTitles} from "@/api"
import clsx from "clsx"
import React, {useState} from "react"
import {HiRocketLaunch} from "react-icons/hi2"
import {LoaderFunction, useLoaderData, useNavigate} from "react-router-dom"
import {s} from "vite/dist/node/types.d-aGj9QkWt"

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

  const onSubmit = () => {
    const query = new URLSearchParams()
    for (const title of selection) {
      query.append("query", title)
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
      <h1 className="text-4xl font-bold mb-1 text-center">ClipMash TV</h1>
      <p className="text-lg mb-4 text-center">
        Click on marker titles to select them.
      </p>
      <div className="flex flex-col justify-center items-center max-w-xl self-center">
        <ul className="flex gap-1 flex-wrap">
          {markers.markers.slice(0, 50).map((marker) => (
            <li key={marker.title}>
              <button
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

        <button
          disabled={!selection.length}
          onClick={onSubmit}
          className="btn btn-success self-end"
        >
          <HiRocketLaunch className="mr-1" />
          Go
        </button>
      </div>
    </main>
  )
}

export default TvStartPage
