import {createStore, StateMachineProvider} from "little-state-machine"
import React, {PropsWithChildren} from "react"
import ReactDOM from "react-dom/client"
import {
  createBrowserRouter,
  isRouteErrorResponse,
  RouterProvider,
  useRouteError,
} from "react-router-dom"
import "./index.css"
import SelectMode from "./routes/select-mode"
import Root, {Footer, styles} from "./routes/root"
import SelectCriteria from "./routes/select/root"
import {FormStage} from "./types/types"
import SelectMarkers, {loader as markerLoader} from "./routes/select-markers"
import VideoOptions from "./routes/video-options"
import Progress from "./routes/progress"
import {nanoid} from "nanoid"
import {loader as rootLoader} from "./routes/root"
import ConfigPage from "./routes/config"
import PreviewClips, {loader as clipLoader} from "./routes/clips"
import Performers, {loader as performerLoader} from "./routes/select/performers"
import Tags, {loader as tagsLoader} from "./routes/select/tags"
import Scenes, {loader as scenesLoader} from "./routes/select/scenes"

const Layout: React.FC<PropsWithChildren> = ({children}) => {
  return (
    <div className={styles.root}>
      <main className={styles.main}>{children}</main>
      <Footer />
    </div>
  )
}

const TroubleshootingInfo = () => {
  return (
    <div className="p-2">
      <p>Try refreshing the page.</p>
      <p>
        If that doesn't help, please open an issue{" "}
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
          <h1 className="font-bold text-3xl mb-4 w-fit">
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
        <h1 className="font-bold text-3xl mb-4">
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
    element: <Root />,
    loader: rootLoader,
    errorElement: <ErrorBoundary />,
    children: [
      {
        index: true,
        element: <SelectMode />,
      },
      {
        path: "select",
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
        path: "/select-markers",
        element: <SelectMarkers />,
        loader: markerLoader,
      },
      {
        path: "/clips",
        element: <PreviewClips />,
        loader: clipLoader,
      },
      {
        path: "/video-options",
        element: <VideoOptions />,
      },
      {
        path: "/progress",
        element: <Progress />,
      },
    ],
  },
  {
    path: "/config",
    element: <ConfigPage />,
    loader: rootLoader,
  },
])

createStore(
  {
    data: {
      stage: FormStage.SelectMode,
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
