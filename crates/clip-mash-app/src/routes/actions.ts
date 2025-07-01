import {GlobalState} from "little-state-machine"
import {FormStage, FormState} from "../types/form-state"

export function updateForm(
  state: GlobalState,
  newState: Partial<FormState>,
): GlobalState {
  return {
    data: {
      ...state.data,
      ...newState,
    },
  }
}

export function resetForm(): GlobalState {
  return {
    data: {
      stage: FormStage.Start,
    },
  }
}
