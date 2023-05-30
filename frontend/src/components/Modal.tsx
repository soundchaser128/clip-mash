import clsx from "clsx"
import {HiXMark} from "react-icons/hi2"

interface Props {
  isOpen: boolean
  onClose?: () => void
  children?: React.ReactNode
  className?: string
  size?: "full-screen" | "fluid"
}

const Modal: React.FC<Props> = ({
  isOpen,
  onClose,
  children,
  className,
  size = "full-screen",
}) => {
  function handleClose() {
    onClose && onClose()
  }

  return (
    <div
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
          size !== "full-screen" && "top-32"
        )}
      >
        <div
          className={clsx("bg-white rounded-lg p-4 flex flex-col", className)}
        >
          {children}
        </div>

        <button
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
