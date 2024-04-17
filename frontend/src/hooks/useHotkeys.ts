import {useEffect} from "react"
import mousetrap from "mousetrap"

type MousetrapCallback = (
  e: mousetrap.ExtendedKeyboardEvent,
  combo: string,
) => boolean | void

type PromiseCallback = (e: KeyboardEvent, combo: string) => Promise<void>

type Callback = MousetrapCallback | PromiseCallback

const useHotkeys = (
  keys: string | string[],
  callback: Callback,
  action?: string,
) => {
  useEffect(() => {
    mousetrap.bind(
      keys,
      (evt, combo) => {
        callback(evt, combo)
      },
      action,
    )
    return () => {
      mousetrap.unbind(keys)
    }
  }, [keys, action, callback])
}

export default useHotkeys
