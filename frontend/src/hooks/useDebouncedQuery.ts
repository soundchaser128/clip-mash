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
  const [, setParams] = useSearchParams()

  const setQuery = (value: string) => {
    setParams({[parameterName]: value})
  }

  return useCallback(debounce(setQuery, wait), [])
}

export default useDebouncedSetQuery
