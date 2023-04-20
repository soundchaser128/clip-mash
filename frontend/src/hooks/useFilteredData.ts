import {useLoaderData} from "react-router-dom"
import useFuse, {Options} from "./useFuse"

type FilterOptions<T> = Omit<Options<T>, "items">

function useFilteredData<T>(options: FilterOptions<T>) {
  const loaderData = useLoaderData() as T[]
  const filtered = useFuse({
    items: loaderData,
    keys: options.keys,
    query: options.query,
  })

  return filtered
}

export default useFilteredData
