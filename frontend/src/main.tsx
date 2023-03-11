import {createStore, StateMachineProvider} from "little-state-machine"
import React from "react"
import ReactDOM from "react-dom/client"
import {createBrowserRouter, RouterProvider} from "react-router-dom"
import "./index.css"
import SelectMode from "./routes/select-mode"
import Root from "./routes/root"
import SelectCriteria, {
  loader as criteriaLoader,
} from "./routes/select-criteria"
import {FormStage} from "./types/types"
import SelectMarkers, {loader as markerLoader} from "./routes/select-markers"
import VideoOptions from "./routes/video-options"
import Progress from "./routes/progress"
import { nanoid } from "nanoid"

const router = createBrowserRouter([
  {
    path: "/",
    element: <Root />,
    children: [
      {
        index: true,
        element: <SelectMode />,
      },
    ],
  },
  {
    path: "/select-criteria",
    element: <Root />,
    children: [
      {
        index: true,
        element: <SelectCriteria />,
        loader: criteriaLoader,
      },
    ],
  },
  {
    path: "/select-markers",
    element: <Root />,
    children: [
      {
        index: true,
        element: <SelectMarkers />,
        loader: markerLoader,
      },
    ],
  },
  {
    path: "/video-options",
    element: <Root />,
    children: [
      {
        index: true,
        element: <VideoOptions />,
      },
    ],
  },
  {
    path: "/progress",
    element: <Root />,
    children: [
      {
        index: true,
        element: <Progress />,
      },
    ],
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
