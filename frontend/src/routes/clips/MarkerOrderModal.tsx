import {useStateMachine} from "little-state-machine"
import {useCallback, useState} from "react"
import Modal from "../../components/Modal"
import DraggableCard from "../../components/DraggableCard"
import {useImmer} from "use-immer"
import {getSegmentStyle} from "../../helpers"
import {HiCheck} from "react-icons/hi2"
import {updateForm} from "../actions"
import {MarkerDto} from "../../api"

interface MarkerCount {
  title: string
  count: number
}

interface MarkerGroup {
  markers: MarkerCount[]
  name: string
}

function getMarkerCounts(markers: MarkerDto[]): MarkerGroup[] {
  const counts = new Map<string, number>()
  for (const marker of markers) {
    const title = marker.primaryTag.trim()
    counts.set(title, (counts.get(title) || 0) + 1)
  }
  const groups = Array.from(counts).map(([title, count]) => ({
    markers: [{title, count}],
    name: title,
  }))

  return groups
}

const MarkerOrderModal = () => {
  const [open, setOpen] = useState(false)
  const {state, actions} = useStateMachine({updateForm})
  const initialTitles = getMarkerCounts(state.data.markers || [])
  const [groups, setGroups] = useImmer(initialTitles)
  const [addingGroup, setAddingGroup] = useState(false)
  const [groupName, setGroupName] = useState("")

  const onClose = () => {
    setOpen(false)
  }

  const moveCard = useCallback((dragIndex: number, hoverIndex: number) => {
    setGroups((draft) => {
      const temp = draft[dragIndex]
      draft.splice(dragIndex, 1)
      draft.splice(hoverIndex, 0, temp)
    })
  }, [])

  const onSave = () => {
    actions.updateForm({
      clipOrder: {
        type: "fixed",
        markerTitleGroups: groups.map((group) =>
          group.markers.map((marker) => marker.title),
        ),
      },
    })
    onClose()
  }

  const onCreateGroup = () => {
    setAddingGroup(true)
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
      <Modal position="top" size="md" isOpen={open} onClose={onClose}>
        <h1 className="text-2xl font-bold mb-2">Marker order</h1>
        <div className="flex gap-4">
          <ul className="flex flex-col gap-1 max-w-lg">
            {groups.map((group, idx) => (
              <DraggableCard
                className="w-full text-center px-4 py-1 rounded-full cursor-move"
                style={getSegmentStyle(idx, groups.length)}
                key={group.name}
                id={group.name}
                text={`${group.name} (${group.markers.reduce(
                  (prev, next) => prev + next.count,
                  0,
                )} markers)`}
                index={idx}
                moveCard={moveCard}
              />
            ))}
          </ul>
          <section className="flex flex-col">
            <h2 className="text-xl font-bold">Groups</h2>
            <div>
              <button
                onClick={onCreateGroup}
                className="btn btn-sm btn-success"
                type="button"
              >
                Add group
              </button>
            </div>

            {addingGroup && (
              <div className="form-control">
                <label className="label">
                  <span className="label-text">Group name</span>
                </label>

                <input
                  type="text"
                  className="input input-bordered"
                  value={groupName}
                  onChange={(e) => setGroupName(e.target.value)}
                />
              </div>
            )}
          </section>
        </div>
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
