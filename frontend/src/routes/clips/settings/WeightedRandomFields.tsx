import {useLoaderData} from "react-router-dom"
import WeightsModal from "./WeightsModal"
import {ClipsLoaderData} from "@/routes/loaders"
import {useStateMachine} from "little-state-machine"
import {useFormContext} from "react-hook-form"
import {ClipFormInputs} from "./ClipSettingsForm"
import {MusicFormFields, RandomizedLengthFormFields} from "./common"

const WeightedRandomFields: React.FC = () => {
  const data = useLoaderData() as ClipsLoaderData
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
        <MusicFormFields strategy="weightedRandom" />
      ) : (
        <RandomizedLengthFormFields strategy="weightedRandom" />
      )}

      <WeightsModal className="mt-4" clips={data.clips} />
    </>
  )
}

export default WeightedRandomFields
