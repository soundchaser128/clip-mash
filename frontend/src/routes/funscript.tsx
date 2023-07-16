import {useStateMachine} from "little-state-machine"
import {useRef, useState} from "react"
import {CreateBeatFunscriptBody} from "../types/types.generated"
import {HiCodeBracket} from "react-icons/hi2"

const FunscriptPage = () => {
  const {state} = useStateMachine()
  const numSongs = state.data.songs?.length || 0
  const interactive = numSongs > 0 || state.data.interactive

  const downloadLink = useRef<HTMLAnchorElement>(null)
  const [creatingScript, setCreatingScript] = useState(false)
  const finalFileName = state.data.finalFileName!

  const onCreateBeatFunscript = async (
    e: React.MouseEvent<HTMLButtonElement>,
  ) => {
    e.preventDefault()
    setCreatingScript(true)
    const songIds = state.data.songs?.map((s) => s.songId) || []
    const data = {
      songIds,
      strokeType: {
        accellerate: {
          start_strokes_per_beat: 3.0,
          end_strokes_per_beat: 1.0 / 3.0,
        },
      },
    } satisfies CreateBeatFunscriptBody
    const response = await fetch("/api/funscript/beat", {
      method: "POST",
      body: JSON.stringify(data),
      headers: {"content-type": "application/json"},
    })
    const script = await response.blob()
    const file = finalFileName.replace(".mp4", ".funscript")
    const downloadUrl = URL.createObjectURL(script)
    if (downloadLink.current) {
      downloadLink.current.href = downloadUrl
      downloadLink.current.download = file
      downloadLink.current.click()
    }
    setCreatingScript(false)
  }

  const onDownloadFunscript = async (
    e: React.MouseEvent<HTMLButtonElement>,
  ) => {
    e.preventDefault()
    setCreatingScript(true)
    const body = JSON.stringify(state.data)
    const response = await fetch("/api/funscript/combined", {
      method: "POST",
      body,
      headers: {"content-type": "application/json"},
    })

    const script = await response.blob()
    const file = finalFileName.replace(".mp4", ".funscript")
    const downloadUrl = URL.createObjectURL(script)
    if (downloadLink.current) {
      downloadLink.current.href = downloadUrl
      downloadLink.current.download = file
      downloadLink.current.click()
    }
    setCreatingScript(false)
  }
  return (
    <>
      <div>
        This compilation is interactive. You can use e.g.{" "}
        <a
          href="https://beta.funscript.io/"
          target="_blank"
          rel="noreferrer"
          className="link"
        >
          Funplayer
        </a>{" "}
        to play it alongside the video in your browser, with supported toys like
        the{" "}
        <a
          href="https://www.thehandy.com/"
          target="_blank"
          rel="noreferrer"
          className="link"
        >
          Handy
        </a>
        .
        <br />
        Make sure to take a look at the generated file before playing it. It
        might contain awkward sections or abrupt changes in speed.
      </div>
      <div className="self-center mt-4 flex flex-col gap-4">
        {numSongs > 0 && (
          <div>
            <p className="font-light self-start mb-1">
              Generate beat-based .funscript file
            </p>
            <button
              onClick={onCreateBeatFunscript}
              className="btn btn-success btn-lg"
              disabled={creatingScript}
            >
              <HiCodeBracket className="w-6 h-6 mr-2" />
              Beat-based funscript
            </button>
          </div>
        )}
        {state.data.interactive && (
          <div>
            <p className="font-light self-start mb-1">
              Generate combined .funscript file
            </p>
            <button
              onClick={onDownloadFunscript}
              className="btn btn-success btn-lg"
              disabled={creatingScript}
            >
              <HiCodeBracket className="w-6 h-6 mr-2" />
              Combined funscript
            </button>
          </div>
        )}
      </div>

      <a className="hidden" ref={downloadLink} />
    </>
  )
}

export default FunscriptPage
