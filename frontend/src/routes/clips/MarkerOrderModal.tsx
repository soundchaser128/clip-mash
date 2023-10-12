import {useStateMachine} from "little-state-machine"
import React, {useCallback, useState} from "react"
import Modal from "../../components/Modal"
import DraggableCard from "../../components/DraggableCard"
import {getSegmentStyle} from "../../helpers"
import {HiCheck, HiChevronRight, HiPlus} from "react-icons/hi2"
import {updateForm} from "../actions"
import {MarkerDto} from "../../api"
import {MarkerCount, MarkerGroup} from "../../types/types"
import {produce} from "immer"

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
  groups: MarkerGroup[]
  onSave: (groups: MarkerGroup[]) => void
  onNext: (groups: MarkerGroup[], markers: MarkerCount[]) => void
}

const MarkerGroupsForm: React.FC<MarkerGroupsFormProps> = ({
  markers,
  onSave,
  groups,
  onNext,
}) => {
  const [selected, setSelected] = useState<MarkerGroup | undefined>(
    groups?.at(0),
  )
  const [newGroup, setNewGroup] = useState<string>("")
  const [groupToAdd, setGroupToAdd] = useState<string>(groups.at(0)?.name || "")

  const onAddToGroup = (marker: MarkerCount) => {
    if (!selected) {
      return
    }
    const newGroups = produce(groups, (draft) => {
      const group = draft.find((g) => g.name === selected.name)
      if (!group) {
        return
      }
      group.markers.push(marker)
    })

    onSave(newGroups)
  }

  const onReset = () => {
    onSave([])
  }

  const onChangeGroupToAdd = (e: React.ChangeEvent<HTMLSelectElement>) => {
    setGroupToAdd(e.target.value)
  }

  const onAddNewGroup = () => {
    if (!newGroup) {
      return
    }
    const newGroups = [...groups, {name: newGroup, markers: []}]
    onSave(newGroups)
    setGroupToAdd(newGroup)
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
                className="bg-secondary text-white px-3 py-2 rounded-lg cursor-pointer hover:bg-secondary-focus flex items-baseline gap-1"
                key={marker.title}
                onClick={() => onAddToGroup(marker)}
              >
                {marker.title}{" "}
                {group && <span className="text-xs">({group})</span>}
              </li>
            )
          })}
        </ul>
        <div className="flex flex-col mt-4">
          <p>
            Selected group:{" "}
            {selected ? (
              <strong>{selected.name}</strong>
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
                <span className="label-text">Select group</span>
              </label>
              <div className="flex items-center gap-2">
                <select
                  value={selected?.name || ""}
                  onChange={(e) =>
                    setSelected(groups.find((g) => g.name === e.target.value))
                  }
                  className="select select-bordered w-full max-w-xs"
                  disabled={groups.length === 0}
                >
                  {groups.map((group) => (
                    <option key={group.name} value={group.name}>
                      {group.name}
                    </option>
                  ))}
                </select>
              </div>
            </div>
          </div>
        </div>
      </section>

      <div className="flex w-full justify-between mt-4">
        <button className="btn btn-warning" onClick={onReset} type="button">
          Reset
        </button>

        <button
          onClick={() => onNext(groups, markers)}
          type="button"
          className="btn btn-success"
        >
          Next
          <HiChevronRight className="ml-1" />
        </button>
      </div>
    </div>
  )
}

interface MarkerOrderFormProps {
  groups: MarkerGroup[]
  onSave: (groups: MarkerGroup[]) => void
}

const MarkerOrderForm: React.FC<MarkerOrderFormProps> = ({onSave, groups}) => {
  const onNext = () => {
    onSave(groups)
  }

  const moveCard = useCallback(
    (dragIndex: number, hoverIndex: number) => {
      const newGroups = produce(groups, (draft) => {
        const temp = draft[dragIndex]
        draft.splice(dragIndex, 1)
        draft.splice(hoverIndex, 0, temp)
      })
      onSave(newGroups)
    },
    [groups, onSave],
  )

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
  const groups = state.data.markerGroups || []
  const [stage, setStage] = useState<Stage>("groups")

  const onClose = () => {
    setOpen(false)
  }

  const onSaveGroups = (groups: MarkerGroup[]) => {
    actions.updateForm({
      markerGroups: groups,
    })
  }

  const onSaveOrder = (groups: MarkerGroup[]) => {
    actions.updateForm({
      markerGroups: groups,
      clipOrder: {
        type: "fixed",
        markerTitleGroups: groups.map((group) =>
          group.markers.map((m) => m.title),
        ),
      },
    })
    onClose()
    setStage("groups")
  }

  const onNext = (groups: MarkerGroup[], markerTitles: MarkerCount[]) => {
    for (const marker of markerTitles) {
      // if no group contains this marker, make a group with just this marker
      const containedInGroup = groups.find((g) =>
        g.markers.find((m) => m.title === marker.title),
      )
      if (!containedInGroup) {
        groups.push({
          name: marker.title,
          markers: [marker],
        })
      }
    }

    onSaveGroups(groups)

    setStage("order")
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
          <MarkerGroupsForm
            groups={groups}
            markers={initialTitles}
            onSave={onSaveGroups}
            onNext={onNext}
          />
        )}
        {stage === "order" && (
          <MarkerOrderForm groups={groups} onSave={onSaveOrder} />
        )}
      </Modal>
    </>
  )
}

export default MarkerOrderModal
