import {
  createStore,
  StateMachineProvider,
  useStateMachine,
} from "little-state-machine"
import React from "react"
import ReactDOM from "react-dom/client"
import {
  createBrowserRouter,
  isRouteErrorResponse,
  LoaderFunction,
  RouterProvider,
  useNavigate,
  useRouteError,
} from "react-router-dom"
import "./index.css"
import SelectCriteria from "./routes/stash/filter/root"
import SelectMarkers, {
  loader as markerLoader,
} from "./routes/stash/select-markers"
import VideoOptions from "./routes/video-options"
import Progress from "./routes/progress"
import PreviewClips from "./routes/clips"
import Performers, {
  loader as performerLoader,
} from "./routes/stash/filter/performers"
import Tags, {loader as tagsLoader} from "./routes/stash/filter/tags"
import Scenes, {loader as scenesLoader} from "./routes/stash/filter/scenes"
import SelectVideoPath from "./routes/local/path"
import SelectSource from "./routes"
import SelectMode from "./routes/select-mode"
import {nanoid} from "nanoid"
import ListVideos, {loader as listVideosLoader} from "./routes/local/videos"
import EditVideoModal from "./routes/local/videos.$id"
import StashRoot from "./routes/root"
import Layout from "./components/Layout"
import Music from "./routes/music"
import ConfigPage from "./routes/stash/config"
import {configLoader, clipsLoader, localMarkerLoader} from "./routes/loaders"
import {DndProvider} from "react-dnd"
import {HTML5Backend} from "react-dnd-html5-backend"
import {SongDto} from "./types.generated"
import MarkersPage from "./routes/local/markers"
import {resetForm} from "./routes/actions"
import DownloadVideosPage from "./routes/local/download"

const TroubleshootingInfo = () => {
  const {actions} = useStateMachine({resetForm})
  const navigate = useNavigate()

  const onReset = () => {
    actions.resetForm()
    navigate("/")
  }

  return (
    <div>
      <h2 className="text-xl mb-2 font-bold">What you can do</h2>
      <ul className="list-disc list-inside">
        <li>Refresh the page.</li>
        <li>
          <span className="link link-primary" onClick={onReset}>
            Reset the page state.
          </span>
        </li>
        <li>
          Open an issue{" "}
          <a
            className="link link-primary"
            href="https://github.com/soundchaser128/stash-compilation-maker/issues"
          >
            here
          </a>
          , describing what you did leading up to the error.
        </li>
      </ul>
    </div>
  )
}

const ErrorBoundary = () => {
  const error = useRouteError()
  console.error(error)

  if (isRouteErrorResponse(error)) {
    const is404 = error.status === 404

    return (
      <Layout>
        <div className="mt-8 flex flex-col">
          <h1 className="font-bold text-5xl mb-4 w-fit">
            {is404 ? "404 - Page not found" : "Sorry, something went wrong."}
          </h1>
          {!is404 && (
            <div className="bg-error text-error-content p-2 rounded-lg self-start mb-4">
              <p>
                Status code <strong>{error.status}</strong>
              </p>
              {error.data.error && <p>{error.data.error}</p>}
              {error.data.request && (
                <p>
                  Request to <code>{error.data.request}</code> failed.
                </p>
              )}
            </div>
          )}
        </div>
        <TroubleshootingInfo />
      </Layout>
    )
  }

  const errorJson = JSON.stringify(error, null, 2)
  const isUsefulJson = errorJson && errorJson !== "{}"
  const err = error as Error
  return (
    <Layout>
      <div className="mt-8 flex flex-col">
        <h1 className="font-bold text-5xl mb-4">
          Sorry, something went wrong.
        </h1>
        <div className="bg-error text-error-content p-2 rounded-lg self-start mb-4">
          <h2 className="font-bold">Error details:</h2>
          <div>
            {isUsefulJson && <pre>{errorJson}</pre>}
            {!isUsefulJson && (
              <p>
                <code>
                  {err.name}: {err.message}
                </code>
              </p>
            )}
          </div>
        </div>
        <TroubleshootingInfo />
      </div>
    </Layout>
  )
}

const musicLoader: LoaderFunction = async () => {
  const response = await fetch("/api/song")
  const data = (await response.json()) as SongDto[]
  return data
}

const router = createBrowserRouter([
  {
    path: "/",
    errorElement: <ErrorBoundary />,
    children: [
      {
        index: true,
        element: <SelectSource />,
      },
      {
        path: "/stash/config",
        element: <ConfigPage />,
      },
      {
        path: "local",
        element: <StashRoot />,
        children: [
          {
            path: "path",
            element: <SelectVideoPath />,
          },
          {
            path: "videos/download",
            element: <DownloadVideosPage />,
          },
          {
            path: "videos",
            element: <ListVideos />,
            loader: listVideosLoader,
            id: "video-list",
            children: [
              {
                path: ":id",
                element: <EditVideoModal />,
              },
            ],
          },
          {
            path: "markers",
            element: <MarkersPage />,
            loader: localMarkerLoader,
          },
          {
            path: "options",
            element: <VideoOptions />,
          },
        ],
      },
      {
        path: "stash",
        element: <StashRoot />,
        loader: configLoader,
        children: [
          {
            path: "mode",
            element: <SelectMode />,
          },
          {
            path: "filter",
            element: <SelectCriteria />,
            id: "select-root",
            children: [
              {
                path: "performers",
                element: <Performers />,
                loader: performerLoader,
              },
              {
                path: "tags",
                element: <Tags />,
                loader: tagsLoader,
              },
              {
                path: "scenes",
                element: <Scenes />,
                loader: scenesLoader,
              },
            ],
          },
          {
            path: "markers",
            element: <SelectMarkers />,
            loader: markerLoader,
          },
          {
            path: "clips",
            element: <PreviewClips />,
            loader: clipsLoader,
          },
          {
            path: "video-options",
            element: <VideoOptions />,
          },
          {
            path: "progress",
            element: <Progress />,
          },
          {
            path: "music",
            element: <Music />,
            loader: musicLoader,
          },
        ],
      },
    ],
  },
])

createStore(
  {
    data: {
      source: undefined,
      id: nanoid(8),
    },
  },
  {
    name: "form-state",
  }
)

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <StateMachineProvider>
      <DndProvider backend={HTML5Backend}>
        <RouterProvider router={router} />
      </DndProvider>
    </StateMachineProvider>
  </React.StrictMode>
)
