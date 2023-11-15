import {HiInformationCircle} from "react-icons/hi2"
import Sentry from "@/sentry"
import {useState} from "react"
import Toast from "./Toast"

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
    <Toast hideCloseButton>
      <span className="mr-8">
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
    </Toast>
  )
}
