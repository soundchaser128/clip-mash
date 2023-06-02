import {useFieldArray, useForm} from "react-hook-form"
import Field from "../../components/Field"
import {HiArrowDownTray} from "react-icons/hi2"
import {useNavigate} from "react-router"

type Inputs = {urls: string[]}

type JsonError = {error: string}

const DownloadVideosPage: React.FC = () => {
  const navigate = useNavigate()
  const {
    handleSubmit,
    register,
    formState: {errors, isSubmitting},
    setError,
    control,
  } = useForm<Inputs>({defaultValues: {urls: ["hello", "world"]}})
  const {fields, append, prepend, remove, swap, move, insert} =
    useFieldArray<Inputs>({
      control,
      name: "urls",
    })

  const onSubmit = async (values: Inputs) => {
    // const response = await fetch(
    //   `/api/local/video/download?url=${encodeURIComponent(values.url)}`,
    //   {method: "POST"}
    // )
    // if (response.ok) {
    //   navigate("/local/videos")
    // } else {
    //   const json = (await response.json()) as JsonError
    //   setError("url", {message: json.error})
    // }
  }

  return (
    <>
      <form
        onSubmit={handleSubmit(onSubmit)}
        className="max-w-lg w-full self-center flex flex-col"
      >
        {errors.urls && <span>Failed to download: {errors.urls.message}</span>}

        <Field label="Video URL">
          {fields.map((field, index) => (
            <input
              key={field.id}
              className="input input-bordered"
              {...register(`urls.${index}` as const)}
            />
          ))}
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
