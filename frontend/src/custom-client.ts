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
  const response = await fetch(`${url}` + new URLSearchParams(params), {
    method,
    ...(data ? {body: JSON.stringify(data)} : {}),
  })

  return response.json()
}

export default customInstance
