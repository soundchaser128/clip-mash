import {useState, useEffect, useRef} from "react"

type Axis = "x" | "y"

interface UseDraggableOptions {
  axis: Axis
}

interface UseDraggableResult {
  x: number
  y: number
  isDragging: boolean
  handleMouseDown: React.MouseEventHandler<HTMLDivElement>
}

function useDraggable({axis}: UseDraggableOptions): UseDraggableResult {
  const [isDragging, setIsDragging] = useState(false)
  const [position, setPosition] = useState({x: 0, y: 0})

  useEffect(() => {
    if (isDragging) {
      const handleMouseMove = (event: MouseEvent) => {
        setPosition((prevPosition) => ({
          x: axis === "x" ? event.clientX : prevPosition.x,
          y: axis === "y" ? event.clientY : prevPosition.y,
        }))
      }

      const handleMouseUp = () => {
        setIsDragging(false)
      }

      document.addEventListener("mousemove", handleMouseMove)
      document.addEventListener("mouseup", handleMouseUp)

      return () => {
        document.removeEventListener("mousemove", handleMouseMove)
        document.removeEventListener("mouseup", handleMouseUp)
      }
    }
  }, [isDragging, axis])

  const handleMouseDown: React.MouseEventHandler<HTMLDivElement> = (event) => {
    event.preventDefault()
    setIsDragging(true)
  }

  return {
    x: position.x,
    y: position.y,
    isDragging,
    handleMouseDown,
  }
}

export default useDraggable
