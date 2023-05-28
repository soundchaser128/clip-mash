import {LoaderFunction, useLoaderData} from "react-router-dom"
import {StateHelpers} from "../types/types"
import invariant from "tiny-invariant"
import {getFormState} from "../helpers"
import {MarkerDto} from "../types.generated"
import MarkerPage from "../components/MarkerPage"

interface Data {
  markers: MarkerDto[]
}

export const loader: LoaderFunction = async () => {
  const state = getFormState()
  if (state) {
    invariant(StateHelpers.isStash(state))
    const params = new URLSearchParams()
    params.set("selectedIds", state.selectedIds!.join(","))
    params.set("mode", state.selectMode!)
    params.set("includeAll", state.includeAll ? "true" : "false")
    const url = `/api/stash/markers?${params.toString()}`
    const response = await fetch(url)
    const markers = await response.json()
    return {markers} satisfies Data
  } else {
    return null
  }
}

function SelectMarkers() {
  const data = useLoaderData() as Data
  return <MarkerPage data={data} withImages />
}

export default SelectMarkers
