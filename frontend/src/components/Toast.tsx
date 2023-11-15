import clsx from "clsx"
import {useState} from "react"
import {
  HiCheck,
  HiExclamationCircle,
  HiExclamationTriangle,
  HiInformationCircle,
  HiXMark,
} from "react-icons/hi2"

export type ToastType = "success" | "error" | "warning" | "info"

interface Props {
  children: React.ReactNode
  type?: ToastType
  hideCloseButton?: boolean
}

const Toast: React.FC<Props> = ({type, children, hideCloseButton}) => {
  const [visible, setVisible] = useState(true)

  const onClose = () => {
    setVisible(false)
  }

  if (!visible) {
    return null
  }

  let Icon
  switch (type) {
    case "success":
      Icon = HiCheck
      break
    case "error":
      Icon = HiExclamationCircle
      break
    case "warning":
      Icon = HiExclamationTriangle
      break
    default:
      Icon = HiInformationCircle
      break
  }

  return (
    <div className="toast toast-top toast-center text-lg z-50">
      <div
        className={clsx("relative alert shadow-xl min-w-[400px]", {
          "alert-success": type === "success",
          "alert-error": type === "error",
          "alert-warning": type === "warning",
          "alert-info": type === "info",
        })}
      >
        <Icon className="w-8 h-8" />
        <div className="w-full text-center">{children}</div>

        {!hideCloseButton && (
          <HiXMark
            onClick={onClose}
            className="w-4 h-4 absolute top-2 right-2 cursor-pointer"
          />
        )}
      </div>
    </div>
  )
}

export default Toast
