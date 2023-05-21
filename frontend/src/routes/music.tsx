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
import {useCallback, useRef, useState} from "react"
import {useLoaderData, useNavigate, useRevalidator} from "react-router-dom"
import {formatSeconds, sumDurations} from "../helpers"
import {
  HiArrowUpTray,
  HiBarsArrowDown,
  HiBolt,
  HiCheck,
  HiChevronRight,
  HiMusicalNote,
} from "react-icons/hi2"
import {Updater, useImmer} from "use-immer"
import {useDrag, useDrop} from "react-dnd"
import type {Identifier, XYCoord} from "dnd-core"
import clsx from "clsx"

interface Inputs {
  musicUrl: string
}

type Mode = "table" | "download" | "order" | "upload"

interface CardProps {
  id: number
  text: string
  index: number
  moveCard: (dragIndex: number, hoverIndex: number) => void
  className?: string
}

interface DragItem {
  index: number
  id: string
  type: string
}

function calcBPM(song: SongDto): string {
  return ((song.beats.length / song.duration) * 60.0).toFixed(0)
}

const Card: React.FC<CardProps> = ({id, text, index, moveCard, className}) => {
  const ref = useRef<HTMLDivElement>(null)
  const [{handlerId}, drop] = useDrop<
    DragItem,
    void,
    {handlerId: Identifier | null}
  >({
    accept: "CARD",
    collect(monitor) {
      return {
        handlerId: monitor.getHandlerId(),
      }
    },
    hover(item: DragItem, monitor) {
      if (!ref.current) {
        return
      }
      const dragIndex = item.index
      const hoverIndex = index

      // Don't replace items with themselves
      if (dragIndex === hoverIndex) {
        return
      }

      // Determine rectangle on screen
      const hoverBoundingRect = ref.current?.getBoundingClientRect()

      // Get vertical middle
      const hoverMiddleY =
        (hoverBoundingRect.bottom - hoverBoundingRect.top) / 2

      // Determine mouse position
      const clientOffset = monitor.getClientOffset()

      // Get pixels to the top
      const hoverClientY = (clientOffset as XYCoord).y - hoverBoundingRect.top

      // Only perform the move when the mouse has crossed half of the items height
      // When dragging downwards, only move when the cursor is below 50%
      // When dragging upwards, only move when the cursor is above 50%

      // Dragging downwards
      if (dragIndex < hoverIndex && hoverClientY < hoverMiddleY) {
        return
      }

      // Dragging upwards
      if (dragIndex > hoverIndex && hoverClientY > hoverMiddleY) {
        return
      }

      // Time to actually perform the action
      moveCard(dragIndex, hoverIndex)

      // Note: we're mutating the monitor item here!
      // Generally it's better to avoid mutations,
      // but it's good here for the sake of performance
      // to avoid expensive index searches.
      item.index = hoverIndex
    },
  })

  const [{isDragging}, drag] = useDrag({
    type: "CARD",
    item: () => {
      return {id, index}
    },
    collect: (monitor: any) => ({
      isDragging: monitor.isDragging(),
    }),
  })

  const opacity = isDragging ? 0 : 1
  drag(drop(ref))

  return (
    <div
      ref={ref}
      style={{opacity}}
      data-handler-id={handlerId}
      className={className}
    >
      {text}
    </div>
  )
}

const ReorderSongs: React.FC<{
  selection: number[]
  songs: SongDto[]
  setSelection: Updater<number[]>
}> = ({selection, songs, setSelection}) => {
  const moveCard = useCallback((dragIndex: number, hoverIndex: number) => {
    setSelection((draft) => {
      const temp = draft[dragIndex]
      draft.splice(dragIndex, 1)
      draft.splice(hoverIndex, 0, temp)
    })
  }, [])

  return (
    <>
      <h2 className="self-center font-bold text-xl">Change order of songs</h2>
      <p className="self-center mb-6">
        Drag and drop songs to change their order in the video.
      </p>
      <div className="self-center flex flex-col gap-2">
        {selection.map((songId, index) => {
          const song = songs.find((s) => s.songId === songId)!
          return (
            <Card
              className="border border-dashed border-primary px-4 py-3 rounded-lg cursor-move"
              key={song.songId}
              id={song.songId}
              text={song.fileName}
              index={index}
              moveCard={moveCard}
            />
          )
        })}
      </div>
    </>
  )
}

export default function Music() {
  const [mode, setMode] = useState<Mode>("table")
  const [file, setFile] = useState<File>()
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
  const [musicVolume, setMusicVolume] = useState(
    state.data.musicVolume ? state.data.musicVolume * 100 : 75
  )

  const totalMarkerDuration = sumDurations(state.data.selectedMarkers)
  const totalMusicDuration = selection
    .map((s) => songs.find((song) => song.songId === s))
    .reduce((sum, song) => sum + song!.duration, 0)

  const musicTooLong = totalMusicDuration > totalMarkerDuration

  const onSubmit = async (values: Inputs) => {
    setLoading(true)
    invariant(StateHelpers.isNotInitial(state.data))

    const response = await fetch(
      `/api/music/download?url=${encodeURIComponent(values.musicUrl)}`,
      {
        method: "POST",
      }
    )
    const data: SongDto = await response.json()

    await fetch(`/api/music/${data.songId}/beats`)

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

  const onUpload = async () => {
    invariant(StateHelpers.isNotInitial(state.data))
    if (file) {
      setLoading(true)
      const formData = new FormData()
      formData.set("file", file)
      const response = await fetch(`/api/music/upload`, {
        method: "POST",
        body: formData,
      })
      const data: SongDto = await response.json()

      actions.updateForm({
        songs: [...(state.data.songs || []), data],
      })
      setLoading(false)
      setMode("table")
      revalidator.revalidate()
    }
  }

  const onNextStage = () => {
    const nextStage = StateHelpers.isLocalFiles(state.data)
      ? LocalFilesFormStage.VideoOptions
      : FormStage.VideoOptions

    actions.updateForm({
      stage: nextStage,
      songs: selection.map((id) => songs.find((s) => s.songId === id)!),
      trimVideoForSongs: true,
      musicVolume: musicVolume / 100.0,
    })

    navigate("/stash/video-options")
  }

  return (
    <>
      <div className="justify-between flex w-full mb-4">
        <div className="flex gap-2">
          <button
            onClick={() => setMode("download")}
            className="btn btn-primary"
          >
            <HiMusicalNote className="mr-2" />
            Download music
          </button>
          <button onClick={() => setMode("upload")} className="btn btn-primary">
            <HiArrowUpTray className="mr-2" />
            Upload music
          </button>
          {mode !== "order" && (
            <button
              disabled={selection.length < 2}
              onClick={() => setMode("order")}
              className="btn btn-secondary w-48"
            >
              <HiBarsArrowDown className="mr-2" />
              Set track order
            </button>
          )}
          {mode === "order" && (
            <button
              className="btn btn-success w-48"
              onClick={() => setMode("table")}
            >
              <HiCheck className="mr-2" />
              Done
            </button>
          )}
        </div>
        <button
          type="button"
          onClick={onNextStage}
          className="btn btn-success place-self-end"
          disabled={musicTooLong}
        >
          Next
          <HiChevronRight className="ml-1" />
        </button>
      </div>

      {mode === "order" && (
        <>
          <ReorderSongs
            setSelection={setSelection}
            songs={songs}
            selection={selection}
          />
        </>
      )}

      {mode === "table" && (
        <div className="flex flex-col gap-2 mb-6">
          {musicTooLong && (
            <div className="alert alert-warning">
              <HiBolt className="w-6 h-6 shrink" />
              <span className="grow">
                The music tracks you selected are longer than the videos. To fix
                it, deselect some music or select some shorter tracks.
              </span>
            </div>
          )}
          <div className="">
            <p className="mb-4 font-light self-center max-w-2xl">
              You can select background music for your video compilation. (this
              is optional). The original sound of the video and the new music
              will be mixed together based on the music volume you selected,
              100% music volume meaning that only the music will be heard.
              <br />
              The length of the video will be determined by the selected music
              if you select any.
            </p>
            <p>
              Selected marker duration:{" "}
              <strong>{formatSeconds(totalMarkerDuration, "short")}</strong>
            </p>
            <p className={clsx(musicTooLong && "text-red-400")}>
              Selected music duration:{" "}
              <strong>{formatSeconds(totalMusicDuration, "short")}</strong>
            </p>
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
                <th>Beats per minute</th>
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
                  <td>{calcBPM(song)}</td>
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

      {mode === "download" && (
        <form
          onSubmit={handleSubmit(onSubmit)}
          className="flex flex-col self-center w-full max-w-xl gap-4"
        >
          <p className="font-light">
            You can download songs from YouTube, Vimeo or any other site that
            yt-dlp supports.
          </p>
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

      {mode === "upload" && (
        <div className="flex flex-col self-center w-full max-w-xl gap-4">
          <p>Select a song to upload:</p>
          <input
            type="file"
            className="file-input file-input-primary"
            name="upload"
            accept="audio/*"
            onChange={(e) => setFile(e.target.files![0])}
          />
          <div className="flex self-end gap-2">
            <button
              type="button"
              onClick={() => setMode("table")}
              className="btn btn-outline"
            >
              Cancel
            </button>
            <button
              onClick={onUpload}
              disabled={loading}
              className="btn btn-success"
            >
              Upload
            </button>
          </div>
        </div>
      )}
    </>
  )
}
