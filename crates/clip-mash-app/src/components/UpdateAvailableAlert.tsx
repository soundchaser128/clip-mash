import {useRouteLoaderData} from "react-router-dom"
import ExternalLink from "./ExternalLink"
import Toast from "./Toast"
import {AppVersion} from "@/api"
import useLocalStorage from "@/hooks/useLocalStorage"

export default function UpdateAvailableAlert() {
  const [alertDismissed, setAlertDismissed] = useLocalStorage(
    "updateAlertDismissed",
    false,
  )

  const onClose = () => {
    setAlertDismissed(true)
  }

  const version = useRouteLoaderData("root") as AppVersion
  if (!version.needsUpdate || alertDismissed) {
    return null
  }

  const url = `https://github.com/soundchaser128/clip-mash/releases/tag/v${version.newestVersion}`

  return (
    <Toast onClose={onClose} type="success">
      <strong>New version available!</strong>
      <br />
      Download it here:{" "}
      <ExternalLink href={url}>ClipMash v{version.newestVersion}</ExternalLink>
    </Toast>
  )
}
