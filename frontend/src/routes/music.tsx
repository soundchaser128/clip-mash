import {useState} from "react"
import Field from "../components/Field"
import Form from "../components/Form"

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
        <>
          <h1 className="font-bold text-3xl mb-4">Select music</h1>
          <Form className="gap-4 self-center">
            <Field label="Music URL">
              <input
                className="input input-bordered w-full"
                placeholder="Supports YouTube, Vimeo, ..."
              />
            </Field>
          </Form>
        </>
      )}
    </>
  )
}
