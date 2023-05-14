import {useStateMachine} from "little-state-machine"
import Field from "../components/Field"
import {useForm} from "react-hook-form"
import {updateForm} from "./actions"
import invariant from "tiny-invariant"
import {FormStage, SongDto, StateHelpers} from "../types/types"
import {useState} from "react"
import {LoaderFunction, useLoaderData, useNavigate} from "react-router-dom"
import {formatSeconds} from "../helpers"
import {HiChevronRight} from "react-icons/hi2"
import {useImmer} from "use-immer"

type MusicMode = "none" | "trimVideo" | "trimMusic"

interface Inputs {
  musicUrl: string
  mode: MusicMode
}

export const loader: LoaderFunction = async () => {
  const response = await fetch("/api/music")
  const data = (await response.json()) as SongDto[]
  return data
}

type Mode = "table" | "form"

export default function Music() {
  const [mode, setMode] = useState<Mode>("table")
  const songs = useLoaderData() as SongDto[]
  const {handleSubmit, register} = useForm<Inputs>({})
  const {actions, state} = useStateMachine({updateForm})
  const [loading, setLoading] = useState(false)
  const [selection, setSelection] = useImmer<number[]>([])
  const navigate = useNavigate()

  const onSubmit = async (values: Inputs) => {
    setLoading(true)
    invariant(StateHelpers.isNotInitial(state.data))

    const response = await fetch(
      `/api/music?url=${encodeURIComponent(values.musicUrl)}`,
      {
        method: "POST",
      }
    )
    const data: SongDto = await response.json()

    actions.updateForm({
      songs: [...(state.data.songs || []), data],
    })
    setLoading(false)
    setMode("table")
  }

  const onToggleSong = (songId: number, checked: boolean) => {
    setSelection((draft) => {
      if (checked) {
        draft.push(songId)
      } else {
        const index = draft.indexOf(songId)
        if (index !== -1) {
          draft.splice(index, 1)
        }
      }
    })
  }

  const onNextStage = () => {
    actions.updateForm({
      stage: FormStage.VideoOptions,
      songs: selection.map((id) => songs.find((s) => s.songId === id)!),
    })

    navigate("/stash/video-options")
  }

  return (
    <>
      <div className="justify-between flex w-full">
        <h2 className="text-2xl font-bold mb-1">Music</h2>

        <button
          type="button"
          onClick={onNextStage}
          className="btn btn-success place-self-end"
        >
          Next
          <HiChevronRight className="ml-1" />
        </button>
      </div>
      <p className="mb-4">
        Select songs to include (if any). You can also select whether the length
        of the video should be determined by the music selected, or if the music
        should be truncated to fit the length of the selected markers.
      </p>
      {mode === "table" && (
        <div className="overflow-x-auto flex flex-col">
          <button
            onClick={() => setMode("form")}
            className="btn btn-primary self-end mb-4"
          >
            Add music
          </button>
          <table className="table table-compact w-full">
            <thead>
              <tr>
                <th>Name</th>
                <th>Duration</th>
                <th>Include</th>
              </tr>
            </thead>
            <tbody>
              {songs.map((song) => (
                <tr key={song.songId}>
                  <td>{song.fileName}</td>
                  <td>{formatSeconds(song.duration, "short")}</td>
                  <td>
                    <input
                      type="checkbox"
                      className="checkbox checkbox-primary"
                      onChange={(e) =>
                        onToggleSong(song.songId, e.target.checked)
                      }
                    />
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      {mode === "form" && (
        <form
          onSubmit={handleSubmit(onSubmit)}
          className="flex flex-col self-center w-full max-w-xl gap-4"
        >
          <Field label="Music URL">
            <input
              className="input input-bordered w-full"
              placeholder="Supports YouTube, Vimeo, ..."
              {...register("musicUrl")}
            />
          </Field>

          <button
            disabled={loading}
            className="btn btn-success self-end"
            type="submit"
          >
            Submit
          </button>
        </form>
      )}
    </>
  )
}
