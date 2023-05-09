import "little-state-machine"
import {FormState} from "./types/types"

declare module "little-state-machine" {
  interface GlobalState {
    data: FormState
  }
}
