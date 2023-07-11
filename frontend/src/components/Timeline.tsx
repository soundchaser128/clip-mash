import clsx from "clsx"

interface Item {
  label: string
  length: number
  offset: number
}

interface Props {
  items: Item[]
  length: number
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

  return <div className="w-full relative">{ticks}</div>
}

interface DraggableHandleProps {
  position: "left" | "right"
}

function DraggableHandle({position}: DraggableHandleProps) {
  return (
    <span
      className={clsx(
        "cursor-grab transition absolute h-10 top-0 w-4 hover:opacity-100 opacity-40 bg-black z-20",
        position === "left" && "rounded-r-xl left-0",
        position === "right" && "rounded-l-xl right-0",
      )}
    />
  )
}

const Segments = ({items, length}: {items: Item[]; length: number}) => {
  const segments = items.map((item, index) => {
    const widthPercent = (item.length / length) * 100
    const offset = (item.offset / length) * 100

    return (
      <div
        key={index}
        className="absolute h-10 bg-slate-200 hover:bg-slate-300 text-black text-xs flex flex-col items-center justify-center text-center"
        style={{
          width: `${widthPercent}%`,
          left: `${offset}%`,
        }}
      >
        <DraggableHandle position="left" />
        {item.label}
        <DraggableHandle position="right" />
      </div>
    )
  })

  return <div className="w-full">{segments}</div>
}

const Timeline: React.FC<Props> = ({length, items}) => {
  return (
    <div className="relative overflow-hidden h-16 mt-4">
      <Segments items={items} length={length} />
      <Ticks length={length} />
    </div>
  )
}

export default Timeline
