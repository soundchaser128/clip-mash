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
  Outlet,
  RouterProvider,
  ScrollRestoration,
  useNavigate,
  useRouteError,
} from "react-router-dom"
import "inter-ui/inter.css"
import "./index.css"

import VideoOptions, {videoOptionsLoader} from "./routes/VideoOptions"
import Progress from "./routes/CreateVideoPage"
import PreviewClips from "./routes/clips/ClipPreviewPage"
import ListVideos from "./routes/library/ListVideos"
import CreateLayout from "./routes/Layout"
import Layout from "./components/Layout"
import Music from "./routes/music/MusicPage"
import {
  clipsLoader,
  localMarkerLoader,
  newIdLoader,
  musicLoader,
  versionLoader,
  stashVideoLoader,
  makeVideoLoader,
  videoDetailsLoader,
} from "./routes/loaders"
import {DndProvider} from "react-dnd"
import {HTML5Backend} from "react-dnd-html5-backend"
import MarkersPage from "./routes/library/SelectMarkers"
import {resetForm} from "./routes/actions"
import AddVideosPage from "./routes/library/add/AddVideosPaage"
import useNotification from "./hooks/useNotification"
import {FormStage} from "./types/form-state"
import DownloadVideosPage from "./routes/library/add/DownloadVideosPage"
import SelectVideos from "./routes/library/add/AddLocalVideosPage"
import AddStashVideoPage from "./routes/library/add/AddStashVideoPage"
import {ConfigProvider} from "./hooks/useConfig"
import FunscriptPage from "./routes/FunscriptPage"
import DownloadVideoPage from "./routes/DownloadFinishedVideo"
import SelectVideosPage from "./routes/library/SelectVideos"
import DownloadMusic from "./routes/music/DownloadMusic"
import UploadMusic from "./routes/music/UploadMusic"
import ReorderSongs from "./routes/music/ReorderSongs"
import {ToastProvider} from "./hooks/useToast"
import HomePage from "./routes/HomePage"
import MarkerModal from "./routes/library/MarkerModal"
import Sentry from "./sentry"
import SentryDebug from "./routes/SentryDebug"
import AppSettingsPage from "./routes/AppSettings"

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
        <li>
          <span
            className="link link-primary"
            onClick={() => window.location.reload()}
          >
            Reload the page.
          </span>
        </li>
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

const Init = () => {
  useNotification()

  return (
    <>
      <Outlet />
      <ScrollRestoration />
    </>
  )
}

const router = createBrowserRouter([
  {
    path: "/",
    errorElement: <ErrorBoundary />,
    element: <Init />,
    id: "root",
    loader: versionLoader,
    children: [
      {
        index: true,
        element: <HomePage />,
        loader: newIdLoader,
      },
      {
        element: <CreateLayout />,
        children: [
          {
            path: "/settings",
            element: <AppSettingsPage />,
          },

          {
            path: "library",
            element: <ListVideos />,
            loader: makeVideoLoader({}),
            children: [
              {
                path: ":id/markers",
                element: <MarkerModal />,
                loader: videoDetailsLoader,
              },
            ],
          },
          {
            path: "library/add",
            element: <AddVideosPage />,
          },
          {path: "library/add/download", element: <DownloadVideosPage />},
          {path: "library/add/folder", element: <SelectVideos />},
          {
            path: "library/add/stash",
            element: <AddStashVideoPage />,
            loader: stashVideoLoader,
          },
          {
            path: "/library/select",
            element: <SelectVideosPage />,
            loader: makeVideoLoader({hasMarkers: true}),
          },
          {
            path: "markers",
            element: <MarkersPage />,
            loader: localMarkerLoader,
          },
          {
            path: "music",
            element: <Music />,
            loader: musicLoader,
          },
          {
            path: "music/download",
            element: <DownloadMusic />,
          },
          {
            path: "music/upload",
            element: <UploadMusic />,
          },
          {
            path: "music/reorder",
            element: <ReorderSongs />,
          },
          {
            path: "video-options",
            element: <VideoOptions />,
            loader: videoOptionsLoader,
          },
          {
            path: "clips",
            element: <PreviewClips />,
            loader: clipsLoader,
          },
          {
            path: "generate",
            element: <Progress />,
          },
          {
            path: ":id/download",
            element: <DownloadVideoPage />,
          },
          {
            path: ":id/funscript",
            element: <FunscriptPage />,
          },
          {
            path: "/sentry-debug",
            element: <SentryDebug />,
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

Sentry.setup()

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <StateMachineProvider>
      <DndProvider backend={HTML5Backend}>
        <ConfigProvider>
          <ToastProvider>
            <RouterProvider router={router} />
          </ToastProvider>
        </ConfigProvider>
      </DndProvider>
    </StateMachineProvider>
  </React.StrictMode>,
)
