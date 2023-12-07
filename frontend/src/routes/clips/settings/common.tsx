import {useFormContext} from "react-hook-form"
import {ClipFormInputs} from "./ClipSettingsForm"
import React from "react"
import {formatSeconds} from "@/helpers"

export const ClipDurationField: React.FC<{
  totalClipDuration: number
}> = ({totalClipDuration}) => {
  const {register, watch} = useFormContext<ClipFormInputs>()
  const currentDuration = watch("maxDuration")

  return (
    <div className="form-field">
      <label className="label">
        <span className="label-text">Compilation length</span>
      </label>
      <input
        type="range"
        className="range range-primary w-full"
        min={0}
        max={totalClipDuration}
        required
        {...register("maxDuration", {
          valueAsNumber: true,
        })}
      />
      <div className="text-xs text-center">
        {formatSeconds(currentDuration, "short")} /{" "}
        {formatSeconds(totalClipDuration, "short")}
      </div>
    </div>
  )
}

export const MusicFormFields: React.FC<{
  strategy: "roundRobin" | "weightedRandom"
}> = ({strategy}) => {
  const {register, watch} = useFormContext<ClipFormInputs>()
  const measureCountType = watch(
    `${strategy}.clipLengths.cutAfterMeasures.type`,
  )

  return (
    <>
      <div className="form-control">
        <label className="label">
          <span className="label-text">Beats per measure</span>
        </label>
        <input
          type="hidden"
          {...register(`${strategy}.clipLengths.type`, {value: "songs"})}
        />
        <input
          type="number"
          className="input input-bordered"
          required
          {...register(`${strategy}.clipLengths.beatsPerMeasure`, {
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
          {...register(`${strategy}.clipLengths.cutAfterMeasures.type`)}
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
            required
            {...register(`${strategy}.clipLengths.cutAfterMeasures.count`, {
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
              required
              {...register(`${strategy}.clipLengths.cutAfterMeasures.min`, {
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
              required
              {...register(`${strategy}.clipLengths.cutAfterMeasures.max`, {
                valueAsNumber: true,
              })}
            />
          </div>
        </>
      )}
    </>
  )
}

export const RandomizedLengthFormFields: React.FC<{
  strategy: "roundRobin" | "weightedRandom"
  totalClipDuration: number
}> = ({strategy, totalClipDuration}) => {
  const {register} = useFormContext<ClipFormInputs>()
  return (
    <>
      <input
        type="hidden"
        {...register(`${strategy}.clipLengths.type`, {value: "randomized"})}
      />
      <div className="form-field">
        <label className="label">
          <span className="label-text">Maximum clip length (seconds)</span>
        </label>
        <input
          type="number"
          className="input input-bordered w-full"
          required
          {...register(`${strategy}.clipLengths.baseDuration`, {
            valueAsNumber: true,
          })}
        />
      </div>

      <ClipDurationField totalClipDuration={totalClipDuration} />
    </>
  )
}
