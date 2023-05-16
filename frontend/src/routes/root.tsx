import {LoaderFunction, json} from "react-router-dom"

export const loader: LoaderFunction = async () => {
  const response = await fetch("/api/stash/config")
  if (response.ok) {
    const config = await response.json()
    return config
  } else {
    const error = await response.text()
    throw json({error, request: "/api/stash/config"}, {status: 500})
  }
}
