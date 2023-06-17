import clsx from "clsx"
import {HiXMark} from "react-icons/hi2"

interface Props {
  isOpen: boolean
  onClose?: () => void
  children?: React.ReactNode
  className?: string
  size?: "full-screen" | "fluid"
  position?: "top" | "off-center"
}

const Modal: React.FC<Props> = ({
  isOpen,
  onClose,
  children,
  className,
  size = "full-screen",
  position = "off-center",
}) => {
  function handleClose() {
    onClose && onClose()
  }

  if (!isOpen) {
    return null
  }

  return (
    <div
      data-testid="modal-root"
      className={clsx(
        isOpen && "fixed inset-0 z-50 overflow-auto",
        !isOpen && "hidden"
      )}
    >
      <div className="fixed inset-0 bg-black opacity-50"></div>
      <div
        className={clsx(
          "fixed left-1/2 transform -translate-x-1/2",
          size === "full-screen" && "w-[95vw] top-4",
          size !== "full-screen" && position === "off-center" && "top-32",
          size !== "full-screen" && position === "top" && "top-4"
        )}
      >
        <div
          data-testid="modal-content"
          className={clsx(
            "bg-white rounded-lg p-4 flex flex-col overflow-y-auto max-h-[95vh]",
            className
          )}
        >
          {children}
        </div>

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
