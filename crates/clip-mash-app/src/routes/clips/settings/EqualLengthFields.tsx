import {useFormContext} from "react-hook-form"
import type {ClipFormInputs} from "./ClipSettingsForm"
import {
  ClipDurationSpreadField,
  CompilationDurationField,
  MinClipDurationField,
} from "./common"

const EqualLengthFields: React.FC<{totalClipDuration: number}> = ({
  totalClipDuration,
}) => {
  const {register} = useFormContext<ClipFormInputs>()

  return (
    <>
      <div className="fieldset">
        <label className="label">
          <span className="text-sm">Base clip duration (seconds)</span>
        </label>
        <input
          type="number"
          className="input"
          {...register("equalLength.clipDuration", {valueAsNumber: true})}
        />
      </div>

      <CompilationDurationField totalClipDuration={totalClipDuration} />
      <MinClipDurationField />
      <ClipDurationSpreadField />
    </>
  )
}

export default EqualLengthFields
