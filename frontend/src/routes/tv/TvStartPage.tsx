import {listMarkerTitles, listPerformers} from "@/api"
import Heading from "@/components/Heading"
import clsx from "clsx"
import React from "react"
import {HiChevronLeft, HiPlus, HiRocketLaunch} from "react-icons/hi2"
import {
  Link,
  LoaderFunction,
  useLoaderData,
  useNavigate,
  useSearchParams,
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

interface FilterState {
  query: string[]
  queryType: "markerTitles" | "performers"
  withMusic: boolean
  showAll: boolean
}

function queryToSelection(query: URLSearchParams): FilterState {
  const queryType = query.get("queryType") as
    | "markerTitles"
    | "performers"
    | null
  const withMusic = query.has("withMusic")
  const queryValues = query.getAll("query")
  const showAll = query.has("showAll")

  return {
    query: queryValues,
    queryType: queryType ?? "markerTitles",
    withMusic,
    showAll,
  }
}

function selectionToQuery(state: FilterState): URLSearchParams {
  const query = new URLSearchParams()
  for (const title of state.query) {
    query.append("query", title)
  }
  query.append("queryType", state.queryType)

  if (state.withMusic) {
    query.append("withMusic", "true")
  }

  if (state.showAll) {
    query.append("showAll", "true")
  }

  return query
}

// TODO store the state in the URL
const TvStartPage: React.FC = () => {
  const navigate = useNavigate()
  const data = useLoaderData() as LoaderData
  const [queryParms] = useSearchParams()
  const state = queryToSelection(queryParms)

  const allItems =
    state.queryType === "performers" ? data.performers : data.markers
  const items = state.showAll ? allItems : allItems.slice(0, INITIAL_COUNT)

  const onChange = (update: Partial<FilterState>) => {
    const query = selectionToQuery({
      ...state,
      ...update,
    })
    navigate({
      search: query.toString(),
    })
  }

  const toggleSelected = (item: Item) => {
    if (state.query.includes(item.title)) {
      onChange({
        query: state.query.filter((title) => title !== item.title),
      })
    } else {
      onChange({
        query: [...state.query, item.title],
      })
    }
  }

  const onSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()

    const query = selectionToQuery(state)

    navigate({
      pathname: "/tv/watch",
      search: query.toString(),
    })
  }

  return (
    <main className="container pt-2 px-1 ml-auto mr-auto flex flex-col min-h-screen">
      <Link
        to="/"
        className="btn btn-primary btn-outline btn-square btn-sm absolute top-4 left-4 z-10"
      >
        <HiChevronLeft />
      </Link>

      <Heading className="text-center mt-4" spacing="tight">
        ClipMash TV
      </Heading>

      <form onSubmit={onSubmit} className="flex flex-col max-w-xl self-center">
        <p className="mb-2">
          Select markers or performers and click on 🚀 Start to watch a
          compilation generated for you in the browser!
        </p>

        <div className="form-control self-center">
          <label className="cursor-pointer label flex gap-4">
            <span className="label-text">Marker titles</span>
            <input
              type="checkbox"
              className="toggle toggle-primary"
              checked={state.queryType === "performers"}
              onChange={(e) =>
                onChange({
                  queryType: e.target.checked ? "performers" : "markerTitles",
                })
              }
            />
            <span className="label-text">Performers</span>
          </label>
        </div>

        <ul className="flex gap-1 flex-wrap">
          {items.map((item) => (
            <li key={item.title}>
              <button
                type="button"
                className={clsx("btn btn-neutral btn-sm", {
                  "btn-outline": !state.query.includes(item.title),
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
            onClick={() => onChange({showAll: !state.showAll})}
            type="button"
          >
            <HiPlus />
            {state.showAll ? "Show less" : "Show all"}
          </button>
        )}

        <div className="form-control self-stretch mt-2">
          <label className="label">
            <span className="label-text mr-2">With Music</span>

            <input
              type="checkbox"
              id="withMusic"
              className="toggle toggle-primary"
              checked={state.withMusic}
              onChange={(e) => onChange({withMusic: e.target.checked})}
            />
          </label>
        </div>

        <button
          disabled={!state.query.length}
          type="submit"
          className="btn btn-success self-center mt-2 btn-lg text-xl w-64"
        >
          <HiRocketLaunch className="mr-1" />
          Start
        </button>
      </form>
    </main>
  )
}

export default TvStartPage
