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
  Link,
  Outlet,
  RouterProvider,
  ScrollRestoration,
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
import ListVideos, {loader as listVideosLoader} from "./routes/local/videos"
import EditVideoModal from "./routes/local/videos.$id"
import CreateLayout from "./routes/root"
import Layout from "./components/Layout"
import Music from "./routes/music"
import ConfigPage from "./routes/stash/config"
import {
  configLoader,
  clipsLoader,
  localMarkerLoader,
  newIdLoader,
  videoDetailsLoader,
  musicLoader,
  versionLoader,
} from "./routes/loaders"
import {DndProvider} from "react-dnd"
import {HTML5Backend} from "react-dnd-html5-backend"
import MarkersPage from "./routes/local/markers"
import {resetForm} from "./routes/actions"
import DownloadVideosPage from "./routes/local/download"
import useNotification from "./hooks/useNotification"
import FunscriptPage from "./routes/funscript"
import {HiRocketLaunch} from "react-icons/hi2"
import {FormStage} from "./types/form-state"

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

const NotificationPermission = () => {
  useNotification()

  return (
    <>
      <Outlet />
      <ScrollRestoration />
    </>
  )
}

const HomePage = () => {
  return (
    <Layout>
      <div className="hero">
        <div className="hero-content self-center">
          <div className="max-w-md flex flex-col">
            <img src="/logo.png" className="w-40 self-center" />
            <p className="mt-2 text-lg text-center opacity-60">
              ClipMash helps you create video compilations.
            </p>
            <div className="self-center btn-group">
              <Link
                className="btn btn-lg btn-primary w-52 mt-4"
                to="/create/library"
              >
                <HiRocketLaunch className="mr-2 w-6 h-6" />
                Start
              </Link>
            </div>
          </div>
        </div>
      </div>
    </Layout>
  )
}

const router = createBrowserRouter([
  {
    path: "/",
    errorElement: <ErrorBoundary />,
    element: <NotificationPermission />,
    id: "root",
    loader: versionLoader,
    children: [
      {
        index: true,
        element: <HomePage />,
      },
      {
        path: "/create",
        element: <CreateLayout />,
        children: [
          {
            path: "library",
            element: <ListVideos />,
            loader: listVideosLoader,
          },
        ],
      },
    ],
  },
])

createStore(
  {
    data: {
      stage: FormStage.Start,
    },
  },
  {
    name: "form-state",
  },
)

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <StateMachineProvider>
      <DndProvider backend={HTML5Backend}>
        <RouterProvider router={router} />
      </DndProvider>
    </StateMachineProvider>
  </React.StrictMode>,
)
