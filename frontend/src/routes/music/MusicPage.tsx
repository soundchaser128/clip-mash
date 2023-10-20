import {useStateMachine} from "little-state-machine"
import {useForm} from "react-hook-form"
import {updateForm} from "../actions"
import React, {useState} from "react"
import {Link, useLoaderData, useNavigate} from "react-router-dom"
import {formatSeconds, sumDurations} from "@/helpers"
import {
  HiArrowUpTray,
  HiBarsArrowDown,
  HiBolt,
  HiChevronRight,
  HiMusicalNote,
} from "react-icons/hi2"
import {useImmer} from "use-immer"
import clsx from "clsx"
import {SongDto} from "@/api"
import HelpModal from "@/components/HelpModal"
import {FormStage, FormState} from "@/types/form-state"
import {ClipStrategy} from "@/types/types"
import SongsTable from "./SongsTable"
import {produce} from "immer"

interface MusicSettingsInputs {
  musicVolume: number
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
    <form onChange={handleSubmit(onSubmit)} className="p-4">
      <h2 className="text-xl font-bold mb-2">Music settings</h2>
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
    </form>
  )
}

export default function Music() {
  const songs = useLoaderData() as SongDto[]
  const {actions, state} = useStateMachine({updateForm})

  const [selection, setSelection] = useImmer<number[]>(
    state.data.songs?.map((song) => song.songId) || [],
  )
  const [formValues, setFormValues] = useState<MusicSettingsInputs>({
    musicVolume: state.data.musicVolume ? state.data.musicVolume * 100 : 75,
  })
  const navigate = useNavigate()

  const totalMarkerDuration = sumDurations(state.data.selectedMarkers)
  const totalMusicDuration = selection
    .map((s) => songs.find((song) => song.songId === s))
    .reduce((sum, song) => sum + (song?.duration || 0), 0)

  const musicTooLong = totalMusicDuration > totalMarkerDuration
  const anySongsSelected = selection.length > 0

  const onToggleSong = (songId: number, checked: boolean) => {
    const newSelection = produce(selection, (draft) => {
      if (checked) {
        draft.push(songId)
      } else {
        const index = draft.indexOf(songId)
        if (index !== -1) {
          draft.splice(index, 1)
        }
      }
    })
    setSelection(newSelection)
    actions.updateForm({
      songs: newSelection.map((id) => songs.find((s) => s.songId === id)!),
    })
  }

  const onFormChange = (values: MusicSettingsInputs) => {
    setFormValues(values)
  }

  const onNextStage = () => {
    const nextStage = FormStage.VideoOptions
    const anySongsSelected = selection.length > 0

    const update: Partial<FormState> = anySongsSelected
      ? {
          stage: nextStage,
          songs: selection.map((id) => songs.find((s) => s.songId === id)!),
          musicVolume: formValues.musicVolume / 100.0,
        }
      : {
          stage: nextStage,
        }

    actions.updateForm(update)
    navigate("/video-options")
  }

  return (
    <>
      <div className="justify-between flex w-full mb-4">
        <div className="flex gap-2">
          <Link to="/music/download" className="btn btn-primary">
            <HiMusicalNote className="mr-2" />
            Download music
          </Link>
          <Link to="/music/upload" className="btn btn-primary">
            <HiArrowUpTray className="mr-2" />
            Upload music
          </Link>
          <Link
            to="/music/reorder"
            className={clsx(
              "btn btn-secondary w-52",
              selection.length < 2 && "btn-disabled",
            )}
          >
            <HiBarsArrowDown className="mr-2" />
            Set track order
          </Link>
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

      {musicTooLong && (
        <div className="alert alert-warning">
          <HiBolt className="w-6 h-6 shrink" />
          <span className="grow">
            The music tracks you selected are longer than the videos. To fix it,
            deselect some music or select some shorter tracks.
          </span>
        </div>
      )}

      <div className="flex gap-2 w-full">
        <SongsTable
          onToggleSong={onToggleSong}
          selection={selection}
          songs={songs}
        />
        {anySongsSelected && (
          <div>
            <MusicSettingsForm
              defaultValues={formValues}
              onChange={onFormChange}
            />
            <div>
              <p>
                Selected marker duration:{" "}
                <strong>{formatSeconds(totalMarkerDuration, "short")}</strong>
              </p>
              <p className={clsx(musicTooLong && "text-red-400")}>
                Selected music duration:{" "}
                <strong>{formatSeconds(totalMusicDuration, "short")}</strong>
              </p>
            </div>
          </div>
        )}
        {!anySongsSelected && (
          <p className="p-4 text-center">Select some songs to show options</p>
        )}
      </div>
    </>
  )
}
