import {useFormContext} from "react-hook-form"
import {ClipFormInputs} from "./ClipSettingsForm"
import {useStateMachine} from "little-state-machine"
import {MusicFormFields, RandomizedLengthFormFields} from "./common"

const RoundRobinClipStrategyForm: React.FC<{
  totalClipDuration: number
}> = ({totalClipDuration}) => {
  const {state} = useStateMachine()
  const {watch} = useFormContext<ClipFormInputs>()
  const hasSongs = state.data.songs?.length || 0 > 0
  const useMusic = watch("useMusic")

  return (
    <>
      {useMusic && hasSongs ? (
        <MusicFormFields strategy="roundRobin" />
      ) : (
        <RandomizedLengthFormFields
          strategy="roundRobin"
          totalClipDuration={totalClipDuration}
        />
      )}
    </>
  )
}

export default RoundRobinClipStrategyForm
