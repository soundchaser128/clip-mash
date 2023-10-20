import {useFormContext} from "react-hook-form"
import {ClipFormInputs} from "./ClipSettingsForm"
import {useStateMachine} from "little-state-machine"
import {MusicFormFields, RandomizedLengthFormFields} from "./common"

const RoundRobinClipStrategyForm: React.FC = () => {
  const {state} = useStateMachine()
  const {register, watch} = useFormContext<ClipFormInputs>()
  const hasSongs = state.data.songs?.length || 0 > 0
  const useMusic = watch("roundRobin.useMusic")

  return (
    <>
      {hasSongs && (
        <div className="form-control">
          <label className="label">
            <span className="label-text">Use music for clip generation?</span>
            <input
              type="checkbox"
              className="checkbox"
              {...register("roundRobin.useMusic")}
            />
          </label>
        </div>
      )}
      {useMusic && hasSongs ? (
        <MusicFormFields strategy="roundRobin" />
      ) : (
        <RandomizedLengthFormFields strategy="roundRobin" />
      )}
    </>
  )
}

export default RoundRobinClipStrategyForm
