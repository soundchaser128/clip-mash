import {
  createStore,
  StateMachineProvider,
  useStateMachine,
} from "little-state-machine"
import React, {useEffect, useState} from "react"
import ReactDOM from "react-dom/client"
import {
  createBrowserRouter,
  isRouteErrorResponse,
  Outlet,
  RouterProvider,
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
import Music from "./routes/music/MusicPage"
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
import {resetForm} from "./routes/actions"
import AddVideosPage from "./routes/library/add"
import useNotification from "./hooks/useNotification"
import {FormStage} from "./types/form-state"
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
import DownloadMusic from "./routes/music/DownloadMusic"
import UploadMusic from "./routes/music/UploadMusic"
import ReorderSongs from "./routes/music/ReorderSongs"
import {ToastProvider} from "./hooks/useToast"

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

const RootElement = () => {
  const [updateDeclined, setUpdateDeclined] = useSessionStorage(
    "updateDeclined",
    false
  )
  const navigate = useNavigate()
  const [updating, setUpdating] = useState(false)
  useNotification()

  useEffect(() => {
    if (Notification.permission === "default") {
      Notification.requestPermission().then((permission) => {
        if (permission === "granted") {
          new Notification("Notifications enabled.", {
            icon: "/android-chrome-192x192.png",
          })
        }
      })
    }
  }, [])

  const onSelfUpdate = async () => {
    setUpdating(true)
    await fetch("/api/self/update", {method: "POST"})
    setUpdating(false)
    window.location.reload()
  }

  const onUpdateDecliend = () => {
    setUpdateDeclined(true)
    navigate("/")
  }

  const data = useLoaderData() as AppVersion
  if (data.needsUpdate && !updateDeclined) {
    return (
      <Layout>
        <div className="flex items-center flex-col">
          <h1 className="mt-8 text-3xl mb-6 font-bold">
            There is a new version of ClipMash available!
          </h1>
          <p className="mb-4 text-lg">
            Update to version <strong>{data.newVersion}</strong> now?
          </p>
          <div className="join">
            <button
              onClick={onSelfUpdate}
              className="btn join-item btn-success w-52"
            >
              <HiCheck className="mr-2" />
              Yes!
            </button>
            <button
              onClick={onUpdateDecliend}
              className="btn join-item btn-error w-52"
            >
              <HiXMark className="mr-2" />
              No, maybe later.
            </button>
          </div>
        </div>
      </Layout>
    )
  } else if (updating) {
    return (
      <div className="flex flex-col h-screen items-center justify-center w-full">
        <p className="text-xl mb-2">Updating...</p>
        <span className="loading loading-ring w-16"></span>
      </div>
    )
  } else {
    return <Outlet />
  }
}

const router = createBrowserRouter([
  {
    path: "/",
    errorElement: <ErrorBoundary />,
    element: <RootElement />,
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
            loader: makeVideoLoader({}),
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
          <ToastProvider>
            <RouterProvider router={router} />
          </ToastProvider>
        </ConfigProvider>
      </DndProvider>
    </StateMachineProvider>
  </React.StrictMode>,
)
