import {useRef, useState} from "react"
import useAnimationFrame from "../hooks/useAnimationFrame"
import clsx from "clsx"

const BeatIndicator: React.FC<{offsets: number[]; autoPlay: boolean}> = ({
  offsets,
  autoPlay,
}) => {
  const offsetIndex = useRef(0)
  const [showBeat, setShowBeat] = useState(false)
  const [measureCount, setMeasureCount] = useState(0)

  useAnimationFrame(({time}) => {
    const nextBeat = offsets[offsetIndex.current]
    const diff = Math.abs(nextBeat - time)

    if (diff <= 0.05) {
      setShowBeat(true)
      window.setTimeout(() => setShowBeat(false), 250)
      offsetIndex.current += 1
      setMeasureCount((n) => n + 1)
    }

    // totalTime.current += delta
  }, autoPlay)

  return (
    <div className="flex items-center justify-between pl-1">
      <span className="label-text">Beat indicator</span>
      <div
        className={clsx(
          "w-12 h-12 self-center rounded-full my-2 flex items-center justify-center",
          showBeat ? "bg-red-500" : "bg-white"
        )}
      >
        {measureCount}
      </div>
    </div>
  )
}

export default BeatIndicator
