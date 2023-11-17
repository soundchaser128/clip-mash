import {HiBookmark, HiServer} from "react-icons/hi2"

export default function SentryDebug() {
  const onClientError = () => {
    throw new Error("Client error for testing Sentry")
  }

  const onServerError = async () => {
    await fetch("/api/debug/sentry-error", {method: "POST"})
  }

  return (
    <div className="self-center flex gap-4">
      <button onClick={onClientError} className="btn btn-error">
        <HiBookmark />
        Throw client error
      </button>

      <button onClick={onServerError} className="btn btn-error">
        <HiServer />
        Throw server error
      </button>
    </div>
  )
}
