import {useEffect, useRef} from "react"
import * as mousetrap from "mousetrap"

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
  const actionRef = useRef<Callback>(callback)

  useEffect(() => {
    mousetrap.bind(
      keys,
      (evt, combo) => {
        console.log(combo)
        typeof actionRef.current === "function" && actionRef.current(evt, combo)
      },
      action,
    )
    return () => {
      mousetrap.unbind(keys)
    }
  }, [keys])
}

export default useHotkeys
