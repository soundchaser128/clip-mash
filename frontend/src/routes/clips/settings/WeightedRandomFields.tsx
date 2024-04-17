import {useLoaderData} from "react-router-dom"
import WeightsModal from "./WeightsModal"
import {ClipsLoaderData} from "@/routes/loaders"
import {useStateMachine} from "little-state-machine"
import {useFormContext} from "react-hook-form"
import {ClipFormInputs} from "./ClipSettingsForm"
import {MusicFormFields, RandomizedLengthFormFields} from "./common"

const WeightedRandomFields: React.FC<{
  totalClipDuration: number
}> = ({totalClipDuration}) => {
  const data = useLoaderData() as ClipsLoaderData
  const {state} = useStateMachine()
  const {watch} = useFormContext<ClipFormInputs>()
  const hasSongs = state.data.songs?.length || 0 > 0
  const useMusic = watch("useMusic")

  return (
    <>
      {useMusic && hasSongs ? (
        <MusicFormFields strategy="weightedRandom" />
      ) : (
        <RandomizedLengthFormFields
          strategy="weightedRandom"
          totalClipDuration={totalClipDuration}
        />
      )}

      <WeightsModal className="mt-4" clips={data.clips} />
    </>
  )
}

export default WeightedRandomFields
