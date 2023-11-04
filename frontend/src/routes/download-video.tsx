import clsx from "clsx"
import {useStateMachine} from "little-state-machine"
import {HiArrowDown, HiCodeBracket, HiHeart} from "react-icons/hi2"
import {Link} from "react-router-dom"
import ExternalLink from "../components/ExternalLink"

const DownloadVideoPage = () => {
  const {state} = useStateMachine()
  const numSongs = state.data.songs?.length || 0
  const interactive = numSongs > 0 || state.data.interactive
  const videoId = state.data.videoId!

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
        <div
          className={clsx(
            "grid gap-2 w-full",
            interactive && "grid-cols-3",
            !interactive && "grid-cols-2",
          )}
        >
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
