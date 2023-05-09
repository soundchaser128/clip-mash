import {createStore, StateMachineProvider} from "little-state-machine"
import React from "react"
import ReactDOM from "react-dom/client"
import {
  createBrowserRouter,
  isRouteErrorResponse,
  RouterProvider,
  useRouteError,
} from "react-router-dom"
import "./index.css"
import SelectCriteria from "./routes/stash/filter/root"
import SelectMarkers, {loader as markerLoader} from "./routes/select-markers"
import VideoOptions from "./routes/video-options"
import Progress from "./routes/progress"
import PreviewClips, {loader as clipLoader} from "./routes/clips"
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
import StashRoot from "./routes/stash/root"
import Layout from "./components/Layout"

const TroubleshootingInfo = () => {
  return (
    <div className="p-2">
      <p>Try refreshing the page.</p>
      <p>
        If that doesn&apos;t help, please open an issue{" "}
        <a
          className="link link-primary"
          href="https://github.com/soundchaser128/stash-compilation-maker/issues"
        >
          here
        </a>
        , describing what you did leading up to the error.
      </p>
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
        <div className="mt-8">
          <h1 className="font-bold text-5xl mb-4 w-fit">
            {is404 ? "404 - Page not found" : "Sorry, something went wrong."}
          </h1>
          {!is404 && (
            <div className="bg-red-200 p-2 rounded-lg text-black">
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

  return (
    <Layout>
      <div className="self-center shrink mt-8">
        <h1 className="font-bold text-5xl mb-4">
          Sorry, something went wrong.
        </h1>
        <div>
          <pre>{JSON.stringify(error, null, 2)}</pre>
        </div>
      </div>
    </Layout>
  )
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
        path: "local",
        element: <StashRoot />,
        children: [
          {
            path: "path",
            element: <SelectVideoPath />,
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
            path: "options",
            element: <VideoOptions />,
          },
        ],
      },
      {
        path: "stash",
        element: <StashRoot />,
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
            loader: clipLoader,
          },
          {
            path: "video-options",
            element: <VideoOptions />,
          },
          {
            path: "progress",
            element: <Progress />,
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
      <RouterProvider router={router} />
    </StateMachineProvider>
  </React.StrictMode>
)
