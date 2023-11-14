import {HiInformationCircle} from "react-icons/hi2"
import Sentry from "@/sentry"
import {useState} from "react"

export default function SentryInfo() {
  const [visible, setVisible] = useState(Sentry.enabled === null)

  const onDisable = () => {
    Sentry.enabled = false
    setVisible(false)
  }

  const onClose = () => {
    Sentry.enabled = true
    Sentry.setup()
    setVisible(false)
  }

  if (!visible) {
    return null
  }

  return (
    <div className="alert self-center my-4 max-w-2xl">
      <HiInformationCircle className="w-8 h-8" />
      <span>
        ClipMash uses{" "}
        <a className="link" href="https://sentry.io">
          Sentry
        </a>{" "}
        to collect anonymous error reports. You can disable this here.
      </span>

      <div className="join">
        <button onClick={onClose} className="btn btn-ghost join-item">
          Close
        </button>
        <button onClick={onDisable} className="btn btn-primary join-item">
          Disable
        </button>
      </div>
    </div>
  )
}
