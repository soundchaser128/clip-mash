import {ListVideoDto, updateVideo} from "@/api"
import Modal from "./Modal"
import {useState} from "react"
import {HiPlus} from "react-icons/hi2"
import {useRevalidator} from "react-router-dom"

interface Props {
  video?: ListVideoDto
  onClose: () => void
}

const AddTagModal: React.FC<Props> = ({video, onClose}) => {
  const [tag, setTag] = useState("")
  const revalidator = useRevalidator()

  const onSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    if (video) {
      await updateVideo(video.video.id, {tags: [...video.video.tags, tag]})
      onClose()
      revalidator.revalidate()
      setTag("")
    }
  }

  return (
    <Modal size="fluid" onClose={onClose} isOpen={!!video}>
      <h2 className="font-bold text-2xl mb-4">Add tag to video</h2>
      <form onSubmit={onSubmit} className="flex flex-col self-center px-8">
        <div className="form-control">
          <label className="label">
            <span className="label-text">Tag</span>
          </label>
          <input
            type="text"
            className="input input-primary"
            placeholder="Enter new tag"
            value={tag}
            onChange={(e) => setTag(e.target.value)}
            autoFocus
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
