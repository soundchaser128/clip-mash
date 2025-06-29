import {
  HandyConnectedResponse,
  HandyPattern,
  generateRandomSeed,
  handyConnected,
  listMarkerTitles,
  listPerformers,
  listVideoTags,
  startHandy,
} from "@/api"
import Heading from "@/components/Heading"
import Modal from "@/components/Modal"
import {useConfig} from "@/hooks/useConfig"
import React, {useEffect, useState} from "react"
import {useForm} from "react-hook-form"
import {
  HiAdjustmentsHorizontal,
  HiArrowPath,
  HiCheck,
  HiChevronLeft,
  HiRocketLaunch,
  HiXMark,
} from "react-icons/hi2"
import {FaDice} from "react-icons/fa6"
import {
  Link,
  LoaderFunction,
  useLoaderData,
  useNavigate,
  useSearchParams,
} from "react-router-dom"
import HandySettings, {prepareSettings} from "./handy/HandySettings"
import {useCreateToast} from "@/hooks/useToast"

type Item = {
  title: string
  count: number
}

type LoaderData = {
  markers: Item[]
  performers: Item[]
  tags: Item[]
  handyStatus: HandyConnectedResponse
}

export const markerTitleLoader: LoaderFunction = async () => {
  const [markerTitles, performers, tags, handy] = await Promise.all([
    listMarkerTitles({count: 1000}),
    listPerformers(),
    listVideoTags(),
    handyConnected(),
  ])

  return {
    markers: markerTitles,
    performers: performers.filter((p) => p.count > 0),
    tags: tags.map((t) => ({title: t.tag, count: t.count})),
    handyStatus: handy,
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

function selectionToQuery(
  state: TvSettings,
  withHandy: boolean,
): URLSearchParams {
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

  if (withHandy) {
    query.append("handyEnabled", "true")
  }

  return query
}

// TODO
//  * make the list of items searchable?
//  * add a forth tab for search query
const TvStartPage: React.FC = () => {
  const navigate = useNavigate()
  const data = useLoaderData() as LoaderData
  const config = useConfig()
  const [queryParms] = useSearchParams()
  const {register, watch, handleSubmit, setValue, reset} = useForm<TvSettings>({
    defaultValues: queryToSelection(queryParms),
  })
  const [handySettingsOpen, setHandySettingsOpen] = useState(false)
  const [handySettings, setHandySettings] = useState<HandyPattern | null>(null)
  const state = watch()
  const toast = useCreateToast()
  const withHandy = Boolean(
    handySettings && config?.handy?.key && config.handy.enabled,
  )

  let items: Item[] = []
  switch (state.queryType) {
    case "performers":
      items = data.performers
      break
    case "videoTags":
      items = data.tags
      break
    case "markerTitles":
      items = data.markers
      break
  }

  const onSubmit = async (values: TvSettings) => {
    const query = selectionToQuery(values, withHandy)

    // meh
    if (handySettings && config?.handy?.key && config.handy.enabled) {
      try {
        await startHandy({
          pattern: prepareSettings(handySettings),
          key: config.handy.key,
        })
      } catch (e) {
        toast({
          message: "Failed to start Handy",
          type: "error",
        })
      }
    }

    navigate({
      pathname: "/tv/watch",
      search: query.toString(),
    })
  }

  const onHandySubmit = (values: HandyPattern) => {
    setHandySettings(values)
    setHandySettingsOpen(false)
  }

  const onGenerateSeed = async () => {
    const seed = await generateRandomSeed()
    setValue("seed", seed)
  }

  const onSelectRandom = () => {
    const randomItems = items
      .map((item) => item.title)
      .sort(() => Math.random() - 0.5)
      .slice(0, 5)
    setValue("query", randomItems)
  }

  useEffect(() => {
    navigate({search: selectionToQuery(state, withHandy).toString()})
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [JSON.stringify(state), withHandy])

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
          Select markers, performers or video tags and click on 🚀 Start to
          watch a compilation generated for you in the browser!
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
          {items.length === 0 && (
            <li className="my-4 text-center font-bold w-full">
              No items available
            </li>
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
          <div className="flex w-full justify-between items-center">
            {data.handyStatus.connected ? (
              <p className="text-success text-sm">
                <HiCheck className="inline-block" /> Handy connected.
              </p>
            ) : (
              <p className="text-error text-sm">
                <HiXMark className="inline-block" /> Handy not connected.
              </p>
            )}

            <button
              type="button"
              className="btn self-end btn-outline"
              onClick={() => setHandySettingsOpen((open) => !open)}
              disabled={!data.handyStatus.connected}
            >
              <HiAdjustmentsHorizontal />
              Set up Handy
            </button>
          </div>
        )}

        <div className="w-full grid grid-cols-3 gap-2 mb-4 mt-2">
          <button
            type="button"
            className="btn btn-outline btn-error self-center"
            onClick={() => reset()}
          >
            <HiXMark />
            Reset
          </button>

          <button
            disabled={!state.query.length}
            type="submit"
            className="btn btn-success btn-lg text-xl"
          >
            <HiRocketLaunch className="mr-1" />
            Start
          </button>

          <button
            type="button"
            className="btn btn-outline self-center"
            onClick={onSelectRandom}
          >
            <FaDice />
            I&apos;m feeling lucky
          </button>
        </div>
      </form>

      <Modal isOpen={handySettingsOpen} size="fluid">
        <HandySettings onSubmit={onHandySubmit} />
      </Modal>
    </main>
  )
}

export default TvStartPage
