import clsx from "clsx"
import {useState} from "react"
import {HiXMark} from "react-icons/hi2"

interface Props {
  children: React.ReactNode
  type: "success" | "error" | "warning" | "info"
  icon?: React.ReactNode
  dismissable?: boolean
}

const Toast: React.FC<Props> = ({
  children,
  type = "info",
  icon,
  dismissable = true,
}) => {
  const [dismissed, setDismissed] = useState(false)
  if (dismissable && dismissed) {
    return null
  }

  const alertClass = clsx("alert shadow-lg relative", {
    "alert-success": type === "success",
    "alert-error": type === "error",
    "alert-warning": type === "warning",
    "alert-info": type === "info",
  })

  return (
    <div className="toast toast-top toast-center">
      <div className={alertClass}>
        <HiXMark
          onClick={() => setDismissed(true)}
          className="absolute top-0 right-0 m-2 w-3 h-3 cursor-pointer"
        />
        {icon}
        <p>{children}</p>
      </div>
    </div>
  )
}

export default Toast
