import {useStateMachine} from "little-state-machine"
import React from "react"
import {useRevalidator} from "react-router-dom"
import {updateForm} from "../actions"
import {
  HiArrowUturnLeft,
  HiArrowUturnRight,
  HiBackward,
  HiCheck,
  HiForward,
  HiTrash,
} from "react-icons/hi2"
import {useForm} from "react-hook-form"
import {Clip, ClipOrder} from "../../api"
import WeightsModal from "./WeightsModal"
import MarkerOrderModal from "./MarkerOrderModal"

interface Inputs {
  clipOrder: ClipOrder
  seed?: string
  splitClips: boolean
  clipDuration: number
  beatsPerMeasure: number
  measureCountType: "fixed" | "random"
  measureCountFixed: number
  measureCountRandomStart: number
  measureCountRandomEnd: number
}

interface SettingsFormProps {
  initialValues: Inputs
  clips: Clip[]
  onRemoveClip: () => void
  onUndo: () => void
  onRedo: () => void
  canUndo: boolean
  canRedo: boolean
  onShiftClips: (direction: "left" | "right") => void
  canShiftLeft: boolean
  canShiftRight: boolean
  confirmBeforeSubmit: boolean
}

const ClipSettingsForm: React.FC<SettingsFormProps> = ({
  initialValues,
  clips,
  onRemoveClip,
  onUndo,
  onRedo,
  canUndo,
  canRedo,
  onShiftClips,
  canShiftLeft,
  canShiftRight,
  confirmBeforeSubmit,
}) => {
  const {register, handleSubmit, watch} = useForm<Inputs>({
    defaultValues: initialValues,
  })
  const doSplitClips = watch("splitClips")
  const measureCountType = watch("measureCountType")

  const revalidator = useRevalidator()
  const {actions, state} = useStateMachine({updateForm})
  const isPmv = state.data.songs?.length !== 0
  const clipOrderType = watch("clipOrder.type")

  const onSubmit = (values: Inputs) => {
    if (
      confirmBeforeSubmit &&
      !window.confirm(
        "You have made manual changes to the clips, this would reset them. Are you sure you want to re-generate the clips?",
      )
    ) {
      return
    }
    actions.updateForm({
      clipDuration: values.clipDuration,
      clipOrder: state.data.clipOrder,
      splitClips: values.splitClips,
      seed: values.seed,
      beatsPerMeasure: values.beatsPerMeasure,
      cutAfterMeasures:
        values.measureCountType === "fixed"
          ? {type: "fixed", count: values.measureCountFixed}
          : {
              type: "random",
              min: values.measureCountRandomStart,
              max: values.measureCountRandomEnd,
            },
    })

    revalidator.revalidate()
  }

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="flex flex-col mb-4">
      <h2 className="text-xl font-bold">Settings</h2>
      {!isPmv && (
        <>
          <div className="w-full flex justify-between mb-4 mt-2">
            <div className="join">
              <div className="tooltip" data-tip="Undo">
                <button
                  disabled={!canUndo}
                  onClick={onUndo}
                  type="button"
                  className="join-item btn btn-sm btn-ghost btn-square"
                >
                  <HiArrowUturnLeft />
                </button>
              </div>
              <div className="tooltip" data-tip="Redo">
                <button
                  disabled={!canRedo}
                  onClick={onRedo}
                  type="button"
                  className="join-item btn btn-sm btn-ghost btn-square"
                >
                  <HiArrowUturnRight />
                </button>
              </div>
            </div>

            <div className="flex gap-2">
              <div className="join">
                <div className="tooltip" data-tip="Move clip left">
                  <button
                    disabled={!canShiftLeft}
                    onClick={() => onShiftClips("left")}
                    className="btn btn-sm btn-ghost join-item"
                    type="button"
                  >
                    <HiBackward />
                  </button>
                </div>
                <div className="tooltip" data-tip="Move clip right">
                  <button
                    disabled={!canShiftRight}
                    onClick={() => onShiftClips("right")}
                    className="btn btn-sm btn-ghost join-item"
                    type="button"
                  >
                    <HiForward />
                  </button>
                </div>
              </div>
              <div className="tooltip" data-tip="Delete current clip">
                <button
                  onClick={onRemoveClip}
                  type="button"
                  className="btn btn-sm btn-error"
                >
                  <HiTrash />
                </button>
              </div>
            </div>
          </div>
          <div className="form-control">
            <label className="label cursor-pointer">
              <span className="label-text mr-2">
                Split up marker videos into clips
              </span>
              <input
                type="checkbox"
                className="toggle"
                {...register("splitClips")}
              />
            </label>
          </div>
          <div className="form-control">
            <label className="label">
              <span className="label-text">
                Maximum duration per clip (in seconds):
              </span>
            </label>
            <input
              type="number"
              className="input input-bordered"
              disabled={!doSplitClips}
              {...register("clipDuration", {valueAsNumber: true})}
            />
          </div>
        </>
      )}

      {isPmv && (
        <>
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
        </>
      )}

      <div className="form-control">
        <label className="label">
          <span className="label-text">Clip order:</span>
        </label>
        <select
          className="select select-bordered"
          {...register("clipOrder.type")}
        >
          <option disabled value="none">
            Select clip ordering
          </option>
          <option value="scene">Scene order</option>
          <option value="fixed">Fixed</option>
          <option value="random">Random</option>
        </select>
      </div>

      <div className="form-control">
        <label className="label">
          <span className="label-text">Random seed:</span>
        </label>
        <input
          type="text"
          className="input input-bordered"
          placeholder="Enter a value to control random number generation (optional)"
          {...register("seed")}
        />
      </div>
      {clipOrderType === "fixed" && <MarkerOrderModal />}
      <div className="flex w-full justify-between items-center mt-4">
        <WeightsModal clips={clips} />

        <button type="submit" className="btn btn-primary">
          <HiCheck className="mr-2" />
          Apply
        </button>
      </div>
    </form>
  )
}

export default ClipSettingsForm
