import {ClipOrder} from "@/api"
import {useForm} from "react-hook-form"

interface Inputs {
  //   seed?: string
  splitClips: boolean
  //  clipDuration: number
  beatsPerMeasure: number
  measureCountType: "fixed" | "random"
  measureCountFixed: number
  measureCountRandomStart: number
  measureCountRandomEnd: number
}

const PmvSettingsForm: React.FC = () => {
  const {register, watch} = useForm<Inputs>()

  const doSplitClips = watch("splitClips")
  const measureCountType = watch("measureCountType")

  return (
    <form>
      <div className="form-control">
        <label className="label">
          <span className="label-text">Beats per measure</span>
        </label>
        <input
          type="number"
          className="input input-bordered"
          disabled={!doSplitClips}
          {...register("beatsPerMeasure", {valueAsNumber: true})}
        />
      </div>
      <div className="form-control">
        <label className="label">
          <span className="label-text">Cut after ... measures</span>
        </label>
        <select
          className="select select-bordered"
          {...register("measureCountType")}
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
            {...register("measureCountFixed", {valueAsNumber: true})}
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
              {...register("measureCountRandomStart", {
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
              {...register("measureCountRandomEnd", {valueAsNumber: true})}
            />
          </div>
        </>
      )}
    </form>
  )
}

export default PmvSettingsForm
