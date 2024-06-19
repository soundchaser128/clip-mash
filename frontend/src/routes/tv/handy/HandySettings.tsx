import {HandyPattern} from "@/api"
import Heading from "@/components/Heading"

import {useForm} from "react-hook-form"

interface Props {
  onSubmit: (pattern: HandyPattern) => void
}

const HandySettings: React.FC<Props> = ({onSubmit}) => {
  const {register, watch, handleSubmit} = useForm<HandyPattern>({})
  const type = watch("type")
  return (
    <form onSubmit={handleSubmit(onSubmit)}>
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

        <div className="form-control">
          <label className="label">
            <span className="label-text">Slide range</span>
          </label>

          <input
            type="range"
            className="range range-primary"
            min={0}
            max={100}
            {...register("parameters.slideRange.min", {valueAsNumber: true})}
          />
        </div>

        {type === "accellerate" && <></>}

        <button type="submit" className="btn btn-success mt-4 self-end">
          Submit
        </button>
      </div>
    </form>
  )
}

export default HandySettings
