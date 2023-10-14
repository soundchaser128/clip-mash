import {useStateMachine} from "little-state-machine"
import React, {useCallback, useState} from "react"
import Modal from "../../components/Modal"
import DraggableCard from "../../components/DraggableCard"
import {getSegmentStyle, pluralize} from "../../helpers"
import {HiCheck, HiChevronRight, HiPlus, HiTrash} from "react-icons/hi2"
import {updateForm} from "../actions"
import {MarkerDto} from "../../api"
import {MarkerCount, MarkerGroup} from "../../types/types"
import {produce} from "immer"
import clsx from "clsx"

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

  const onAddToGroup = (marker: MarkerCount) => {
    if (!selected) {
      return
    }
    const newGroups = produce(groups, (draft) => {
      for (const group of draft) {
        group.markers = group.markers.filter((m) => m.title !== marker.title)
      }

      const group = draft.find((g) => g.name === selected.name)
      if (!group) {
        return
      }
      group.markers.push(marker)
    })

    onSave(newGroups)
  }

  const onRemoveFromGroup = (marker: MarkerCount, groupName: string) => {
    const newGroups = produce(groups, (draft) => {
      const group = draft.find((g) => g.name === groupName)
      if (!group) {
        return
      }
      group.markers = group.markers.filter((m) => m.title !== marker.title)
    })

    onSave(newGroups)
  }

  const onReset = () => {
    onSave([])
    setSelected(undefined)
  }

  const onAddNewGroup = () => {
    const newGroups = [
      ...groups,
      {name: `Group ${groups.length + 1}`, markers: []},
    ]
    onSave(newGroups)
  }

  // TODO make the markers to be added more interactive, i.e. add a plus icon
  //  when the marker isn't part of a group yet etc.

  return (
    <div className="flex flex-col">
      <h1 className="text-2xl font-bold mb-2">Marker groups</h1>
      <p>
        You can group multiple markers together, so that they appear together in
        the finished compilation. You can also change the order of the groups in
        the next step.
      </p>
      <section className="flex flex-col">
        <h2 className="mb-2 mt-4 font-bold text-xl">All marker titles</h2>
        <div className="flex items-start flex-wrap flex-row gap-1">
          {markers.map((marker) => {
            const isContainedInGroup = groups.find((g) =>
              g.markers.find((m) => m.title === marker.title),
            )
            return (
              <button
                className={clsx(
                  "btn btn-sm",
                  !isContainedInGroup && "btn-secondary",
                )}
                key={marker.title}
                onClick={() => onAddToGroup(marker)}
                type="button"
              >
                <HiPlus className="mr-2" />
                {marker.title}
              </button>
            )
          })}
        </div>
        <div className="flex justify-between items-center">
          <h2 className="mb-2 mt-4 font-bold text-xl">Groups</h2>
          <button
            type="button"
            onClick={onAddNewGroup}
            className="btn btn-success self-end my-4"
          >
            <HiPlus className="mr-2" />
            Add new group
          </button>
        </div>
        <div className="w-full grid grid-flow-col auto-cols-fr gap-2">
          {groups.length === 0 && (
            <p className="text-center w-full">
              No groups yet. Add a new group to get started. Select a group by
              clicking on it, then add markers to it by clicking on the marker
              titles above.
            </p>
          )}
          {groups.map((group) => {
            const enabled = selected?.name === group.name
            const markerCount = group.markers.reduce(
              (count, m) => count + m.count,
              0,
            )
            return (
              <div
                onClick={() => setSelected(group)}
                className={clsx(
                  "card card-compact bg-base-100 border-2",
                  enabled && "border-primary",
                )}
                key={group.name}
              >
                <div className="card-body">
                  <h3 className="card-title flex-grow-0">
                    {group.name} ({markerCount}{" "}
                    {pluralize("marker", markerCount)})
                  </h3>
                  <ul className="text-base flex flex-wrap gap-1">
                    {group.markers.map((marker) => (
                      <li
                        className="badge badge-ghost badge-lg"
                        key={marker.title}
                      >
                        {marker.title}
                        <button
                          onClick={() => onRemoveFromGroup(marker, group.name)}
                          className="btn btn-error btn-xs btn-square btn-ghost ml-1 text-red-500"
                        >
                          <HiTrash className="" />
                        </button>
                      </li>
                    ))}
                  </ul>
                </div>
              </div>
            )
          })}
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
