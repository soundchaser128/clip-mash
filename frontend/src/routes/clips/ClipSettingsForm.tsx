import {useStateMachine} from "little-state-machine"
import React from "react"
import {useRevalidator} from "react-router-dom"
import {updateForm} from "../actions"
import {HiRocketLaunch} from "react-icons/hi2"
import {FormProvider, useForm} from "react-hook-form"
import {Clip, ClipLengthOptions, ClipPickerOptions} from "../../api"
import {ClipStrategy} from "@/types/types"
import RoundRobinClipStrategyForm from "./settings/RoundRobinClipStrategyForm"
import {FormState} from "@/types/form-state"

const initialValues = (state: FormState): ClipFormInputs => ({})

export interface ClipFormInputs {
  seed?: string
  clipStrategy?: ClipStrategy
  roundRobin?: {
    clipLengths: ClipLengthOptions
    useMusic?: boolean
  }

  weightedRandom?: {
    clipLengths: ClipLengthOptions
    // todo
    weights: any
  }
}

interface SettingsFormProps {
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
  const {actions, state} = useStateMachine({updateForm})
  const formContext = useForm<ClipFormInputs>({
    defaultValues: initialValues(state.data),
  })
  const {register, watch, handleSubmit} = formContext
  const revalidator = useRevalidator()
  const clipStrategy = watch("clipStrategy")

  const onSubmit = (values: ClipFormInputs) => {
    if (
      confirmBeforeSubmit &&
      !window.confirm(
        "You have made manual changes to the clips, this would reset them. Are you sure you want to re-generate the clips?",
      )
    ) {
      return
    }

    actions.updateForm({
      clipOptions: values,
    })

    revalidator.revalidate()
  }

  return (
    <FormProvider {...formContext}>
      <form onSubmit={handleSubmit(onSubmit)} className="flex flex-col mb-4">
        <h2 className="text-xl font-bold">Settings</h2>
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

        {clipStrategy === "roundRobin" && <RoundRobinClipStrategyForm />}
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
