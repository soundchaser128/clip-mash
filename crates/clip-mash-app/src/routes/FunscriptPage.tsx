import {useStateMachine} from "little-state-machine"
import {useEffect, useRef, useState} from "react"
import {
  CreateBeatFunscriptBody,
  getBeatFunscript,
  getCombinedFunscript,
  listFinishedVideos,
} from "../api"
import {HiCodeBracket} from "react-icons/hi2"
import ExternalLink from "../components/ExternalLink"

const FunscriptPage = () => {
  const {state} = useStateMachine()
  const numSongs = state.data.songs?.length || 0

  const downloadLink = useRef<HTMLAnchorElement>(null)
  const [creatingScript, setCreatingScript] = useState(false)
  const [finalFileName, setFinalFileName] = useState<string>()
  const videoId = state.data.videoId!

  useEffect(() => {
    listFinishedVideos().then((fileNames) => {
      const fileName = fileNames.find((f) => f.includes(videoId))
      setFinalFileName(fileName || "compilation.mp4")
    })
  }, [videoId])

  const downloadJson = (json: unknown) => {
    const file = finalFileName!.replace(".mp4", ".funscript")
    const string = JSON.stringify(json)
    const blob = new Blob([string], {type: "application/json"})
    const downloadUrl = URL.createObjectURL(blob)
    if (downloadLink.current) {
      downloadLink.current.href = downloadUrl
      downloadLink.current.download = file
      downloadLink.current.click()
    }
  }

  const onCreateBeatFunscript = async (
    e: React.MouseEvent<HTMLButtonElement>,
  ) => {
    e.preventDefault()
    setCreatingScript(true)
    const songIds = state.data.songs?.map((s) => s.songId) || []
    const data = {
      songIds,
      strokeType: {
        accelerate: {
          start_strokes_per_beat: 3.0,
          end_strokes_per_beat: 1.0 / 3.0,
        },
      },
    } satisfies CreateBeatFunscriptBody

    const script = await getBeatFunscript(data)
    downloadJson(script)
    setCreatingScript(false)
  }

  const onDownloadFunscript = async (
    e: React.MouseEvent<HTMLButtonElement>,
  ) => {
    e.preventDefault()
    setCreatingScript(true)
    const script = await getCombinedFunscript({
      clips: state.data.clips!,
    })
    downloadJson(script)
    setCreatingScript(false)
  }
  return (
    <>
      <div className="mt-4 max-w-2xl self-center flex flex-col gap-2">
        <p>
          <code>.funscript</code> files are used by computer-connected sex toys
          like{" "}
          <ExternalLink href="https://www.thehandy.com/">
            the Handy
          </ExternalLink>{" "}
          to sync their actions to a video.
        </p>
        <p>
          You can generate a beat-based funscript if the compilation had music
          added to it, or, if the source videos have <code>.funscript</code>{" "}
          files stored next to them, you can generate a file that combines the
          actions of the included videos.
        </p>
        <p>
          You can use apps like{" "}
          <ExternalLink href="https://beta.funscript.io/app/play">
            funscript.io
          </ExternalLink>{" "}
          to run the script alongside the video.
        </p>
      </div>
      <div className="self-center mt-4 flex flex-col gap-6">
        <div>
          <p className="self-start mb-1">Generate beat-based .funscript file</p>
          <button
            onClick={onCreateBeatFunscript}
            className="btn btn-success btn-lg w-full"
            disabled={creatingScript || numSongs === 0}
          >
            <HiCodeBracket className="w-6 h-6 mr-2" />
            Beat-based funscript
          </button>
        </div>

        <div>
          <p className="self-start mb-1">Generate combined .funscript file</p>
          <button
            onClick={onDownloadFunscript}
            className="btn btn-success btn-lg w-full"
            disabled={creatingScript || !state.data.interactive}
          >
            <HiCodeBracket className="w-6 h-6 mr-2" />
            Combined funscript
          </button>
        </div>
      </div>

      <a className="hidden" ref={downloadLink} />
    </>
  )
}

export default FunscriptPage
