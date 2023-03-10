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
])

createStore(
  {
    data: {
      stage: FormStage.SelectMode,
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
