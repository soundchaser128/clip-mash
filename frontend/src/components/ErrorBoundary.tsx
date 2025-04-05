import TroubleshootingInfo from "./TroubleshootingInfo"
import Layout from "./Layout"
import {isRouteErrorResponse, useRouteError} from "react-router-dom"
import {useEffect} from "react"

async function logResponseError(response: Response) {
  let body
  if (!response.bodyUsed) {
    body = await response.text()
  }

  console.error("ErrorBoundary caught response:", {
    url: response.url,
    status: response.status,
    statusText: response.statusText,
    body,
  })
}

const ErrorBoundary = () => {
  const error = useRouteError()

  useEffect(() => {
    if (error instanceof Error) {
      console.error("ErrorBoundary caught error", error)
    } else if (error instanceof Response) {
      logResponseError(error)
    } else {
      console.error("ErrorBoundary caught some other error", error)
    }
  }, [error])

  if (isRouteErrorResponse(error)) {
    const is404 = error.status === 404

    return (
      <Layout>
        <div className="mt-8 flex flex-col">
          <h1 className="font-bold text-5xl mb-4 w-fit">
            {is404 ? "404 - Page not found" : "Sorry, something went wrong."}
          </h1>
          {!is404 && (
            <div className="bg-error text-error-content p-2 rounded-lg self-start mb-4">
              <p>
                Status code <strong>{error.status}</strong>
              </p>
              {error.data.error && <p>{error.data.error}</p>}
              {error.data.request && (
                <p>
                  Request to <code>{error.data.request}</code> failed.
                </p>
              )}
            </div>
          )}
        </div>
        <TroubleshootingInfo />
      </Layout>
    )
  }

  const errorJson = JSON.stringify(error, null, 2)
  const isUsefulJson = errorJson && errorJson !== "{}"
  const err = error as Error
  return (
    <Layout>
      <div className="mt-8 flex flex-col">
        <h1 className="font-bold text-5xl mb-4">
          Sorry, something went wrong.
        </h1>
        <div className="bg-error text-error-content p-2 rounded-lg self-start mb-4">
          <h2 className="font-bold">Error details:</h2>
          <div>
            {isUsefulJson && <pre>{errorJson}</pre>}
            {!isUsefulJson && (
              <p>
                <code>
                  {err.name}: {err.message}
                </code>
              </p>
            )}
          </div>
        </div>
        <TroubleshootingInfo />
      </div>
    </Layout>
  )
}

export default ErrorBoundary
