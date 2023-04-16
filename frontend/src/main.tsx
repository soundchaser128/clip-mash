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
import SelectCriteria, {
  loader as criteriaLoader,
} from "./routes/select-criteria"
import {FormStage} from "./types/types"
import SelectMarkers, {loader as markerLoader} from "./routes/select-markers"
import VideoOptions from "./routes/video-options"
import Progress from "./routes/progress"
import {nanoid} from "nanoid"
import {loader as rootLoader} from "./routes/root"
import ConfigPage from "./routes/config"
import PreviewClips, {loader as clipLoader} from "./routes/clips"

const Layout: React.FC<PropsWithChildren> = ({children}) => {
  return (
    <div className={styles.root}>
      <main className={styles.main}>{children}</main>
      <Footer />
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
        <div className="self-center shrink mt-8">
          <h1 className="font-bold text-3xl mb-4">
            {is404 ? "404 - Page not found" : "Sorry, something went wrong."}
          </h1>
          {!is404 && (
            <div>
              <p>Status code {error.status}</p>
              {error.data.error && <p>{error.data.error}</p>}
              {error.data.request && (
                <p>
                  Request to <code>{error.data.request}</code> failed.
                </p>
              )}
            </div>
          )}
        </div>
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
        path: "/select-criteria",
        element: <SelectCriteria />,
        loader: criteriaLoader,
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
