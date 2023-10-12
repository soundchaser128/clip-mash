import {useStateMachine} from "little-state-machine"
import React, {useCallback, useState} from "react"
import Modal from "../../components/Modal"
import DraggableCard from "../../components/DraggableCard"
import {useImmer} from "use-immer"
import {getSegmentStyle} from "../../helpers"
import {HiCheck, HiChevronRight, HiPlus} from "react-icons/hi2"
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

function getMarkerCounts(markers: MarkerDto[]): MarkerCount[] {
  const counts = new Map<string, number>()
  for (const marker of markers) {
    const title = marker.primaryTag.trim()
    counts.set(title, (counts.get(title) || 0) + 1)
  }
  const groups = Array.from(counts).map(([title, count]) => ({
    title,
    count,
  }))

  return groups
}

type Stage = "groups" | "order"

interface MarkerGroupsFormProps {
  markers: MarkerCount[]
  onSave: (groups: MarkerGroup[]) => void
}

const MarkerGroupsForm: React.FC<MarkerGroupsFormProps> = ({
  markers,
  onSave,
}) => {
  const [groups, setGroups] = useImmer<MarkerGroup[]>([])
  const [selected, setSelected] = useState<MarkerCount>()
  const [newGroup, setNewGroup] = useState<string>("")
  const [groupToAdd, setGroupToAdd] = useState<string>()

  const onAddToGroup = () => {
    if (!selected) {
      return
    }
    setGroups((draft) => {
      // if selected is already in a group, remove it
      for (const group of draft) {
        const idx = group.markers.findIndex((m) => m.title === selected.title)
        if (idx !== -1) {
          group.markers.splice(idx, 1)
          break
        }
      }
      const group = draft.find((g) => g.name === groupToAdd)
      group?.markers.push(selected)
      setSelected(undefined)
    })
  }

  const onChangeGroupToAdd = (e: React.ChangeEvent<HTMLSelectElement>) => {
    setGroupToAdd(e.target.value)
  }

  const onAddNewGroup = () => {
    if (!newGroup) {
      return
    }
    setGroups((draft) => {
      draft.push({
        name: newGroup,
        markers: [],
      })
    })
    setNewGroup("")
    setGroupToAdd(newGroup)
  }

  const onNext = () => {
    onSave(groups)
  }

  return (
    <div className="flex flex-col">
      <h1 className="text-2xl font-bold mb-2">Marker groups</h1>
      <p>
        You can group multiple markers together, so that they appear together in
        the finished compilation.
      </p>
      <section className="flex flex-col">
        <h2 className="mb-2 mt-4 font-bold text-xl">All marker titles</h2>
        <ul className="flex items-start flex-wrap flex-row gap-1">
          {markers.map((marker) => {
            const group = groups.find((g) =>
              g.markers.find((m) => m.title === marker.title),
            )?.name
            return (
              <li
                className="bg-secondary text-secondary-content px-3 py-2 rounded-lg cursor-pointer hover:bg-secondary-focus"
                key={marker.title}
                onClick={() => setSelected(marker)}
              >
                {marker.title}{" "}
                {group && <span className="text-xs">({group})</span>}
              </li>
            )
          })}
        </ul>
        <div className="flex flex-col mt-4">
          <p>
            Selected marker:{" "}
            {selected ? (
              <strong>{selected.title}</strong>
            ) : (
              <strong>None</strong>
            )}
          </p>
          <div className="flex flex-col gap-4">
            <div className="mt-4">
              <div className="form-control">
                <label className="label">
                  <span className="label-text">Create new group</span>
                </label>
                <div className="flex gap-2 items-center">
                  <input
                    type="text"
                    className="input input-bordered max-w-xs w-full"
                    value={newGroup}
                    onChange={(e) => setNewGroup(e.target.value)}
                    placeholder="Group name"
                  />
                  <button
                    onClick={onAddNewGroup}
                    className="btn btn-square btn-success btn-sm"
                    type="button"
                  >
                    <HiPlus />
                  </button>
                </div>
              </div>
            </div>

            <div className="form-control">
              <label className="label">
                <span className="label-text">Add marker to group</span>
              </label>
              <div className="flex items-center gap-2">
                <select
                  value={groupToAdd}
                  onChange={onChangeGroupToAdd}
                  className="select select-bordered w-full max-w-xs"
                  disabled={!selected || groups.length === 0}
                >
                  {groups.map((group) => (
                    <option key={group.name} value={group.name}>
                      {group.name}
                    </option>
                  ))}
                </select>
                <button
                  onClick={onAddToGroup}
                  className="btn btn-square btn-success btn-sm"
                  type="button"
                >
                  <HiPlus />
                </button>
              </div>
            </div>
          </div>
        </div>
      </section>

      <button
        onClick={onNext}
        type="button"
        className="self-end btn btn-success mt-4"
      >
        Next
        <HiChevronRight className="ml-1" />
      </button>
    </div>
  )
}

interface MarkerOrderFormProps {
  groups: MarkerGroup[]
  onSave: (groups: MarkerGroup[]) => void
}

const MarkerOrderForm: React.FC<MarkerOrderFormProps> = ({
  onSave,
  groups: initialGroups,
}) => {
  const [groups, setGroups] = useImmer<MarkerGroup[]>(initialGroups)

  const onNext = () => {
    onSave(groups)
  }

  const moveCard = useCallback((dragIndex: number, hoverIndex: number) => {
    setGroups((draft) => {
      const temp = draft[dragIndex]
      draft.splice(dragIndex, 1)
      draft.splice(hoverIndex, 0, temp)
    })
  }, [])

  return (
    <>
      <h1 className="text-2xl font-bold mb-2">Marker order</h1>
      <div className="flex flex-col gap-4 self-center">
        <ul className="flex items-start flex-col gap-1">
          {groups.map((group, idx) => (
            <DraggableCard
              className="w-full text-center px-4 py-1 rounded-full cursor-pointer"
              style={getSegmentStyle(idx, groups.length)}
              key={group.name}
              id={group.name}
              moveCard={moveCard}
              text={group.name}
              index={idx}
            />
          ))}
        </ul>
        <section className="flex flex-col"></section>
      </div>
      <button
        type="button"
        onClick={onNext}
        className="self-end btn btn-success mt-4"
      >
        <HiCheck className="mr-2" />
        Save
      </button>
    </>
  )
}

const MarkerOrderModal = () => {
  const [open, setOpen] = useState(false)
  const {state, actions} = useStateMachine({updateForm})
  const initialTitles = getMarkerCounts(state.data.markers || [])
  const [groups, setGroups] = useImmer<MarkerGroup[]>([])
  const [stage, setStage] = useState<Stage>("groups")

  const onClose = () => {
    setOpen(false)
  }

  const onSaveGroups = (groups: MarkerGroup[]) => {
    setGroups(groups)
    setStage("order")
  }

  const onSaveOrder = (groups: MarkerGroup[]) => {
    actions.updateForm({
      clipOrder: {
        type: "fixed",
        markerTitleGroups: groups.map((group) =>
          group.markers.map((m) => m.title),
        ),
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
      <Modal position="top" size="md" isOpen={open} onClose={onClose}>
        {stage === "groups" && (
          <MarkerGroupsForm markers={initialTitles} onSave={onSaveGroups} />
        )}
        {stage === "order" && (
          <MarkerOrderForm groups={groups} onSave={onSaveOrder} />
        )}
      </Modal>
    </>
  )
}

export default MarkerOrderModal
