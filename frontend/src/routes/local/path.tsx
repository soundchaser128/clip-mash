import {HiCheck} from "react-icons/hi2"
import {StateHelpers} from "../../types/types"
import {useStateMachine} from "little-state-machine"
import invariant from "tiny-invariant"
import {useNavigate} from "react-router-dom"
import {useForm} from "react-hook-form"
import {useState} from "react"

interface Inputs {
  path: string
  recurse: boolean
  fileName: string
}

export default function SelectVideos() {
  const {state} = useStateMachine()
  invariant(StateHelpers.isLocalFiles(state.data))
  const navigate = useNavigate()
  const [submitting, setSubmitting] = useState(false)

  const {register, handleSubmit} = useForm<Inputs>({})

  const onSubmit = async (values: Inputs) => {
    setSubmitting(true)
    const response = await fetch("/api/local/video", {
      method: "POST",
      body: JSON.stringify({
        path: values.path,
        recurse: values.recurse,
      }),
      headers: {
        "Content-Type": "application/json",
      },
    })
    if (response.ok) {
      navigate("/local/videos")
    } else {
      const text = await response.text()
      console.error("request failed", text)
    }
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
          <div className="self-center flex gap-4 items-center">
            <span className="loading loading-ring loading-lg" />
            <p className="text-sm">
              Scanning your videos...
              <br /> This might take a while, depending on how many videos you
              have.
            </p>
          </div>
        )}
      </form>
    </>
  )
}
