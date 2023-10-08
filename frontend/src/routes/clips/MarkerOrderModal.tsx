import {useStateMachine} from "little-state-machine"
import {useCallback, useState} from "react"
import Modal from "../../components/Modal"
import DraggableCard from "../../components/DraggableCard"
import {useImmer} from "use-immer"
import {getSegmentStyle} from "../../helpers"
import {HiCheck} from "react-icons/hi2"
import {updateForm} from "../actions"

const MarkerOrderModal = () => {
  const [open, setOpen] = useState(false)
  const {state, actions} = useStateMachine({updateForm})
  const initialTitles = Array.from(
    new Set(state.data.markers?.map((m) => m.primaryTag.trim()) || []),
  )
  const [markers, setMarkers] = useImmer(initialTitles)

  const onClose = () => {
    setOpen(false)
  }

  const moveCard = useCallback(
    (dragIndex: number, hoverIndex: number) => {
      setMarkers((draft) => {
        const temp = draft[dragIndex]
        draft.splice(dragIndex, 1)
        draft.splice(hoverIndex, 0, temp)
      })
    },
    [setMarkers],
  )

  const onSave = () => {
    actions.updateForm({
      clipOrder: {
        type: "fixed",
        marker_titles: markers,
      },
    })
    onClose()
  }

  return (
    <>
      <button
        onClick={() => setOpen(true)}
        type="button"
        className="btn btn-primary mt-2"
      >
        Set marker order
      </button>
      <Modal position="top" size="fluid" isOpen={open} onClose={onClose}>
        <h1 className="text-2xl font-bold mb-2">Marker order</h1>
        <ul className="flex flex-col gap-1">
          {markers.map((title, idx) => (
            <DraggableCard
              className="w-full text-center px-4 py-1 rounded-full cursor-move"
              style={getSegmentStyle(idx, markers.length)}
              key={title}
              id={title}
              text={`${idx + 1} - ${title}`}
              index={idx}
              moveCard={moveCard}
            />
          ))}
        </ul>
        <button
          type="button"
          onClick={onSave}
          className="self-end btn btn-success mt-4"
        >
          <HiCheck className="mr-2" />
          Save
        </button>
      </Modal>
    </>
  )
}

export default MarkerOrderModal
