import {getSegmentColor, getSegmentTextColor} from "../helpers"

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

const Segments = ({items, length}: {items: Item[]; length: number}) => {
  const segments = items.map((item, index) => {
    const widthPercent = (item.length / length) * 100
    const offset = (item.offset / length) * 100
    const backgroundColor = getSegmentColor(index, items.length)
    const color = getSegmentTextColor(backgroundColor)

    return (
      <div
        key={index}
        className="absolute py-2 text-center bg-gray-300 h-10"
        style={{
          width: `${widthPercent}%`,
          left: `${offset}%`,
          backgroundColor,
          color,
        }}
      >
        <div className="relative">
          <div className="cursor-grab transition rounded-r-xl absolute h-10 w-4 hover:opacity-100 left-0 -top-2 opacity-50 bg-white z-20" />
          <div className="cursor-grab transition rounded-l-xl absolute h-10 w-4 hover:opacity-100 right-0 opacity-50 -top-2 bg-white z-20" />
        </div>
        {item.label}
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
