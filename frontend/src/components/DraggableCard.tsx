import React, {useRef} from "react"
import {useDrag, useDrop} from "react-dnd"
import type {Identifier, XYCoord} from "dnd-core"

export interface DraggableCardProps {
  id: number | string
  text: string
  index: number
  moveCard: (dragIndex: number, hoverIndex: number) => void
  className?: string
  style?: React.CSSProperties
}

interface DragItem {
  index: number
  id: string
  type: string
}

const DraggableCard: React.FC<DraggableCardProps> = ({
  id,
  text,
  index,
  moveCard,
  className,
  style,
}) => {
  const ref = useRef<HTMLDivElement>(null)
  const [{handlerId}, drop] = useDrop<
    DragItem,
    void,
    {handlerId: Identifier | null}
  >({
    accept: "CARD",
    collect(monitor) {
      return {
        handlerId: monitor.getHandlerId(),
      }
    },
    hover(item: DragItem, monitor) {
      if (!ref.current) {
        return
      }
      const dragIndex = item.index
      const hoverIndex = index

      // Don't replace items with themselves
      if (dragIndex === hoverIndex) {
        return
      }

      // Determine rectangle on screen
      const hoverBoundingRect = ref.current?.getBoundingClientRect()

      // Get vertical middle
      const hoverMiddleY =
        (hoverBoundingRect.bottom - hoverBoundingRect.top) / 2

      // Determine mouse position
      const clientOffset = monitor.getClientOffset()

      // Get pixels to the top
      const hoverClientY = (clientOffset as XYCoord).y - hoverBoundingRect.top

      // Only perform the move when the mouse has crossed half of the items height
      // When dragging downwards, only move when the cursor is below 50%
      // When dragging upwards, only move when the cursor is above 50%

      // Dragging downwards
      if (dragIndex < hoverIndex && hoverClientY < hoverMiddleY) {
        return
      }

      // Dragging upwards
      if (dragIndex > hoverIndex && hoverClientY > hoverMiddleY) {
        return
      }

      // Time to actually perform the action
      moveCard(dragIndex, hoverIndex)

      // Note: we're mutating the monitor item here!
      // Generally it's better to avoid mutations,
      // but it's good here for the sake of performance
      // to avoid expensive index searches.
      item.index = hoverIndex
    },
  })

  const [{isDragging}, drag] = useDrag({
    type: "CARD",
    item: () => {
      return {id, index}
    },
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    collect: (monitor: any) => ({
      isDragging: monitor.isDragging(),
    }),
  })

  const opacity = isDragging ? 0 : 1
  drag(drop(ref))

  return (
    <div
      ref={ref}
      data-handler-id={handlerId}
      className={className}
      style={{...style, opacity}}
    >
      {text}
    </div>
  )
}

export default DraggableCard
