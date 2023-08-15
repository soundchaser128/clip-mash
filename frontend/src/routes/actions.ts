import {GlobalState} from "little-state-machine"
import {FormState} from "../types/form-state"

export function updateForm(
  state: GlobalState,
  newState: Partial<FormState>,
): GlobalState {
  return {
    // @ts-expect-error broken, fixme
    data: {
      ...state.data,
      ...newState,
    },
  }
}

export function resetForm(): GlobalState {
  return {
    data: {
      source: undefined,
    },
  }
}
