import clsx from "clsx"
import {getSegmentColor, getSegmentTextColor} from "../helpers"

interface Item {
  label: string
  length: number
}

interface Props {
  items: Item[]
  onItemClick: (item: Item, index: number) => void
  selectedIndex?: number
  fadeInactiveItems?: boolean
}

export const SegmentedBar: React.FC<Props> = ({
  items,
  onItemClick,
  selectedIndex,
  fadeInactiveItems,
}) => {
  const total = items.reduce((total, item) => total + item.length, 0)
  const styles = items.map((item, index) => {
    const backgroundColor = getSegmentColor(index, items.length)
    const textColor = getSegmentTextColor(backgroundColor)
    const widthPercent = (item.length / total) * 100
    return {
      backgroundColor,
      color: textColor,
      width: `${widthPercent}%`,
    }
  })

  return (
    <div className="flex h-10 mt-2 gap-0.5">
      {items.map((item, index) => {
        const style = styles[index]
        return (
          <div
            key={index}
            className={clsx(
              "flex justify-center items-center text-sm cursor-pointer text-white",
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
