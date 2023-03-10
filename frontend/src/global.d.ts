import "little-state-machine"
import {FormStage} from "./routes/select-mode"
import {FormState} from "./types/types"

declare module "little-state-machine" {
  interface GlobalState {
    data: FormState
  }
}
