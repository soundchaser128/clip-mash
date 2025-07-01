import {json} from "react-router-dom"

type Method = "get" | "post" | "put" | "delete" | "patch"

export const BASE_URL = "http://localhost:5174"

interface Params {
  url: string
  method: Uppercase<Method> | Lowercase<Method>
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  params?: Record<string, any>
  data?: unknown
  responseType?: string
  headers?: Record<string, string>
}

export const customInstance = async <T>({
  url,
  method,
  params,
  data,
  headers,
}: Params): Promise<T> => {
  let fullUrl = BASE_URL + url
  if (params) {
    const filtered = Object.entries(params).filter(
      ([, value]) => value !== "" && value !== null && value !== undefined,
    )
    const search = new URLSearchParams(filtered)

    fullUrl += "?" + search.toString()
  }

  let requestBody = undefined

  if (data instanceof FormData) {
    requestBody = data
    // bug: multipart/form-data doesn't seem to work with the backend
    if (headers) {
      delete headers["Content-Type"]
    }
  } else if (typeof data !== "undefined") {
    requestBody = JSON.stringify(data)
  }

  const response = await fetch(fullUrl, {
    method,
    body: requestBody,
    headers,
  })

  if (response.ok) {
    return response.json()
  } else {
    const text = await response.text()
    throw json(
      {error: text, request: url},
      {
        status: response.status,
        headers: response.headers,
        statusText: response.statusText,
      },
    )
  }
}

export default customInstance
