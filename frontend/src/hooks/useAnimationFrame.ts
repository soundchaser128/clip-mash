import {useLayoutEffect, useRef} from "react"

type Time = {time: number; delta: number}
type Callback = (time: Time) => void

function normalizeTs(n: number): number {
  n -= performance.timeOrigin / 1e9
  n /= 1000.0
  return n
}

export default (cb: Callback, enabled: boolean) => {
  if (typeof performance === "undefined" || typeof window === "undefined") {
    return
  }

  const cbRef = useRef<Callback>()
  const frame = useRef<number>()
  const last = useRef(performance.now())

  cbRef.current = cb

  const animate = (now: number) => {
    const ts = normalizeTs(now)
    cbRef.current!({
      time: ts,
      delta: ts - normalizeTs(last.current),
    })
    last.current = now
    frame.current = requestAnimationFrame(animate)
  }

  useLayoutEffect(() => {
    if (enabled) {
      frame.current = requestAnimationFrame(animate)
      return () => {
        if (frame.current) {
          cancelAnimationFrame(frame.current)
        }
      }
    }
  }, [enabled])
}
