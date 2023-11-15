import {useRouteLoaderData} from "react-router-dom"
import ExternalLink from "./ExternalLink"
import Toast from "./Toast"
import {AppVersion} from "@/api"

export default function UpdateAvailableAlert() {
  const version = useRouteLoaderData("root") as AppVersion
  if (!version.needsUpdate) {
    return null
  }

  const url = `https://github.com/soundchaser128/clip-mash/releases/tag/v${version.newestVersion}`

  return (
    <Toast type="success">
      <strong>New version available!</strong>
      <br />
      Download it here:{" "}
      <ExternalLink href={url}>ClipMash v{version.newestVersion}</ExternalLink>
    </Toast>
  )
}
