import Field from "../components/Field"
import {useForm} from "react-hook-form"

type MusicMode = "none" | "trimVideo" | "trimMusic"

interface Inputs {
  musicUrl: string
  mode: MusicMode
}

export default function Music() {
  const {handleSubmit, register} = useForm<Inputs>({})

  const onSubmit = async (values: Inputs) => {
    await fetch(`/api/music?url=${encodeURIComponent(values.musicUrl)}`, {
      method: "POST",
    })
  }

  return (
    <>
      <p className="text-center mb-4">Select music settings</p>

      <form
        onSubmit={handleSubmit(onSubmit)}
        className="flex flex-col self-center w-full max-w-xl gap-4"
      >
        <Field label="Music mode">
          <select className="select select-bordered" {...register("mode")}>
            <option value="trimVideo">Trim video based on music</option>
            <option value="trimMusic">Trim music based on video</option>
          </select>
        </Field>
        <Field label="Music URL">
          <input
            className="input input-bordered w-full"
            placeholder="Supports YouTube, Vimeo, ..."
            {...register("musicUrl")}
          />
        </Field>

        <button className="btn btn-success self-end" type="submit">
          Submit
        </button>
      </form>
    </>
  )
}
