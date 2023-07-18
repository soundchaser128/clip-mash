import clsx from "clsx"
import React, {useRef} from "react"
import Draggable from "react-draggable"

interface Item {
  label: string
  length: number
  offset: number
}

interface Props {
  items: Item[]
  length: number
  src: string
  autoPlay: boolean
}

const Ticks = ({length}: {length: number}) => {
  const ticks = []
  for (let i = 0; i < length; i++) {
    if (i % 10 === 0) {
      const left = (i / length) * 100
      ticks.push(
        <div
          key={i}
          className="w-0.5 text-xs absolute top-10 h-2 bg-gray-400"
          style={{
            left: `${left}%`,
          }}
        >
          <span className="absolute top-3 right-[-5px]">{i}</span>
        </div>,
      )
    }
  }

  return <div className="w-full relative h-10">{ticks}</div>
}

const segmentStyles =
  "absolute h-10 bg-slate-200 hover:bg-slate-300 text-black text-xs flex flex-col items-center justify-center text-center border-x-2 border-slate-500"

const Segment = ({
  width,
  offset,
  children,
}: {
  width: number
  offset: number
  children: React.ReactNode
}) => {
  const ref = useRef<HTMLDivElement>(null)
  const [dragging, setDragging] = React.useState(false)

  const onStart = () => {
    setDragging(true)
  }

  const onStop = () => {
    setDragging(false)
  }

  return (
    <Draggable onStart={onStart} onStop={onStop} nodeRef={ref} axis="x">
      <div
        ref={ref}
        className={clsx(segmentStyles, dragging && "opacity-75")}
        style={{
          width: `${width}%`,
          left: `${offset}%`,
        }}
      >
        {children}
      </div>
    </Draggable>
  )
}

const Segments = ({items, length}: {items: Item[]; length: number}) => {
  const segments = items.map((item, index) => {
    const widthPercent = (item.length / length) * 100
    const offset = (item.offset / length) * 100

    return (
      <Segment key={index} width={widthPercent} offset={offset}>
        {item.label}
      </Segment>
    )
  })

  return <div className="w-full bg-slate-50 h-10">{segments}</div>
}

const Timeline: React.FC<Props> = ({length, items, src}) => {
  return (
    <div className="relative mt-4 flex flex-col shrink-0 w-2/3">
      <video className="max-h-[90vh]" muted controls src={src} />
      <Segments items={items} length={length} />
      <Ticks length={length} />
    </div>
  )
}

export default Timeline
