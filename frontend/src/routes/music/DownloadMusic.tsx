import {SongDto, downloadMusic, getBeats} from "@/api"
import Field from "@/components/Field"
import Loader from "@/components/Loader"
import {useState} from "react"
import {useForm} from "react-hook-form"
import {Link, useNavigate} from "react-router-dom"

interface Inputs {
  musicUrl: string
}

const DownloadMusic: React.FC = () => {
  const [loading, setLoading] = useState(false)
  const {handleSubmit, register, reset} = useForm<Inputs>({})
  const navigate = useNavigate()

  const onSubmit = async (values: Inputs) => {
    setLoading(true)

    const data = await downloadMusic({
      url: values.musicUrl,
    })
    await getBeats(data.songId)

    setLoading(false)
    reset()
    navigate("/music")
  }

  if (loading) {
    return (
      <Loader className="self-center">
        Downloading song and detecting beats.
      </Loader>
    )
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
        <Link to="/music" className="btn btn-outline">
          Cancel
        </Link>
        <button className="btn btn-success" type="submit">
          Submit
        </button>
      </div>
    </form>
  )
}

export default DownloadMusic
