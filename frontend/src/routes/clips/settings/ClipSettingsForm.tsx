import {useStateMachine} from "little-state-machine"
import React, {useEffect} from "react"
import {useRevalidator} from "react-router-dom"
import {updateForm} from "../../actions"
import {
  HiArrowUturnLeft,
  HiArrowUturnRight,
  HiBackward,
  HiForward,
  HiRocketLaunch,
  HiTrash,
} from "react-icons/hi2"
import {FormProvider, useForm} from "react-hook-form"
import {Clip, ClipLengthOptions, ClipOrder} from "../../../api"
import {ClipStrategy} from "@/types/types"
import RoundRobinFields from "./RoundRobinFields"
import {FormState} from "@/types/form-state"
import WeightedRandomFields from "./WeightedRandomFields"
import EqualLengthFields from "./EqualLengthFields"
import MarkerOrderModal from "../MarkerOrderModal"

const initialValues = (state: FormState): ClipFormInputs =>
  state.clipOptions || {
    clipStrategy: "equalLength",
    equalLength: {
      clipDuration: 30,
    },
    clipOrder: {type: "scene"},
  }

interface CommonInputs {
  clipStrategy: ClipStrategy
  clipOrder: ClipOrder
  seed?: string
}

interface RoundRobinFormInputs {
  clipStrategy: "roundRobin"
  roundRobin: {
    clipLengths?: ClipLengthOptions
    useMusic?: boolean
  }
}

interface EqualLengthFormInputs {
  clipStrategy: "equalLength"
  equalLength: {
    clipDuration: number
  }
}

interface WeightedRandomFormInputs {
  clipStrategy: "weightedRandom"
  weightedRandom: {
    clipLengths: ClipLengthOptions
    weights: [string, number][]
    useMusic?: boolean
  }
}

interface NoSplitFormInputs {
  clipStrategy: "noSplit"
}

export type ClipFormInputs = CommonInputs &
  (
    | RoundRobinFormInputs
    | EqualLengthFormInputs
    | WeightedRandomFormInputs
    | NoSplitFormInputs
  )

interface SettingsFormProps {
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
  const {actions, state} = useStateMachine({updateForm})
  const formContext = useForm<ClipFormInputs>({
    defaultValues: initialValues(state.data),
  })
  const {register, watch, handleSubmit, setValue} = formContext
  const revalidator = useRevalidator()
  const clipStrategy = watch("clipStrategy")
  const clipOrder = watch("clipOrder.type")

  const onSubmit = (values: ClipFormInputs) => {
    if (
      confirmBeforeSubmit &&
      !window.confirm(
        "You have made manual changes to the clips, this would reset them. Are you sure you want to re-generate the clips?",
      )
    ) {
      return
    }
    actions.updateForm({clipOptions: values})
    revalidator.revalidate()
  }

  useEffect(() => {
    switch (clipStrategy) {
      case "equalLength":
        setValue("equalLength.clipDuration", 15)
        break
      case "roundRobin":
        setValue("roundRobin.clipLengths.baseDuration", 15)
        setValue("roundRobin.clipLengths.type", "randomized")
        break
      case "weightedRandom":
        setValue("weightedRandom.clipLengths.baseDuration", 15)
        setValue("weightedRandom.clipLengths.type", "randomized")
        break
    }
  }, [clipStrategy, setValue])

  return (
    <FormProvider {...formContext}>
      <form onSubmit={handleSubmit(onSubmit)} className="flex flex-col mb-4">
        <h2 className="text-xl font-bold">Settings</h2>
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
          <label className="label">
            <span className="label-text">Clip order</span>
          </label>
          <select
            className="select select-primary"
            {...register("clipOrder.type")}
          >
            <option value="">Select...</option>
            <option value="scene">Scene order</option>
            <option value="random">Random</option>
            <option value="fixed">Fixed</option>
          </select>
        </div>

        {clipOrder === "fixed" && <MarkerOrderModal />}

        <div className="form-control">
          <label className="label">
            <span className="label-text">Clip generation method</span>
          </label>
          <select
            className="select select-primary"
            {...register("clipStrategy")}
          >
            <option value="">Select...</option>
            <option value="roundRobin">Round-robin</option>
            <option value="weightedRandom">Weighted random</option>
            <option value="equalLength">Equal length</option>
            <option value="noSplit">No splitting</option>
          </select>
        </div>

        {clipStrategy === "roundRobin" && <RoundRobinFields />}
        {clipStrategy === "weightedRandom" && <WeightedRandomFields />}
        {clipStrategy === "equalLength" && <EqualLengthFields />}

        <div className="form-control">
          <label className="label">
            <span className="label-text">Random seed</span>
          </label>
          <input
            className="input input-bordered"
            type="text"
            placeholder="Enter a value to control random number generation (optional)"
            {...register("seed")}
          />
        </div>

        <div className="flex flex-row justify-between mt-4">
          <span />
          <button type="submit" className="btn btn-success ">
            <HiRocketLaunch />
            Generate
          </button>
        </div>
      </form>
    </FormProvider>
  )
}

export default ClipSettingsForm
