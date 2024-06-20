import {HandyPattern} from "@/api"
import Heading from "@/components/Heading"
import {DeepPartial} from "@/types/types"
import {produce} from "immer"
import {useForm} from "react-hook-form"

interface Props {
  onSubmit: (pattern: HandyPattern) => void
}

export function prepareSettings(settings: HandyPattern): HandyPattern {
  return produce(settings, (draft) => {
    if ("sessionDuration" in draft.parameters) {
      draft.parameters.sessionDuration = draft.parameters.sessionDuration * 60
    }
  })
}

const defaultValues: DeepPartial<HandyPattern> = {
  type: "accellerate",
  parameters: {
    slideRange: {
      min: 0,
      max: 100,
    },
    sessionDuration: 10,
    startSpeed: 5,
    endSpeed: 80,
  },
}

const HandySettings: React.FC<Props> = ({onSubmit}) => {
  const {register, watch, handleSubmit} = useForm<HandyPattern>({
    defaultValues,
  })
  const type = watch("type")
  return (
    <form
      onSubmit={handleSubmit(onSubmit)}
      className="max-w-xl self-center w-full"
    >
      <Heading level={2}>Handy settings</Heading>
      <div className="form-control">
        <label className="label">
          <span className="label-text">Pattern type</span>
        </label>

        <select
          {...register("type")}
          className="select select-bordered select-primary"
        >
          <option value="random">Random</option>
          <option value="accellerate">Accelerate</option>
          <option value="cycle-accellerate">Accellerating cycle</option>
        </select>

        <div className="w-full gap-2">
          <label className="label">
            <span className="label-text">Slide range</span>
          </label>
          <div className="flex gap-2 items-center">
            <input
              className="input input-bordered"
              required
              type="number"
              min={0}
              max={100}
              {...register("parameters.slideRange.min", {
                valueAsNumber: true,
                required: true,
              })}
            />
            -
            <input
              className="input input-bordered"
              required
              type="number"
              min={0}
              max={100}
              {...register("parameters.slideRange.max", {
                valueAsNumber: true,
                required: true,
              })}
            />
          </div>
        </div>

        {type === "accellerate" && (
          <>
            <div className="form-control">
              <label className="label">
                <span className="label-text">Session duration in minutes</span>
              </label>

              <input
                className="input input-bordered"
                required
                type="number"
                min={1}
                {...register("parameters.sessionDuration", {
                  valueAsNumber: true,
                  required: true,
                })}
              />
            </div>

            <div className="form-control">
              <label className="label">
                <span className="label-text">Start speed</span>
              </label>

              <input
                className="input input-bordered"
                required
                type="number"
                min={1}
                {...register("parameters.startSpeed", {
                  valueAsNumber: true,
                  required: true,
                })}
              />
            </div>

            <div className="form-control">
              <label className="label">
                <span className="label-text">End speed</span>
              </label>

              <input
                className="input input-bordered"
                required
                type="number"
                min={1}
                {...register("parameters.endSpeed", {
                  valueAsNumber: true,
                  required: true,
                })}
              />
            </div>
          </>
        )}

        {type === "random" && (
          <>
            <div className="w-full gap-2">
              <label className="label">
                <span className="label-text">Speed range</span>
              </label>
              <div className="flex gap-2 items-center">
                <input
                  className="input input-bordered"
                  required
                  type="number"
                  min={0}
                  max={100}
                  {...register("parameters.speedRange.min", {
                    valueAsNumber: true,
                    required: true,
                  })}
                />
                -
                <input
                  className="input input-bordered"
                  required
                  type="number"
                  min={0}
                  max={100}
                  {...register("parameters.speedRange.max", {
                    valueAsNumber: true,
                    required: true,
                  })}
                />
              </div>
            </div>
          </>
        )}

        <button type="submit" className="btn btn-success mt-4 self-end">
          Submit
        </button>
      </div>
    </form>
  )
}

export default HandySettings
