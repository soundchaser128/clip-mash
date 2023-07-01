import {useForm} from "react-hook-form"
import Field from "../../components/Field"
import {HiArrowDownTray, HiXMark} from "react-icons/hi2"
import {useNavigate} from "react-router"
import useNotification from "../../hooks/useNotification"
import Loader from "../../components/Loader"
import {useState} from "react"

type Inputs = {urls: string}

type JsonError = {error: string}

const splitRegex = /\s+|\n/

const validateUrl = (url: string): string | undefined => {
  try {
    new URL(url)
    return undefined
  } catch (err: unknown) {
    return (err as Error).message
  }
}

const DownloadVideosPage: React.FC = () => {
  const navigate = useNavigate()
  const {
    handleSubmit,
    register,
    formState: {errors, isSubmitting},
    setError,
    clearErrors,
  } = useForm<Inputs>()
  const sendNotification = useNotification()
  const [videoCount, setVideoCount] = useState<number>()

  const onSubmit = async (values: Inputs) => {
    const urls = values.urls.split(splitRegex).map((res) => res.trim())
    setVideoCount(urls.length)
    const errors = []
    for (const url of urls) {
      const validationError = validateUrl(url)
      if (validationError) {
        errors.push(`Invalid URL ${url}: ${validationError}`)
      }
    }
    if (errors.length > 0) {
      setError("urls", {message: errors.join("\n")})
    } else {
      const promises = urls.map((url) => {
        return fetch(
          `/api/local/video/download?url=${encodeURIComponent(url)}`,
          {method: "POST"}
        )
      })
      const responses = await Promise.all(promises)
      const errors = []
      for (const response of responses) {
        if (!response.ok) {
          const json = (await response.json()) as JsonError
          errors.push(json.error)
        }
      }
      if (errors.length > 0) {
        setError("urls", {message: errors.join("\n")})
      } else {
        sendNotification("Success", "Video downloads finished!")
        navigate("/local/videos")
      }
    }
  }

  if (isSubmitting) {
    return <Loader>Downloading {videoCount} videos.</Loader>
  }

  return (
    <>
      {errors.urls && (
        <div className="bg-error p-4 shadow-xl rounded-xl text-error-content max-w-2xl self-center mb-4 relative">
          <HiXMark
            className="absolute top-2 right-2 w-6 h-6 cursor-pointer"
            onClick={() => clearErrors("urls")}
          />
          <p className="font-bold">Some downloads failed:</p>
          <code>{errors.urls.message}</code>
        </div>
      )}
      <form
        onSubmit={handleSubmit(onSubmit)}
        className="max-w-lg w-full self-center flex flex-col"
      >
        <p className="mb-4 text-gray-500">
          You can download videos from YouTube, Vimeo or any other site that
          yt-dlp supports.
        </p>
        <Field label="Video URLs">
          <textarea
            className="textarea textarea-bordered"
            placeholder="Enter URLs separated by line breaks..."
            rows={5}
            {...register(`urls`)}
          />
        </Field>

        <button
          type="submit"
          disabled={isSubmitting}
          className="mt-4 btn btn-success self-end"
        >
          <HiArrowDownTray className="mr-2" />
          Download
        </button>
      </form>
    </>
  )
}

export default DownloadVideosPage
