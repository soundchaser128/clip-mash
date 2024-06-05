import {
  ListPerformerResponse,
  MarkerCount,
  listMarkerTitles,
  listPerformers,
} from "@/api"
import Heading from "@/components/Heading"
import clsx from "clsx"
import React, {useState} from "react"
import {HiChevronLeft, HiPlus, HiRocketLaunch} from "react-icons/hi2"
import {
  Link,
  LoaderFunction,
  useLoaderData,
  useNavigate,
} from "react-router-dom"

const INITIAL_COUNT = 25

type Item = {
  title: string
  count: number
}

type LoaderData = {
  markers: Item[]
  performers: Item[]
}

export const markerTitleLoader: LoaderFunction = async () => {
  const [markerTitles, performers] = await Promise.all([
    listMarkerTitles({count: 1000}),
    listPerformers(),
  ])

  return {
    markers: markerTitles,
    performers: performers.map((p) => ({
      title: p.performer,
      count: p.video_count,
    })),
  } satisfies LoaderData
}

const TvStartPage: React.FC = () => {
  const navigate = useNavigate()
  const data = useLoaderData() as LoaderData
  const [selection, setSelection] = useState<string[]>([])
  const [withMusic, setWithMusic] = useState<boolean>(false)
  const [showAll, setShowAll] = useState<boolean>(false)
  const [usePerformers, setUsePerformers] = useState<boolean>(false)
  const allItems = usePerformers ? data.performers : data.markers

  const items = showAll ? allItems : allItems.slice(0, INITIAL_COUNT)

  const onSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    const query = new URLSearchParams()
    for (const title of selection) {
      query.append("query", title)
    }
    query.append("queryType", usePerformers ? "performers" : "markerTitles")

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
    <main className="container pt-2 px-1 ml-auto mr-auto flex flex-col min-h-screen">
      <Link
        to="/"
        className="btn btn-primary btn-outline btn-square btn-sm absolute top-4 left-4 z-10"
      >
        <HiChevronLeft />
      </Link>

      <Heading className="text-center" spacing="tight">
        ClipMash TV
      </Heading>

      <p className="self-center mb-2">
        Select markers or performers and click on 🚀 Start to watch a
        compilation generated for you in the browser!
      </p>

      <div className="self-center form-control">
        <label className="cursor-pointer label flex gap-4">
          <span className="label-text">Marker titles</span>
          <input
            type="checkbox"
            className="toggle toggle-primary"
            checked={usePerformers}
            onChange={(e) => setUsePerformers(e.target.checked)}
          />
          <span className="label-text">Performers</span>
        </label>
      </div>

      <form
        onSubmit={onSubmit}
        className="flex flex-col justify-center items-center max-w-xl self-center"
      >
        <ul className="flex gap-1 flex-wrap justify-center">
          {items.map((item) => (
            <li key={item.title}>
              <button
                type="button"
                className={clsx("btn btn-neutral btn-sm", {
                  "btn-outline": !selection.includes(item.title),
                })}
                onClick={() => toggleSelected(item)}
              >
                {item.title} ({item.count})
              </button>
            </li>
          ))}
        </ul>

        {items.length > INITIAL_COUNT && (
          <button
            className="btn btn-outline btn-primary mt-2 btn-sm"
            onClick={() => setShowAll(!showAll)}
            type="button"
          >
            <HiPlus />
            {showAll ? "Show less" : "Show all"}
          </button>
        )}

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
          Start
        </button>
      </form>
    </main>
  )
}

export default TvStartPage
