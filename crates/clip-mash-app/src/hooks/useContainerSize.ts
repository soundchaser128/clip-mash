import useResizeObserver from "@react-hook/resize-observer"
import React, {useLayoutEffect, useState} from "react"

export interface ContainerSize {
  width: number
  height: number
}

function useContainerSize(target: React.RefObject<HTMLElement>): ContainerSize {
  const [size, setSize] = useState({width: 0, height: 0})

  useLayoutEffect(() => {
    if (target.current) {
      setSize(target.current.getBoundingClientRect())
    }
  }, [target])

  useResizeObserver(target, (entry) => setSize(entry.contentRect))

  return size
}

export default useContainerSize
