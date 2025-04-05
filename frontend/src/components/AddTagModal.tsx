import Modal from "./Modal"
import {useState} from "react"
import {HiPlus} from "react-icons/hi2"

interface Props {
  isOpen: boolean
  onSubmit: (tag: string) => void
  onClose: () => void
}

const AddTagModal: React.FC<Props> = ({onSubmit, onClose, isOpen}) => {
  const [tag, setTag] = useState("")

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    onSubmit(tag)
  }

  return (
    <Modal size="fluid" onClose={onClose} isOpen={isOpen}>
      <h2 className="font-bold text-2xl mb-4">Add tag to video</h2>
      <form onSubmit={handleSubmit} className="flex flex-col self-center px-8">
        <div className="form-control">
          <label htmlFor="newTag" className="label">
            <span className="label-text">Tag</span>
          </label>
          <input
            type="text"
            className="input input-primary"
            placeholder="Enter new tag"
            value={tag}
            onChange={(e) => setTag(e.target.value)}
            autoFocus
            name="newTag"
          />
        </div>
        <button className="btn btn-success self-end mt-2" type="submit">
          <HiPlus />
          Add
        </button>
      </form>
    </Modal>
  )
}

export default AddTagModal
