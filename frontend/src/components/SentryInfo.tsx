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
    <div className="toast toast-center toast-top z-20">
      <div className="alert shadow-xl">
        <HiInformationCircle className="w-8 h-8" />
        <span>
          ClipMash uses{" "}
          <a className="link" href="https://sentry.io">
            Sentry
          </a>{" "}
          to collect anonymous error reports.
        </span>
        <div className="join">
          <button onClick={onClose} className="btn btn-primary join-item">
            Enable
          </button>
          <button onClick={onDisable} className="btn btn-secondary join-item">
            Disable
          </button>
        </div>
      </div>
    </div>
  )
}
