import useHotkeys from "@/hooks/useHotkeys"
import clsx from "clsx"
import {useEffect} from "react"
import {HiXMark} from "react-icons/hi2"
import {animated, useTransition} from "@react-spring/web"

export interface ModalProps {
  isOpen: boolean
  onClose?: () => void
  children?: React.ReactNode
  className?: string
  size?: "md" | "lg" | "fluid"
  position?: "top" | "off-center"
}

const Modal: React.FC<ModalProps> = ({
  isOpen,
  onClose,
  children,
  className,
  size = "lg",
  position = "off-center",
}) => {
  useEffect(() => {
    if (isOpen) {
      document.body.style.overflow = "hidden"
    }
    return () => {
      document.body.style.overflow = "unset"
    }
  }, [isOpen])

  function handleClose() {
    onClose && onClose()
  }

  useHotkeys("esc", handleClose)

  const transition = useTransition(isOpen, {
    from: {
      scale: 0,
      opacity: 0,
    },
    enter: {
      scale: 1,
      opacity: 1,
    },
    leave: {
      scale: 0,
      opacity: 0,
    },
  })

  if (!isOpen) {
    return null
  }

  return (
    <div
      data-testid="modal-root"
      className={clsx(
        isOpen && "fixed inset-0 z-50 overflow-auto",
        !isOpen && "hidden",
      )}
    >
      <div className="fixed inset-0 bg-black bg-opacity-50 backdrop-blur-sm"></div>
      <div
        className={clsx(
          "fixed left-1/2 transform -translate-x-1/2",
          size === "lg" && "w-[95vw] top-4 h-[90vh]",
          size !== "lg" && position === "off-center" && "top-32",
          size !== "lg" && position === "top" && "top-4",
          size === "md" && "w-[50vw]",
        )}
      >
        {transition((style, isOpen) =>
          isOpen ? (
            <animated.div
              style={style}
              data-testid="modal-content"
              className={clsx(
                "bg-base-100 rounded-lg p-4 flex flex-col overflow-y-auto max-h-[95vh]",
                className,
              )}
            >
              {children}
            </animated.div>
          ) : null,
        )}

        <button
          data-testid="modal-close-button"
          className="absolute top-1 right-1 text-gray-700 hover:text-gray-800"
          onClick={handleClose}
        >
          <HiXMark className="w-6 h-6 inline" />
        </button>
      </div>
    </div>
  )
}

export default Modal
