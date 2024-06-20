import {
  HandyPattern,
  generateRandomSeed,
  listMarkerTitles,
  listPerformers,
  startHandy,
} from "@/api"
import Heading from "@/components/Heading"
import Modal from "@/components/Modal"
import {useConfig} from "@/hooks/useConfig"
import React, {useEffect, useState} from "react"
import {useForm} from "react-hook-form"
import {HiArrowPath, HiChevronLeft, HiRocketLaunch} from "react-icons/hi2"
import {
  Link,
  LoaderFunction,
  useLoaderData,
  useNavigate,
  useSearchParams,
} from "react-router-dom"
import HandySettings, {prepareSettings} from "./handy/HandySettings"

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
    performers: performers.filter((p) => p.count > 0),
  } satisfies LoaderData
}

export type TvQueryType = "markerTitles" | "performers" | "videoTags"

export interface TvSettings {
  query: string[]
  queryType: TvQueryType
  withMusic: boolean
  seed: string
}

function queryToSelection(query: URLSearchParams): TvSettings {
  const queryType = query.get("queryType") as TvQueryType | null
  const withMusic = query.has("withMusic")
  const queryValues = query.getAll("query")
  const seed = query.get("seed") ?? ""

  return {
    query: queryValues,
    queryType: queryType ?? "markerTitles",
    withMusic,
    seed,
  }
}

function selectionToQuery(state: TvSettings): URLSearchParams {
  const query = new URLSearchParams()
  for (const title of state.query) {
    query.append("query", title)
  }
  query.append("queryType", state.queryType)

  if (state.withMusic) {
    query.append("withMusic", "true")
  }

  if (state.seed?.trim().length > 0) {
    query.append("seed", state.seed)
  }
  return query
}

// TODO
//  * make the list of items searchable?
//  * add a button to clear the form
//  * add a button to select all items
//  * add a button to select random items
const TvStartPage: React.FC = () => {
  const navigate = useNavigate()
  const data = useLoaderData() as LoaderData
  const config = useConfig()
  const [queryParms] = useSearchParams()
  const {register, watch, handleSubmit, setValue} = useForm<TvSettings>({
    defaultValues: queryToSelection(queryParms),
  })
  const [handySettingsOpen, setHandySettingsOpen] = useState(false)
  const [handySettings, setHandySettings] = useState<HandyPattern | null>(null)
  const state = watch()

  let items: Item[] = []
  switch (state.queryType) {
    case "performers":
      items = data.performers
      break
    case "videoTags":
      // items = []
      // TODO
      break
    case "markerTitles":
      items = data.markers
      break
  }

  const onSubmit = async (values: TvSettings) => {
    const query = selectionToQuery(values)

    if (handySettings && config?.handy?.key && config.handy.enabled) {
      await startHandy({
        pattern: prepareSettings(handySettings),
        key: config.handy.key,
      })
    }

    navigate({
      pathname: "/tv/watch",
      search: query.toString(),
    })
  }

  const onGenerateSeed = async () => {
    const seed = await generateRandomSeed()
    setValue("seed", seed)
  }

  useEffect(() => {
    navigate({search: selectionToQuery(state).toString()})
  }, [JSON.stringify(state)])

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

      <form
        onSubmit={handleSubmit(onSubmit)}
        className="flex flex-col max-w-xl self-center"
      >
        <p className="mb-2">
          Select markers or performers and click on 🚀 Start to watch a
          compilation generated for you in the browser!
        </p>

        <div role="tablist" className="tabs tabs-bordered mb-4">
          <input
            type="radio"
            role="tab"
            className="tab"
            aria-label="Marker titles"
            value="markerTitles"
            {...register("queryType")}
          />
          <input
            type="radio"
            role="tab"
            className="tab"
            aria-label="Performers"
            value="performers"
            {...register("queryType")}
          />

          <input
            type="radio"
            role="tab"
            className="tab"
            aria-label="Video tags"
            value="videoTags"
            {...register("queryType")}
          />
        </div>

        <ul className="flex gap-1 flex-wrap">
          {state.queryType === "videoTags" && (
            <p className="my-4 text-center w-full">Not implemented yet.</p>
          )}

          {items.map((item) => (
            <li key={item.title}>
              <input
                type="checkbox"
                className="btn btn-neutral btn-outline btn-sm"
                aria-label={`${item.title} (${item.count})`}
                value={item.title}
                {...register("query")}
              />
            </li>
          ))}
        </ul>

        <div className="form-control self-stretch mt-2">
          <label className="label">
            <span className="label-text mr-2">With Music</span>

            <input
              type="checkbox"
              id="withMusic"
              className="checkbox checkbox-primary"
              {...register("withMusic")}
            />
          </label>
        </div>

        <div className="form-control self-stretch mt-2">
          <label className="label">
            <span className="label-text mr-2">Seed</span>
            <div className="join">
              <input
                type="text"
                className="input input-sm input-bordered join-item"
                {...register("seed")}
              />
              <button
                type="button"
                className="btn btn-sm btn-square join-item"
                onClick={onGenerateSeed}
              >
                <HiArrowPath />
              </button>
            </div>
          </label>
        </div>
        {config?.handy?.enabled && (
          <>
            <button
              type="button"
              className="btn self-end"
              onClick={() => setHandySettingsOpen((open) => !open)}
            >
              Set up Handy
            </button>
            <Modal isOpen={handySettingsOpen}>
              <HandySettings onSubmit={setHandySettings} />
            </Modal>
          </>
        )}

        <button
          disabled={!state.query.length}
          type="submit"
          className="btn btn-success self-center mt-2 btn-lg text-xl w-64 mb-4"
        >
          <HiRocketLaunch className="mr-1" />
          Start
        </button>
      </form>
    </main>
  )
}

export default TvStartPage
