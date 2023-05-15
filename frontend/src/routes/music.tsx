import {useStateMachine} from "little-state-machine"
import Field from "../components/Field"
import {useForm} from "react-hook-form"
import {updateForm} from "./actions"
import invariant from "tiny-invariant"
import {FormStage, SongDto, StateHelpers} from "../types/types"
import {useState} from "react"
import {
  LoaderFunction,
  useLoaderData,
  useNavigate,
  useRevalidator,
} from "react-router-dom"
import {formatSeconds} from "../helpers"
import {HiChevronRight, HiMusicalNote} from "react-icons/hi2"
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
  const revalidator = useRevalidator()
  const [trimVideo, setTrimVideo] = useState(false)
  const [musicVolume, setMusicVolume] = useState(75)

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
    revalidator.revalidate()
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
      trimVideoForSongs: trimVideo,
      musicVolume: musicVolume / 100.0,
    })

    navigate("/stash/video-options")
  }

  return (
    <>
      <div className="justify-between flex w-full mb-4">
        <div />
        <button
          type="button"
          onClick={onNextStage}
          className="btn btn-success place-self-end"
        >
          Next
          <HiChevronRight className="ml-1" />
        </button>
      </div>

      <div className="form-control self-start">
        <label className="label cursor-pointer">
          <span className="label-text mr-2">
            Trim video based on music used
          </span>
          <input
            type="checkbox"
            className="toggle"
            onChange={(e) => setTrimVideo(e.target.checked)}
            checked={trimVideo}
          />
        </label>
      </div>

      <div className="form-control self-start">
        <label className="label">
          <span className="label-text">Music volume</span>
        </label>
        <input
          type="range"
          min="0"
          max="100"
          className="range range-sm w-72"
          step="5"
          value={musicVolume}
          onChange={(e) => setMusicVolume(e.target.valueAsNumber)}
        />
        <div className="w-full flex justify-between text-xs px-2">
          <span>0%</span>
          <span>100%</span>
        </div>
      </div>

      {mode === "table" && (
        <div className="overflow-x-auto flex flex-col">
          <table className="table table-compact w-full">
            <thead>
              <tr>
                <th>Name</th>
                <th>Duration</th>
                <th>URL</th>
                <th>Include</th>
              </tr>
            </thead>
            <tbody>
              {songs.length === 0 && (
                <tr>
                  <td className="text-center p-4" colSpan={4}>
                    No music yet.
                  </td>
                </tr>
              )}
              {songs.map((song) => (
                <tr key={song.songId}>
                  <td>{song.fileName}</td>
                  <td>{formatSeconds(song.duration, "short")}</td>
                  <td>{song.url}</td>
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
