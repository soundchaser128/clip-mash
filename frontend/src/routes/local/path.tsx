import {HiCheck} from "react-icons/hi2"
import {useNavigate} from "react-router-dom"
import {useForm} from "react-hook-form"
import {useState} from "react"
import Loader from "../../components/Loader"
import {addNewVideos} from "../../api"

interface Inputs {
  path: string
  recurse: boolean
  fileName: string
}

export default function SelectVideos() {
  const navigate = useNavigate()
  const [submitting, setSubmitting] = useState(false)
  const {register, handleSubmit} = useForm<Inputs>({})

  const onSubmit = async (values: Inputs) => {
    setSubmitting(true)
    await addNewVideos({
      type: "local",
      path: values.path,
      recurse: values.recurse,
    })
    navigate("/library")
  }

  return (
    <>
      <form
        onSubmit={handleSubmit(onSubmit)}
        className="flex gap-4 items-start flex-col self-center"
      >
        {!submitting && (
          <>
            <div className="form-control">
              <label className="label">
                <span className="label-text">
                  Folder containing your videos
                </span>
              </label>
              <input
                required
                type="text"
                className="input input-bordered w-96"
                placeholder="C:\Users\CoolUser\Videos\DefinitelyNotPorn"
                {...register("path", {required: true, minLength: 3})}
              />
            </div>
            <div className="form-control justify-between w-full">
              <label className="label cursor-pointer">
                <span className="label-text">
                  Look at all the subdirectories as well
                </span>
                <input
                  type="checkbox"
                  className="toggle"
                  {...register("recurse")}
                />
              </label>
            </div>
            <button type="submit" className="btn btn-success self-end">
              <HiCheck className="mr-2" />
              Submit
            </button>
          </>
        )}

        {submitting && (
          <Loader>
            Scanning your videos and generating preview images...
            <br /> This might take a while, depending on how many videos you
            have.
          </Loader>
        )}
      </form>
    </>
  )
}
