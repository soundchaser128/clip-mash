import {useStateMachine} from "little-state-machine"
import {useForm} from "react-hook-form"
import {useNavigate} from "react-router-dom"
import {
  FormStage,
  LocalFilesFormStage,
  StashFormState,
  StateHelpers,
} from "../types/types"
import {updateForm} from "./actions"
import {HiChevronRight} from "react-icons/hi2"

type Inputs = Pick<StashFormState, "outputFps" | "outputResolution">

const defaultOptions: Inputs = {
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
    actions.updateForm({
      ...values,
      stage: StateHelpers.isLocalFiles(state.data)
        ? LocalFilesFormStage.PreviewClips
        : FormStage.PreviewClips,
    })
    navigate("/stash/clips")
  }

  return (
    <>
      <form className="grid grid-cols-3" onSubmit={handleSubmit(onSubmit)}>
        <div />
        <div className="flex flex-col gap-4 self-center max-w-lg">
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
              {...register("outputFps", {valueAsNumber: true})}
            />
          </div>
        </div>
        <div className="w-full flex justify-between mb-4">
          <span />
          <button type="submit" className="btn btn-success">
            Next
            <HiChevronRight className="ml-1" />
          </button>
        </div>
      </form>
    </>
  )
}

export default VideoOptions
