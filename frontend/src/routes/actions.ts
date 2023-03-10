import {GlobalState} from "little-state-machine"
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
