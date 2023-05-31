import {useForm} from "react-hook-form"
import Field from "../../components/Field"
import {HiArrowDownTray} from "react-icons/hi2"
import {useNavigate} from "react-router"

type Inputs = {url: string}

type JsonError = {error: string}

const DownloadVideosPage: React.FC = () => {
  const navigate = useNavigate()
  const {
    handleSubmit,
    register,
    formState: {errors},
    setError,
  } = useForm<Inputs>()
  const onSubmit = async (values: Inputs) => {
    const response = await fetch(
      `/api/local/video/download?url=${encodeURIComponent(values.url)}`,
      {method: "POST"}
    )
    if (response.ok) {
      navigate("/local/videos")
    } else {
      const json = (await response.json()) as JsonError
      setError("url", {message: json.error})
    }
  }

  return (
    <>
      <form
        onSubmit={handleSubmit(onSubmit)}
        className="max-w-lg w-full self-center flex flex-col"
      >
        {errors.url && <span>Failed to download: {errors.url.message}</span>}
        <Field label="Video URL">
          <input
            {...register("url", {required: true})}
            className="input input-bordered"
            placeholder="Enter a video URL..."
            type="url"
          />
        </Field>

        <button type="submit" className="mt-4 btn btn-success self-end">
          <HiArrowDownTray className="mr-2" />
          Download
        </button>
      </form>
    </>
  )
}

export default DownloadVideosPage
