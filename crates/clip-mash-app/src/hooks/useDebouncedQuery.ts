import debounce from "lodash.debounce"
import {useCallback} from "react"
import {useSearchParams} from "react-router-dom"

interface Options {
  wait?: number
  parameterName?: string
  replaceAll?: boolean
}

export type QueryValue = string | boolean | undefined
export type QueryPair = [string, QueryValue]
export type QueryPairs = QueryPair[]

function useDebouncedSetQuery({
  wait = 500,
  parameterName = "query",
  replaceAll = true,
}: Options = {}) {
  const [params, setParams] = useSearchParams()

  const setQuery = (value: string) => {
    if (value.trim().length > 0) {
      if (replaceAll) {
        setParams({[parameterName]: value})
      } else {
        addOrReplaceParam(parameterName, value)
      }
    }
  }

  const addOrReplaceParam = (key: string, value: QueryValue) => {
    addOrReplaceParams([[key, value]])
  }

  const addOrReplaceParams = (pairs: QueryPairs) => {
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
          if (value.trim().length > 0) {
            qs.set(key, value)
          }
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
