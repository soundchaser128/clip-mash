import {uploadMusic} from "@/api"
import {useStateMachine} from "little-state-machine"
import {useState} from "react"
import {updateForm} from "../actions"
import {Link, useNavigate, useRevalidator} from "react-router-dom"
import useNotification from "@/hooks/useNotification"

const UploadMusic: React.FC = () => {
  const [file, setFile] = useState<File>()
  const [loading, setLoading] = useState(false)
  const {state, actions} = useStateMachine({updateForm})
  const revalidator = useRevalidator()
  const sendNotification = useNotification()
  const navigate = useNavigate()

  const onUpload = async () => {
    if (file) {
      setLoading(true)
      const song = await uploadMusic({file})

      actions.updateForm({
        songs: [...(state.data.songs || []), song],
      })
      revalidator.revalidate()
      sendNotification("Success", "Song downloaded successfully!")
      setLoading(false)
      navigate("/music")
    }
  }

  return (
    <div className="flex flex-col self-center w-full max-w-xl gap-4">
      <p>Select a song to upload:</p>
      <input
        type="file"
        className="file-input file-input-primary"
        name="upload"
        accept="audio/*"
        onChange={(e) => setFile(e.target.files![0])}
      />
      <div className="flex self-end gap-2">
        <Link to="/music" className="btn btn-outline">
          Cancel
        </Link>
        <button
          onClick={onUpload}
          disabled={loading}
          className="btn btn-success"
        >
          Upload
        </button>
      </div>
    </div>
  )
}

export default UploadMusic
