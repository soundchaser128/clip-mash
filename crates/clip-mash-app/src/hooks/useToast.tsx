import {ToastType} from "@/components/Toast"
import {createContext, useContext, useState} from "react"

interface ToastData {
  type: ToastType
  message: string
}

interface ToastContextData {
  setToastData: (data: ToastData | undefined) => void
  data?: ToastData
}

const ToastContext = createContext<ToastContextData | undefined>(undefined)

interface ToastProviderProps {
  children: React.ReactNode
}

export const useToast = () => {
  const toastData = useContext(ToastContext)
  return toastData
}

export const useCreateToast = () => {
  const {setToastData} = useContext(ToastContext)!
  return setToastData
}

export const ToastProvider: React.FC<{children: React.ReactNode}> = ({
  children,
}: ToastProviderProps) => {
  const [toastData, setToastData] = useState<ToastData>()

  return (
    <ToastContext.Provider
      value={{
        setToastData,
        data: toastData,
      }}
    >
      {children}
    </ToastContext.Provider>
  )
}
