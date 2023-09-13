import {json} from "react-router-dom"

interface Params {
  url: string
  method: "get" | "post" | "put" | "delete" | "patch"
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  params?: any
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
  let fullUrl = url
  if (params) {
    fullUrl += "?" + new URLSearchParams(params)
  }

  const response = await fetch(fullUrl, {
    method,
    body: data ? JSON.stringify(data) : undefined,
    headers,
  })

  if (response.ok) {
    return response.json()
  } else {
    const text = await response.text()
    throw json({error: text, request: url}, {status: response.status})
  }
}

export default customInstance
