import {useFormContext} from "react-hook-form"
import {ClipFormInputs} from "./ClipSettingsForm"

const EqualLengthFields: React.FC = () => {
  const {register} = useFormContext<ClipFormInputs>()
  return (
    <>
      <div className="form-control">
        <label className="label">
          <span className="label-text">Clip duration (seconds)</span>
        </label>
        <input
          type="number"
          className="input input-bordered"
          {...register("equalLength.clipDuration", {valueAsNumber: true})}
        />
      </div>
    </>
  )
}

export default EqualLengthFields
