import DraggableCard from "@/components/DraggableCard"
import {useStateMachine} from "little-state-machine"
import {useCallback} from "react"
import {HiCheck} from "react-icons/hi2"
import {useImmer} from "use-immer"
import {updateForm} from "../actions"
import {useNavigate} from "react-router-dom"

const ReorderSongs: React.FC = () => {
  const {state, actions} = useStateMachine({updateForm})
  const songs = state.data?.songs || []
  const [selection, setSelection] = useImmer(songs.map((s) => s.songId))
  const navigate = useNavigate()

  const moveCard = useCallback((dragIndex: number, hoverIndex: number) => {
    setSelection((draft) => {
      const temp = draft[dragIndex]
      draft.splice(dragIndex, 1)
      draft.splice(hoverIndex, 0, temp)
    })
  }, [])

  const onDone = () => {
    actions.updateForm({
      songs: selection.map((id) => songs.find((s) => s.songId === id)!),
    })
    navigate("/music")
  }

  return (
    <>
      <h2 className="self-center font-bold text-xl">Change order of songs</h2>
      <p className="self-center mb-6">
        Drag and drop songs to change their order in the video.
      </p>
      <div className="self-center flex flex-col gap-2">
        {selection.map((songId, index) => {
          const song = songs.find((s) => s.songId === songId)!
          return (
            <DraggableCard
              className="text-center bg-primary text-primary-content px-4 py-1 rounded-full cursor-move"
              key={song.songId}
              id={song.songId}
              text={song.fileName}
              index={index}
              moveCard={moveCard}
            />
          )
        })}
      </div>
      <button
        onClick={onDone}
        className="btn btn-success self-end"
        type="button"
      >
        <HiCheck />
        Done
      </button>
    </>
  )
}

export default ReorderSongs
