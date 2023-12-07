import {useFormContext} from "react-hook-form"
import {ClipFormInputs} from "./ClipSettingsForm"
import {ClipDurationField} from "./common"

const EqualLengthFields: React.FC<{totalClipDuration: number}> = ({
  totalClipDuration,
}) => {
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

      <ClipDurationField totalClipDuration={totalClipDuration} />
    </>
  )
}

export default EqualLengthFields
