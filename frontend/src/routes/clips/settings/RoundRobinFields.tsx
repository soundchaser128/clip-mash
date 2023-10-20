import {useFormContext} from "react-hook-form"
import {ClipFormInputs} from "./ClipSettingsForm"
import {useStateMachine} from "little-state-machine"

const MusicFormFields = () => {
  const {register, watch} = useFormContext<ClipFormInputs>()
  const measureCountType = watch("roundRobin.clipLengths.cutAfterMeasures.type")

  return (
    <>
      <div className="form-control">
        <label className="label">
          <span className="label-text">Beats per measure</span>
        </label>
        <input
          type="hidden"
          {...register("roundRobin.clipLengths.type", {value: "songs"})}
        />
        <input
          type="number"
          className="input input-bordered"
          {...register("roundRobin.clipLengths.beatsPerMeasure", {
            valueAsNumber: true,
          })}
        />
      </div>
      <div className="form-control">
        <label className="label">
          <span className="label-text">Cut after ... measures</span>
        </label>
        <select
          className="select select-bordered"
          {...register("roundRobin.clipLengths.cutAfterMeasures.type")}
        >
          <option disabled value="none">
            Select how to cut...
          </option>
          <option value="random">Randomized</option>
          <option value="fixed">Fixed</option>
        </select>
      </div>

      {measureCountType === "fixed" && (
        <div className="form-control">
          <label className="label cursor-pointer">
            <span className="label-text">Cut after how many measures?</span>
          </label>
          <input
            type="number"
            className="input input-bordered"
            {...register("roundRobin.clipLengths.cutAfterMeasures.count", {
              valueAsNumber: true,
            })}
          />
        </div>
      )}

      {measureCountType === "random" && (
        <>
          <div className="form-control">
            <label className="label cursor-pointer">
              <span className="label-text">Minimum</span>
            </label>
            <input
              type="number"
              className="input input-bordered"
              {...register("roundRobin.clipLengths.cutAfterMeasures.min", {
                valueAsNumber: true,
              })}
            />
          </div>
          <div className="form-control">
            <label className="label cursor-pointer">
              <span className="label-text">Maximum</span>
            </label>
            <input
              type="number"
              className="input input-bordered"
              {...register("roundRobin.clipLengths.cutAfterMeasures.max", {
                valueAsNumber: true,
              })}
            />
          </div>
        </>
      )}
    </>
  )
}

const RandomizedLengthFormFields: React.FC = () => {
  const {register, watch} = useFormContext<ClipFormInputs>()
  return (
    <>
      <input
        type="hidden"
        {...register("roundRobin.clipLengths.type", {value: "randomized"})}
      />
      <div className="form-field">
        <label className="label">
          <span className="label-text">Maximum clip length</span>
        </label>
        <input
          type="number"
          className="input input-bordered w-full"
          {...register("roundRobin.clipLengths.baseDuration", {
            valueAsNumber: true,
          })}
        />
      </div>
    </>
  )
}

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
        <MusicFormFields />
      ) : (
        <RandomizedLengthFormFields />
      )}
    </>
  )
}

export default RoundRobinClipStrategyForm
