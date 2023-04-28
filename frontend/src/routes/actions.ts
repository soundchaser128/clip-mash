import {GlobalState} from "little-state-machine"
import {FormState} from "../types/types"

export function updateForm(
  state: GlobalState,
  newState: Partial<FormState>
): GlobalState {
  return {
    // @ts-expect-error fixme
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
