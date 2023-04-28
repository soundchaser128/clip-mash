import clsx from "clsx"
import React, {useState} from "react"
import {HiXMark} from "react-icons/hi2"

interface Props {
  isOpen: boolean
  onClose?: () => void
  children?: React.ReactNode
  className?: string
}

const Modal: React.FC<Props> = ({isOpen, onClose, children, className}) => {
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
      <div className="w-2/3 fixed top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2">
        <div
          className={clsx("bg-white rounded-lg p-8 flex flex-col", className)}
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
