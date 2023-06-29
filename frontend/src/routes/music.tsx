import {useStateMachine} from "little-state-machine"
import Field from "../components/Field"
import {useForm} from "react-hook-form"
import {updateForm} from "./actions"
import {
  ClipStrategy,
  FormStage,
  LocalFilesFormStage,
  StateHelpers,
} from "../types/types"
import React, {useCallback, useRef, useState} from "react"
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
import {SongDto} from "../types.generated"
import HelpModal from "../components/HelpModal"
import useNotification from "../hooks/useNotification"

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
    // eslint-disable-next-line react-hooks/exhaustive-deps
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

interface SongsTableProps {
  songs: SongDto[]
  selection: number[]
  onToggleSong: (songId: number, checked: boolean) => void
}

const SongsTable: React.FC<SongsTableProps> = ({
  songs,
  selection,
  onToggleSong,
}) => {
  return (
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
              <td>
                <a
                  href={song.url}
                  target="_blank"
                  className="link"
                  rel="noreferrer"
                >
                  {song.url}
                </a>
              </td>
              <td>{calcBPM(song)}</td>
              <td>
                <input
                  type="checkbox"
                  className="checkbox checkbox-primary"
                  checked={selection.includes(song.songId)}
                  onChange={(e) => onToggleSong(song.songId, e.target.checked)}
                />
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  )
}

interface UploadMusicProps {
  onCancel: () => void
  onSuccess: (song: SongDto) => void
}

const UploadMusic: React.FC<UploadMusicProps> = ({onCancel, onSuccess}) => {
  const [file, setFile] = useState<File>()
  const [loading, setLoading] = useState(false)

  const onUpload = async () => {
    if (file) {
      setLoading(true)
      const formData = new FormData()
      formData.set("file", file)
      const response = await fetch(`/api/song/upload`, {
        method: "POST",
        body: formData,
      })
      const data: SongDto = await response.json()
      onSuccess(data)
    }
  }

  return (
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
        <button type="button" onClick={onCancel} className="btn btn-outline">
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
  )
}

const DownloadMusic: React.FC<UploadMusicProps> = ({onSuccess, onCancel}) => {
  const [loading, setLoading] = useState(false)
  const {handleSubmit, register, reset} = useForm<Inputs>({})

  const onSubmit = async (values: Inputs) => {
    setLoading(true)

    const response = await fetch(
      `/api/song/download?url=${encodeURIComponent(values.musicUrl)}`,
      {
        method: "POST",
      }
    )
    const data: SongDto = await response.json()
    await fetch(`/api/song/${data.songId}/beats`)
    setLoading(false)
    onSuccess(data)
    reset()
  }

  return (
    <form
      onSubmit={handleSubmit(onSubmit)}
      className="flex flex-col self-center w-full max-w-xl gap-4"
    >
      <p className="font-light">
        You can download songs from YouTube, Vimeo or any other site that yt-dlp
        supports.
      </p>
      <Field label="Music URL">
        <input
          className="input input-bordered w-full"
          placeholder="Supports YouTube, Vimeo, ..."
          {...register("musicUrl")}
        />
      </Field>
      <div className="flex gap-2 self-end">
        <button type="button" onClick={onCancel} className="btn btn-outline">
          Cancel
        </button>
        <button disabled={loading} className="btn btn-success" type="submit">
          Submit
        </button>
      </div>
    </form>
  )
}

interface MusicSettingsInputs {
  musicVolume: number
  clipStrategy: ClipStrategy
}

interface MusicSettingsFormProps {
  defaultValues: MusicSettingsInputs
  onChange: (settings: MusicSettingsInputs) => void
}

const MusicSettingsForm: React.FC<MusicSettingsFormProps> = ({
  defaultValues,
  onChange,
}) => {
  const {register, handleSubmit, watch} = useForm<MusicSettingsInputs>({
    defaultValues,
  })
  const musicVolume = watch("musicVolume")

  const onSubmit = (values: MusicSettingsInputs) => {
    onChange(values)
  }

  return (
    <form onChange={handleSubmit(onSubmit)} className="self-start">
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
          {...register("musicVolume", {valueAsNumber: true})}
        />
        <div className="w-full flex justify-between text-xs px-2">
          <span>0%</span>
          <span className="font-bold">{musicVolume}%</span>
          <span>100%</span>
        </div>
      </div>
      <div className="form-control self-start">
        <label className="label">
          <span className="label-text">Clip generation strategy</span>
        </label>
        <select
          className="select select-bordered"
          {...register("clipStrategy")}
        >
          <option value="roundRobin">Music-based (cut on the beat)</option>
          <option value="equalLength">Random lengths (default)</option>
        </select>
      </div>
    </form>
  )
}

export default function Music() {
  const [mode, setMode] = useState<Mode>("table")
  const songs = useLoaderData() as SongDto[]
  const {actions, state} = useStateMachine({updateForm})

  const [selection, setSelection] = useImmer<number[]>(
    state.data.songs?.map((song) => song.songId) || []
  )
  const [formValues, setFormValues] = useState<MusicSettingsInputs>({
    clipStrategy: state.data.clipStrategy || "roundRobin",
    musicVolume: state.data.musicVolume ? state.data.musicVolume * 100 : 75,
  })
  const navigate = useNavigate()
  const revalidator = useRevalidator()
  const sendNotification = useNotification()

  const totalMarkerDuration = sumDurations(state.data.selectedMarkers)
  const totalMusicDuration = selection
    .map((s) => songs.find((song) => song.songId === s))
    .reduce((sum, song) => sum + (song?.duration || 0), 0)

  const musicTooLong = totalMusicDuration > totalMarkerDuration
  const anySongsSelected = selection.length > 0

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

  const onUploadSuccess = async (song: SongDto) => {
    actions.updateForm({
      songs: [...(state.data.songs || []), song],
    })
    setMode("table")
    revalidator.revalidate()
    sendNotification(`Song downloaded successfully!`)
  }

  const onFormChange = (values: MusicSettingsInputs) => {
    setFormValues(values)
  }

  const onNextStage = () => {
    const nextStage = StateHelpers.isLocalFiles(state.data)
      ? LocalFilesFormStage.VideoOptions
      : FormStage.VideoOptions

    actions.updateForm({
      stage: nextStage,
      songs: selection.map((id) => songs.find((s) => s.songId === id)!),
      trimVideoForSongs: true,
      musicVolume: formValues.musicVolume / 100.0,
      clipStrategy: formValues.clipStrategy,
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
              className="btn btn-secondary w-52"
            >
              <HiBarsArrowDown className="mr-2" />
              Set track order
            </button>
          )}
          {mode === "order" && (
            <button
              className="btn btn-success w-52"
              onClick={() => setMode("table")}
            >
              <HiCheck className="mr-2" />
              Done
            </button>
          )}
        </div>
        <div className="flex gap-2">
          <HelpModal>
            <h1 className="mb-4 font-bold text-2xl">Music options</h1>
            <p className="mb-2">
              You can select background music for your video compilation. (this
              is optional). The original sound of the video and the new music
              will be mixed together based on the music volume you selected,
              100% music volume meaning that only the music will be heard.
            </p>
            <p>
              The length of the video will be determined by the selected music
              if you select any. You can also choose how the clips will be
              generated: Either by selecting a base duration for the clips (the
              generated clips will then have some fraction of the length of that
              base duration) or by using the detected BPM to direct the cuts
              (cuts will only happen on the beat)
            </p>
          </HelpModal>
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
      </div>

      {mode === "order" && (
        <ReorderSongs
          setSelection={setSelection}
          songs={songs}
          selection={selection}
        />
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
          <div>
            {anySongsSelected && (
              <>
                <p>
                  Selected marker duration:{" "}
                  <strong>{formatSeconds(totalMarkerDuration, "short")}</strong>
                </p>
                <p className={clsx(musicTooLong && "text-red-400")}>
                  Selected music duration:{" "}
                  <strong>{formatSeconds(totalMusicDuration, "short")}</strong>
                </p>
              </>
            )}
          </div>
          {anySongsSelected && (
            <MusicSettingsForm
              defaultValues={formValues}
              onChange={onFormChange}
            />
          )}
        </div>
      )}

      {mode === "table" && (
        <SongsTable
          songs={songs}
          selection={selection}
          onToggleSong={onToggleSong}
        />
      )}

      {mode === "download" && (
        <DownloadMusic
          onCancel={() => setMode("table")}
          onSuccess={onUploadSuccess}
        />
      )}

      {mode === "upload" && (
        <UploadMusic
          onCancel={() => setMode("table")}
          onSuccess={onUploadSuccess}
        />
      )}
    </>
  )
}
