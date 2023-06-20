import {useState} from "react"
import Modal from "./Modal"
import {HiInformationCircle} from "react-icons/hi2"

interface Props {
  children: React.ReactNode
}

const HelpModal: React.FC<Props> = ({children}) => {
  const [open, setOpen] = useState(false)

  return (
    <>
      <button onClick={() => setOpen(true)} className="btn btn-info">
        <HiInformationCircle className="mr-2" />
        Information
      </button>
      <Modal
        position="off-center"
        size="fluid"
        isOpen={open}
        onClose={() => setOpen(false)}
      >
        <div className="text-gray-600 self-start">{children}</div>
        <button onClick={() => setOpen(false)} className="btn self-end mt-6">
          Close
        </button>
      </Modal>
    </>
  )
}

export default HelpModal
