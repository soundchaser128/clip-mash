import {useStateMachine} from "little-state-machine"
import {useForm} from "react-hook-form"
import {LoaderFunction, useLoaderData, useNavigate} from "react-router-dom"
import {FormStage, FormState} from "@/types/form-state"
import {updateForm} from "./actions"
import {
  HiArrowsRightLeft,
  HiChevronRight,
  HiInformationCircle,
} from "react-icons/hi2"
import {videosNeedEncoding} from "@/api"
import {getFormState} from "@/helpers/form"

export const videoOptionsLoader: LoaderFunction = async () => {
  const formState = getFormState()!
  const needsEncode = await videosNeedEncoding(formState.videoIds || [])
  return needsEncode
}

type Inputs = Pick<
  FormState,
  "outputFps" | "videoCodec" | "videoQuality" | "encodingEffort" | "padding"
> & {outputWidth: number; outputHeight: number}

const defaultOptions: Inputs = {
  outputFps: 30,
  videoCodec: "h264",
  videoQuality: "medium",
  encodingEffort: "medium",
  outputWidth: 1280,
  outputHeight: 720,
}

function VideoOptions() {
  const needsEncode = useLoaderData() as boolean

  const {actions, state} = useStateMachine({updateForm})
  const navigate = useNavigate()
  const {register, handleSubmit, setValue, watch} = useForm<Inputs>({
    defaultValues: {...defaultOptions, ...state.data},
  })

  const width = watch("outputWidth")
  const height = watch("outputHeight")

  const onSubmit = (values: Inputs) => {
    actions.updateForm({
      ...values,
      stage: FormStage.PreviewClips,
      outputResolution: [values.outputWidth, values.outputHeight],
    })
    navigate("/clips")
  }

  const onSwapResolutionValues = () => {
    setValue("outputWidth", height)
    setValue("outputHeight", width)
  }

  return (
    <>
      <form className="grid grid-cols-3" onSubmit={handleSubmit(onSubmit)}>
        <div />
        {!needsEncode && (
          <div className="place-self-center alert alert-info">
            <HiInformationCircle /> All videos have the same encoding
            parameters, no encoding will be performed.
          </div>
        )}
        {needsEncode && (
          <div className="flex flex-col gap-4 self-center max-w-lg">
            <div className="form-control w-full">
              <label className="label">
                <span className="label-text">Output resolution</span>
              </label>
              <div className="flex items-center gap-2">
                <input
                  type="number"
                  className="input input-bordered w-1/2"
                  {...register("outputWidth", {valueAsNumber: true})}
                />
                <span>x</span>
                <input
                  type="number"
                  className="input input-bordered w-1/2"
                  {...register("outputHeight", {valueAsNumber: true})}
                />
                <button
                  onClick={onSwapResolutionValues}
                  type="button"
                  className="btn btn-square btn-sm"
                >
                  <HiArrowsRightLeft />
                </button>
              </div>
            </div>

            <div className="form-control">
              <label className="label">
                <span className="label-text">Output frames per second</span>
              </label>
              <input
                type="number"
                placeholder="Type here"
                className="input input-bordered"
                {...register("outputFps", {valueAsNumber: true})}
              />
            </div>
            <div className="form-control">
              <label className="label">
                <span className="label-text">Video codec</span>
              </label>
              <select
                className="select select-bordered"
                {...register("videoCodec")}
              >
                <option disabled value="none">
                  Select codec
                </option>
                <option value="h264">
                  H.264 (most common, quick to encode)
                </option>
                <option value="h265">
                  H.265 (more efficient, slower to encode)
                </option>
                <option value="av1">
                  AV1 (even more efficient, slower to encode)
                </option>
              </select>
            </div>

            <div className="form-control">
              <label className="label">
                <span className="label-text">Video quality</span>
              </label>
              <select
                className="select select-bordered"
                {...register("videoQuality")}
              >
                <option disabled value="none">
                  Select quality
                </option>
                <option value="low">Low</option>
                <option value="medium">Medium</option>
                <option value="high">High</option>
                <option value="lossless">Almost lossless</option>
              </select>
            </div>

            <div className="form-control">
              <label className="label">
                <span className="label-text">Encoding effort</span>
              </label>
              <select
                className="select select-bordered"
                {...register("encodingEffort")}
              >
                <option disabled value="none">
                  Select encoding effort
                </option>
                <option value="low">Low</option>
                <option value="medium">Medium</option>
                <option value="high">High</option>
              </select>
            </div>

            <div className="form-control">
              <label className="label">
                <span className="label-text">
                  Padding for videos with different aspect ratios
                </span>
              </label>
              <select
                className="select select-bordered"
                {...register("padding")}
              >
                <option disabled value="none">
                  Select padding
                </option>
                <option value="black">Black</option>
                <option value="blur">
                  Blurred video content (experimental)
                </option>
              </select>
            </div>
          </div>
        )}

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
