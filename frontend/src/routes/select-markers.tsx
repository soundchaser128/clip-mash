import {LoaderFunction, useLoaderData} from "react-router-dom"
import {FormState} from "../types/types"

interface Marker {
  id: string
  primaryTag: string
  streamUrl: string
  screenshotUrl: string
  start: number
  end?: number
  sceneTitle: string
}

interface Data {
  markers: Marker[]
}

export const loader: LoaderFunction = async () => {
  const json = sessionStorage.getItem("form-state")
  if (json) {
    const state: {data: FormState} = JSON.parse(json)
    const params = new URLSearchParams()
    params.set("selectedIds", state.data.selectedIds!.join(","))
    params.set("mode", state.data.selectMode!)
    const url = `/api/markers?${params.toString()}`
    const response = await fetch(url)
    const markers = await response.json()
    return {markers} satisfies Data
  } else {
    return null
  }
}

function SelectMarkers() {
  const data = useLoaderData() as Data

  return (
    <div>
      <section className="grid grid-cols-4 gap-2 w-full">
        {data.markers.map((marker) => (
          <article key={marker.id} className="card bg-base-100 shadow-xl">
            <figure>
              <img
                src={marker.screenshotUrl}
                alt={marker.primaryTag}
                className="aspect-[2/3] object-cover object-top w-full"
              />
            </figure>
            <div className="card-body">
              <h2 className="card-title">{marker.primaryTag}</h2>
              <p>{marker.sceneTitle}</p>
            </div>
          </article>
        ))}
      </section>
    </div>
  )
}

export default SelectMarkers
