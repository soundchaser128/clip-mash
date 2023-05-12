import {useState} from "react"

type MusicMode = "none" | "trimVideo" | "trimMusic"

export default function Music() {
  const [mode, setMode] = useState<MusicMode>()

  return (
    <>
      <p className="text-center mb-4">Select music settings</p>
      <div className="self-center grid grid-cols-3 gap-2">
        <button
          className="btn btn-primary btn-lg"
          onClick={() => setMode("none")}
        >
          None
        </button>
        <button
          className="btn btn-primary btn-lg"
          onClick={() => setMode("trimVideo")}
        >
          Trim video
        </button>
        <button
          className="btn btn-primary btn-lg"
          onClick={() => setMode("trimMusic")}
        >
          Trim music
        </button>
      </div>

      {mode !== "none" && (
        <section>
          Select music
          {/* 
            * Enter YouTube URL - OR - Upload music
            * 
          
          */}
        </section>
      )}
    </>
  )
}
