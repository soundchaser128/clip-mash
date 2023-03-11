import {GlobalState} from "little-state-machine"
import {nanoid} from "nanoid"
import {FormStage, FormState} from "../types/types"

export function updateForm(
  state: GlobalState,
  newState: Partial<FormState>
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
      stage: FormStage.SelectMode,
      id: nanoid(8),
    },
  }
}
