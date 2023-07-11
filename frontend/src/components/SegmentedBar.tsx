import clsx from "clsx"
import {getSegmentColor, getSegmentTextColor} from "../helpers"
import {useCallback, useRef} from "react"
import {useDrag} from "react-dnd"

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

interface DraggableHandleProps {
  position: "left" | "right"
  onDragEnd: (position: DOMRect) => void
}

function DraggableHandle({position, onDragEnd}: DraggableHandleProps) {
  const [collected, drag, dragPreview] = useDrag(() => ({
    type: "draggable-handle",
    collect: (monitor) => ({
      isDragging: !!monitor.isDragging(),
    }),
  }))
  return (
    <span
      ref={drag}
      className={clsx(
        "z-30 cursor-grab border-primary absolute top-0 h-full",
        position === "right" && "right-0 border-r-[12px]",
        position === "left" && "left-0 border-l-[12px]",
        collected.isDragging && "opacity-50",
      )}
    />
  )
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

  const onDragStartPoint = () => {
    //
  }

  const onDragEndPoint = () => {
    //
  }

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
                <DraggableHandle position="left" onDragEnd={onDragStartPoint} />
                <DraggableHandle position="right" onDragEnd={onDragEndPoint} />
              </div>
            </div>
            {item.label}
          </div>
        )
      })}
    </div>
  )
}
