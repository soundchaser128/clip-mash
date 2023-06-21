import Fuse from "fuse.js"
import {useMemo} from "react"

export interface Options<T> {
  items: T[]
  keys: (keyof T & string)[] | string[]
  query?: string
  threshold?: number
}

function useFuse<T>({items, keys, query, threshold = 0.1}: Options<T>): T[] {
  const fuse = useMemo(() => {
    const fuse = new Fuse(items, {keys, threshold})
    return fuse
  }, [items, keys])

  if (query && query.trim()) {
    const result = fuse.search(query)
    return result.map((i) => i.item)
  } else {
    return items
  }
}

export default useFuse
