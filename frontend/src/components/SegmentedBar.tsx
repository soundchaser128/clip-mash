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
}

export const SegmentedBar: React.FC<Props> = ({
  items,
  onItemClick,
  selectedIndex,
  fadeInactiveItems,
  length,
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
    <div className="flex h-10 mt-2 gap-0.5 relative w-full">
      {items.map((item, index) => {
        const style = styles[index]
        return (
          <div
            key={index}
            className={clsx(
              "absolute text-sm text-white py-2 text-center",
              !fadeInactiveItems && "hover:opacity-80",
              index !== selectedIndex &&
                fadeInactiveItems &&
                "opacity-30 hover:opacity-60",
              index === selectedIndex && fadeInactiveItems && "opacity-100",
            )}
            style={style}
          >
            <div className="relative w-full">
              <div className="absolute w-full left-0 h-11 -top-3">
                <span className="z-30 cursor-grab border-l-[12px] border-black absolute left-0 top-0 h-full"></span>
                <span className="z-30 cursor-grab border-r-[12px] border-black absolute right-0 top-0 h-full"></span>
              </div>
            </div>
            {item.label}
          </div>
        )
      })}
    </div>
  )
}
