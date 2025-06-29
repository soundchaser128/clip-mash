import {Location, To} from "react-router-dom"

export function searchLink(location: Location, tag: string): To {
  const searchParams = new URLSearchParams(location.search)
  searchParams.set("query", tag)
  return {
    search: searchParams.toString(),
  }
}
