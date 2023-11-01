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

  const addOrReplaceParam = (
    key: string,
    value: string | boolean | undefined,
  ) => {
    addOrReplaceParams([[key, value]])
  }

  const addOrReplaceParams = (
    pairs: [string, string | boolean | undefined][],
  ) => {
    const qs = new URLSearchParams(params)
    for (const [key, value] of pairs) {
      switch (typeof value) {
        case "boolean":
          if (value) {
            qs.set(key, value.toString())
          } else {
            qs.delete(key)
          }
          break
        case "string":
          qs.set(key, value)
          break
        case "undefined":
          qs.delete(key)
      }
    }
    setParams(qs)
  }

  const debounced = useCallback(debounce(setQuery, wait), [])

  return {
    setQueryDebounced: debounced,
    addOrReplaceParam,
    addOrReplaceParams,
  }
}

export default useDebouncedSetQuery
