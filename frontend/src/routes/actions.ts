import {GlobalState} from "little-state-machine"
import {FormState} from "../types/types"
import {nanoid} from "nanoid"

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
      source: undefined,
      id: nanoid(8),
    },
  }
}
