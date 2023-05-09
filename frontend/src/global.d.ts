import "little-state-machine"
import {State} from "./types/state"

declare module "little-state-machine" {
  interface GlobalState {
    data: State
  }
}
