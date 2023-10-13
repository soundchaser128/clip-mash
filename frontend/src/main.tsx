import {
  createStore,
  StateMachineProvider,
  useStateMachine,
} from "little-state-machine"
import React, {useEffect, useRef, useState} from "react"
import ReactDOM from "react-dom/client"
import {
  createBrowserRouter,
  isRouteErrorResponse,
  Link,
  Outlet,
  RouterProvider,
  ScrollRestoration,
  useLoaderData,
  useNavigate,
  useRouteError,
} from "react-router-dom"
import "inter-ui/inter.css"
import "./index.css"

import VideoOptions, {videoOptionsLoader} from "./routes/video-options"
import Progress from "./routes/progress"
import PreviewClips from "./routes/clips/clips"
import ListVideos from "./routes/library/videos"
import CreateLayout from "./routes/root"
import Layout from "./components/Layout"
import Music from "./routes/music"
import {
  clipsLoader,
  localMarkerLoader,
  newIdLoader,
  musicLoader,
  versionLoader,
  videoDetailsLoader,
  stashVideoLoader,
  makeVideoLoader,
} from "./routes/loaders"
import {DndProvider} from "react-dnd"
import {HTML5Backend} from "react-dnd-html5-backend"
import MarkersPage from "./routes/library/markers"
import {resetForm, updateForm} from "./routes/actions"
import AddVideosPage from "./routes/library/add"
import useNotification from "./hooks/useNotification"
import {HiFolder, HiRocketLaunch} from "react-icons/hi2"
import {FormStage, FormState} from "./types/form-state"
import EditVideoModal from "./routes/library/videos.$id"
import DownloadVideosPage from "./routes/library/add/download"
import SelectVideos from "./routes/library/add/folder"
import AddStashVideoPage from "./routes/library/add/stash"
import {ConfigProvider} from "./hooks/useConfig"
import StashConfigPage from "./routes/stash-config"
import FunscriptPage from "./routes/funscript"
import DownloadVideoPage from "./routes/download-video"
import SelectVideosPage from "./routes/library/select-videos"
import HomePage from "./routes/home"

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
            path: "stash/config",
            element: <StashConfigPage />,
          },
          {
            path: "library",
            element: <ListVideos />,
            loader: makeVideoLoader(false),
            children: [
              {
                path: ":id/markers",
                element: <EditVideoModal />,
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
            loader: makeVideoLoader(true),
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
        <ConfigProvider>
          <RouterProvider router={router} />
        </ConfigProvider>
      </DndProvider>
    </StateMachineProvider>
  </React.StrictMode>,
)
