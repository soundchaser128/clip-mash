import {useStateMachine} from "little-state-machine"
import {
  HiArrowDownTray,
  HiCodeBracket,
  HiDocumentText,
  HiHeart,
} from "react-icons/hi2"
import {Link} from "react-router-dom"
import ExternalLink from "../components/ExternalLink"
import {CreateVideoBody, DescriptionType, generateDescription} from "@/api"
import {saveBlobToDisk} from "@/helpers"

const DownloadVideoPage = () => {
  const {state} = useStateMachine()
  const numSongs = state.data.songs?.length || 0
  const interactive = numSongs > 0 || state.data.interactive
  const videoId = state.data.videoId!

  const onGenerateDescription = async (format: DescriptionType) => {
    const data = {
      clips: state.data.clips!,
      fileName: state.data.fileName!,
      songIds: state.data.songs?.map((s) => s.songId) || [],
      videoId: state.data.videoId!,
      encodingEffort: state.data.encodingEffort!,
      outputFps: state.data.outputFps!,
      outputResolution: state.data.outputResolution!,
      selectedMarkers: state.data.selectedMarkers!,
      videoCodec: state.data.videoCodec!,
      videoQuality: state.data.videoQuality!,
      musicVolume: state.data.musicVolume,
    } satisfies CreateVideoBody

    const response = await generateDescription(format, data)
    const blob = new Blob([response.body], {type: response.contentType})
    let extension
    switch (response.contentType) {
      case "text/markdown":
        extension = "md"
        break
      case "application/json":
        extension = "json"
        break
      case "application/yaml":
        extension = "yaml"
        break
      default:
        extension = "txt"
    }

    const fileName = `${state.data.fileName || "Compilation"} [${
      state.data.videoId
    }] - Description.${extension}`

    saveBlobToDisk(fileName, blob)
  }

  return (
    <div className="mt-2 w-full self-center flex flex-col items-center">
      <div className="flex flex-col gap-6">
        <h1 className="text-5xl font-bold text-center">🎉 Success!</h1>
        <p>
          You can now download the finished compilation!{" "}
          {interactive && (
            <>
              You can also create a <code>.funscript</code> file for use with
              sex toys.
            </>
          )}
        </p>
        <section className="flex flex-col gap-2">
          <a
            href={`/api/project/download?videoId=${encodeURIComponent(
              videoId,
            )}`}
            className="btn btn-success btn-lg"
            download
          >
            <HiArrowDownTray className="w-6 h-6 mr-2" />
            Download video
          </a>
          <div className="join self-center">
            <ExternalLink
              href="https://ko-fi.com/soundchaser128"
              className="btn btn-secondary join-item"
            >
              <HiHeart className="w-6 h-6 mr-2" />
              Support the developer
            </ExternalLink>
            <details className="dropdown">
              <summary className="btn w-full mb-1 join-item">
                <HiDocumentText className="w-6 h-6 mr-2" />
                Generate description
              </summary>
              <ul className="p-2 shadow-xl menu dropdown-content z-[1] bg-base-200 rounded-box w-full">
                <li>
                  <button onClick={() => onGenerateDescription("markdown")}>
                    Markdown
                  </button>
                </li>
                <li>
                  <button onClick={() => onGenerateDescription("json")}>
                    JSON
                  </button>
                </li>
                <li>
                  <button onClick={() => onGenerateDescription("yaml")}>
                    YAML
                  </button>
                </li>
              </ul>
            </details>
            {interactive && (
              <Link
                className="btn btn-primary join-item"
                to={`/${videoId}/funscript`}
              >
                <HiCodeBracket className="w-6 h-6 mr-2" />
                Create funscript
              </Link>
            )}
          </div>
        </section>
      </div>
    </div>
  )
}

export default DownloadVideoPage
