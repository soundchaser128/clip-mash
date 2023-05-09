import {GlobalState} from "little-state-machine"
import {nanoid} from "nanoid"
import {State} from "../types/state"

export function updateForm(
  state: GlobalState,
  newState: Partial<State>
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
