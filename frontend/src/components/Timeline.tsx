import clsx from "clsx"
import {getSegmentColor, getSegmentTextColor} from "../helpers"

interface Item {
  label: string
  length: number
  offset: number
}

interface Props {
  items: Item[]
  length: number
  onItemClick: (item: Item, index: number) => void
  selectedIndex?: number
  fadeInactiveItems?: boolean
  time?: number
  markPoints?: number[]
  onMarkerClick?: (time: number, index: number) => void
}

const Timeline: React.FC<Props> = ({
  items,
  onItemClick,
  selectedIndex,
  fadeInactiveItems,
  length,
  time,
  markPoints,
  onMarkerClick,
}) => {
  const styles = items.map((item, index) => {
    const backgroundColor = getSegmentColor(index, items.length)
    const textColor = getSegmentTextColor(backgroundColor)
    const widthPercent = (item.length / length) * 100
    const offset = (item.offset / length) * 100
    return {
      backgroundColor,
      color: textColor,
      width: `${widthPercent}%`,
      left: `${offset}%`,
      display: "absolute",
    } satisfies React.CSSProperties
  })

  return (
    <div className="flex h-10 mt-2 gap-0.5 relative w-full bg-base-200">
      {typeof time === "number" && (
        <span
          style={{left: `${(time / length) * 100}%`}}
          className="absolute -bottom-2 bg-gray-700 rounded-full w-4 h-4 z-10"
        />
      )}

      {markPoints?.map((time, index) => (
        <span
          key={time}
          style={{left: `${(time / length) * 100}%`}}
          className="top-0 absolute py-2 bg-green-500 w-2 bg-opacity-50 h-10 z-10 cursor-pointer"
          onClick={() => onMarkerClick && onMarkerClick(time, index)}
        />
      ))}

      {items.map((item, index) => {
        const style = styles[index]
        return (
          <div
            key={index}
            className={clsx(
              "absolute text-sm cursor-pointer text-white py-2 text-center",
              !fadeInactiveItems && "hover:opacity-80",
              index !== selectedIndex &&
                fadeInactiveItems &&
                "opacity-30 hover:opacity-60",
              index === selectedIndex && fadeInactiveItems && "opacity-100",
            )}
            style={style}
            onClick={() => onItemClick(item, index)}
          >
            {item.label}
          </div>
        )
      })}
    </div>
  )
}

export default Timeline
