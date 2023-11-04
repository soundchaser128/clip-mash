import "little-state-machine"
import {FormState} from "./types/form-state"

declare module "little-state-machine" {
  interface GlobalState {
    data: FormState
  }
}
