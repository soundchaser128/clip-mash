import {json} from "react-router-dom"

export const customInstance = async <T>({
  url,
  method,
  params,
  data,
}: {
  url: string
  method: "get" | "post" | "put" | "delete" | "patch"
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  params?: any
  data?: unknown
  responseType?: string
  headers?: Record<string, string>
}): Promise<T> => {
  let fullUrl = url
  if (params) {
    fullUrl += "?" + new URLSearchParams(params)
  }

  const response = await fetch(fullUrl, {
    method,
    ...(data ? {body: JSON.stringify(data)} : {}),
  })

  if (response.ok) {
    return response.json()
  } else {
    const text = await response.text()
    throw json({error: text, request: url}, {status: response.status})
  }
}

export default customInstance
