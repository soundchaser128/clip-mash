import debounce from "lodash.debounce"
import {useCallback} from "react"
import {useSearchParams} from "react-router-dom"

interface Options {
  wait?: number
  parameterName?: string
}

function useDebouncedSetQuery({
  wait = 500,
  parameterName = "query",
}: Options = {}) {
  const [params, setParams] = useSearchParams()

  const setQuery = (value: string) => {
    setParams({[parameterName]: value})
  }

  const addOrReplaceParam = (key: string, value: string | undefined) => {
    const qs = new URLSearchParams(params)
    if (typeof value !== "undefined") {
      qs.set(key, value)
    } else {
      qs.delete(key)
    }
    setParams(qs)
  }

  const debounced = useCallback(debounce(setQuery, wait), [])

  return {
    setQueryDebounced: debounced,
    addOrReplaceParam,
  }
}

export default useDebouncedSetQuery
