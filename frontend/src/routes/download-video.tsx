import {useStateMachine} from "little-state-machine"
import {
  HiArrowDown,
  HiCodeBracket,
  HiDocumentText,
  HiHeart,
} from "react-icons/hi2"
import {Link} from "react-router-dom"
import ExternalLink from "../components/ExternalLink"
import {CreateVideoBody, generateDescription} from "@/api"
import {saveBlobToDisk} from "@/helpers"

const DownloadVideoPage = () => {
  const {state} = useStateMachine()
  const numSongs = state.data.songs?.length || 0
  const interactive = numSongs > 0 || state.data.interactive
  const videoId = state.data.videoId!

  const onGenerateDescription = async () => {
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

    const description = await generateDescription("markdown", data)
    const blob = new Blob([description], {type: "text/plain"})
    saveBlobToDisk("description.md", blob)
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
        <div className="grid grid-flow-col gap-2 w-full">
          <div className="flex flex-col">
            <a
              href={`/api/project/download?videoId=${encodeURIComponent(
                videoId,
              )}`}
              className="btn btn-success btn-lg"
              download
            >
              <HiArrowDown className="w-6 h-6 mr-2" />
              Download video
            </a>
          </div>
          <div className="flex flex-col">
            <button
              onClick={onGenerateDescription}
              className="btn btn-success btn-lg"
            >
              <HiDocumentText className="w-6 h-6 mr-2" />
              Generate description
            </button>
          </div>
          {interactive && (
            <div className="flex flex-col">
              <Link
                className="btn btn-primary btn-lg"
                to={`/${videoId}/funscript`}
              >
                <HiCodeBracket className="w-6 h-6 mr-2" />
                Create funscript
              </Link>
            </div>
          )}
          <div className="flex flex-col">
            <ExternalLink
              href="https://ko-fi.com/soundchaser128"
              className="btn btn-lg btn-secondary"
            >
              <HiHeart className="w-6 h-6 mr-2" />
              Support the developer
            </ExternalLink>
          </div>
        </div>
      </div>
    </div>
  )
}

export default DownloadVideoPage
