import {useStateMachine} from "little-state-machine"
import Field from "../components/Field"
import {useForm} from "react-hook-form"
import {updateForm} from "./actions"
import invariant from "tiny-invariant"
import {
  FormStage,
  LocalFilesFormStage,
  SongDto,
  StateHelpers,
} from "../types/types"
import {useState} from "react"
import {
  LoaderFunction,
  useLoaderData,
  useNavigate,
  useRevalidator,
} from "react-router-dom"
import {formatSeconds} from "../helpers"
import {HiBarsArrowDown, HiChevronRight, HiMusicalNote} from "react-icons/hi2"
import {useImmer} from "use-immer"

interface Inputs {
  musicUrl: string
}

type Mode = "table" | "form" | "order"

const ReorderSongs: React.FC<{selection: number[]; songs: SongDto[]}> = ({
  selection,
  songs,
}) => {
  return (
    <>
      <h2 className="self-center font-bold text-xl">Change order of songs</h2>
      <p className="self-center mb-6">
        Drag and drop songs to change their order in the video.
      </p>
      <ul className="self-center flex flex-col gap-2">
        {songs
          .filter((s) => selection.includes(s.songId))
          .map((song, index) => (
            <li
              className="border border-primary px-2 py-1 rounded-lg"
              key={song.songId}
            >
              {index + 1}. {song.fileName}
            </li>
          ))}
      </ul>
    </>
  )
}

export default function Music() {
  const [mode, setMode] = useState<Mode>("table")
  const songs = useLoaderData() as SongDto[]
  const {handleSubmit, register} = useForm<Inputs>({})
  const {actions, state} = useStateMachine({updateForm})
  invariant(StateHelpers.isNotInitial(state.data))
  const [loading, setLoading] = useState(false)
  const [selection, setSelection] = useImmer<number[]>(
    state.data.songs?.map((song) => song.songId) || []
  )
  const navigate = useNavigate()
  const revalidator = useRevalidator()
  const [trimVideo, setTrimVideo] = useState(
    state.data.trimVideoForSongs || false
  )
  const [musicVolume, setMusicVolume] = useState(
    state.data.musicVolume ? state.data.musicVolume * 100 : 75
  )

  const indexOptions: number[] = []
  for (let i = 0; i < selection.length; i++) {
    indexOptions.push(i + 1)
  }

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
    const nextStage = StateHelpers.isLocalFiles(state.data)
      ? LocalFilesFormStage.VideoOptions
      : FormStage.VideoOptions

    actions.updateForm({
      stage: nextStage,
      songs: selection.map((id) => songs.find((s) => s.songId === id)!),
      trimVideoForSongs: trimVideo,
      musicVolume: musicVolume / 100.0,
    })

    navigate("/stash/video-options")
  }

  return (
    <>
      <div className="justify-between flex w-full mb-4">
        <div className="flex gap-2">
          <button onClick={() => setMode("form")} className="btn btn-primary">
            <HiMusicalNote className="mr-2" />
            Add music
          </button>
          {mode !== "order" && (
            <button
              disabled={selection.length < 2}
              onClick={() => setMode("order")}
              className="btn btn-secondary"
            >
              <HiBarsArrowDown className="mr-2" />
              Set track order
            </button>
          )}
          {mode === "order" && (
            <button
              className="btn btn-success"
              onClick={() => setMode("table")}
            >
              Done
            </button>
          )}
        </div>
        <button
          type="button"
          onClick={onNextStage}
          className="btn btn-success place-self-end"
        >
          Next
          <HiChevronRight className="ml-1" />
        </button>
      </div>

      {mode === "order" && (
        <>
          <ReorderSongs songs={songs} selection={selection} />
        </>
      )}

      {mode === "table" && (
        <div className="flex flex-col gap-2 mb-6">
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
              <span className="font-bold">{musicVolume}%</span>
              <span>100%</span>
            </div>
          </div>
        </div>
      )}

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
                  <td className="text-center p-4" colSpan={5}>
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
                      checked={selection.includes(song.songId)}
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
          <div className="flex gap-2 self-end">
            <button
              type="button"
              onClick={() => setMode("table")}
              className="btn btn-outline"
            >
              Cancel
            </button>
            <button
              disabled={loading}
              className="btn btn-success"
              type="submit"
            >
              Submit
            </button>
          </div>
        </form>
      )}
    </>
  )
}
