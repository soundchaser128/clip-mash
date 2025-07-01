import {HandyPattern} from "@/api"
import Heading from "@/components/Heading"
import {DeepPartial} from "@/types/types"
import {produce} from "immer"
import {UseFormRegister, useForm} from "react-hook-form"
import {HiCheck} from "react-icons/hi2"

interface Props {
  onSubmit: (pattern: HandyPattern) => void
}

interface RangeInputProps {
  label: string
  name: string
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  register: UseFormRegister<any>
}

const RangeInput: React.FC<RangeInputProps> = ({label, name, register}) => {
  return (
    <>
      <div className="w-full gap-2">
        <label className="label">
          <span className="label-text">{label}</span>
        </label>
        <div className="flex gap-2 items-center">
          <input
            className="input input-bordered"
            required
            type="number"
            min={0}
            max={100}
            {...register(`${name}.min`, {
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
            {...register(`${name}.max`, {
              valueAsNumber: true,
              required: true,
            })}
          />
        </div>
      </div>
    </>
  )
}

export function prepareSettings(settings: HandyPattern): HandyPattern {
  return produce(settings, (draft) => {
    if ("sessionDuration" in draft.parameters) {
      draft.parameters.sessionDuration = draft.parameters.sessionDuration * 60
    }

    if ("cycleDuration" in draft.parameters) {
      draft.parameters.cycleDuration = draft.parameters.cycleDuration * 60
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
    // @ts-expect-error intentionally provide all values
    speedRange: {
      min: 10,
      max: 50,
    },
    intervalRange: {
      min: 5,
      max: 15,
    },
    jitter: 10,
    sessionDuration: 10,
    cycleDuration: 1,
    startSpeed: 5,
    endSpeed: 80,
    startRange: {
      min: 5,
      max: 15,
    },
    endRange: {
      min: 40,
      max: 70,
    },
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
      className="max-w-xl self-center w-full flex flex-col"
    >
      <Heading level={2}>Handy settings</Heading>
      <p className="text-sm mb-2">
        Set up the pattern for the Handy to follow.
      </p>
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
      </div>
      <p className="mt-1 mb-2 text-sm">
        {type === "accellerate" &&
          "The speed will increase over the selected duration from the start speed to the end speed."}
        {type === "random" &&
          "The speed will change randomly between the two selected values. The amount of randomness adjusts how large each change can be."}
        {type === "cycle-accellerate" &&
          "The speed starts out cycling between the two selected start range values and will gradually increase over the selected session duration until it reaches the end range."}
      </p>

      <RangeInput
        label="Slide range"
        name="parameters.slideRange"
        register={register}
      />

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
          <RangeInput
            label="Speed range"
            name="parameters.speedRange"
            register={register}
          />

          <div className="form-control">
            <label className="label">
              <span className="label-text">Randomness</span>
            </label>

            <input
              className="input input-bordered"
              required
              type="number"
              min={1}
              {...register("parameters.jitter", {
                valueAsNumber: true,
                required: true,
              })}
            />
          </div>

          <RangeInput
            label="Time between speed changes (in seconds)"
            name="parameters.intervalRange"
            register={register}
          />
        </>
      )}

      {type === "cycle-accellerate" && (
        <>
          <RangeInput
            label="Start speed range"
            name="parameters.startRange"
            register={register}
          />

          <RangeInput
            label="End speed range"
            name="parameters.endRange"
            register={register}
          />

          <div className="form-control">
            <label className="label">
              <span className="label-text">Session duration (in minutes)</span>
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
              <span className="label-text">Cycle duration (in minutes)</span>
            </label>

            <input
              className="input input-bordered"
              required
              type="number"
              min={1}
              {...register("parameters.cycleDuration", {
                valueAsNumber: true,
                required: true,
              })}
            />
          </div>
        </>
      )}

      <button type="submit" className="btn btn-success mt-4 self-end">
        <HiCheck />
        Submit
      </button>
    </form>
  )
}

export default HandySettings
