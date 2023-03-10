import {useStateMachine} from "little-state-machine"
import {useForm} from "react-hook-form"
import {useNavigate} from "react-router-dom"
import {FormStage, FormState} from "../types/types"
import {updateForm} from "./actions"

type Inputs = Pick<
  FormState,
  "clipDuration" | "clipOrder" | "outputFps" | "outputResolution"
>

const defaultOptions: Inputs = {
  clipDuration: 15,
  clipOrder: "random",
  outputFps: 30,
  outputResolution: "720",
}

function VideoOptions() {
  const {actions, state} = useStateMachine({updateForm})
  const navigate = useNavigate()
  const {register, handleSubmit} = useForm<Inputs>({
    defaultValues: {...defaultOptions, ...state.data},
  })

  const onSubmit = (values: Inputs) => {
    actions.updateForm({...values, stage: FormStage.Wait})
    navigate("/progress")
  }

  return (
    <>
      <form onSubmit={handleSubmit(onSubmit)}>
        <div className="w-full flex justify-between mb-4">
          <span />
          <button type="submit" className="btn btn-success">
            Next
          </button>
        </div>
        <div className="flex flex-col gap-4 max-w-sm w-full">
          <div className="form-control">
            <label className="label">
              <span className="label-text">
                Maximum duration per clip (in seconds):
              </span>
            </label>
            <input
              type="number"
              placeholder="Type here"
              className="input input-bordered"
              {...register("clipDuration")}
            />
          </div>

          <div className="form-control">
            <label className="label">
              <span className="label-text">Output resolution:</span>
            </label>
            <select
              className="select select-bordered"
              {...register("outputResolution")}
            >
              <option disabled value="none">
                Select resolution
              </option>
              <option value="720">1280x720</option>
              <option value="1080">1920x1080</option>
              <option value="4K">3840x2160</option>
            </select>
          </div>

          <div className="form-control">
            <label className="label">
              <span className="label-text">Output frames per second:</span>
            </label>
            <input
              type="number"
              placeholder="Type here"
              className="input input-bordered"
              min="30"
              max="120"
              {...register("outputFps")}
            />
          </div>

          <div className="form-control">
            <label className="label">
              <span className="label-text">Clip order:</span>
            </label>
            <select
              className="select select-bordered"
              {...register("clipOrder")}
            >
              <option disabled value="none">
                Select clip ordering
              </option>
              <option value="random">Random</option>
              <option value="scene-order">Sccene order`</option>
            </select>
          </div>
        </div>
      </form>
    </>
  )
}

export default VideoOptions
