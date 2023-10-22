import {useStateMachine} from "little-state-machine"
import React, {useState} from "react"
import Modal from "@/components/Modal"
import {pluralize} from "@/helpers"
import {
  HiBarsArrowDown,
  HiCheck,
  HiChevronRight,
  HiPlus,
  HiTrash,
} from "react-icons/hi2"
import {updateForm} from "../actions"
import {MarkerDto, MarkerCount, MarkerGroup} from "@/api"
import {produce} from "immer"
import clsx from "clsx"
import {useFormContext} from "react-hook-form"
import {ClipFormInputs} from "./settings/ClipSettingsForm"

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

interface MarkerGroupsFormProps {
  markers: MarkerCount[]
  groups: MarkerGroup[]
  onSave: (groups: MarkerGroup[], markers: MarkerCount[]) => void
  onClose: (groups: MarkerGroup[], markers: MarkerCount[]) => void
}

const MarkerGroupsForm: React.FC<MarkerGroupsFormProps> = ({
  markers,
  onSave,
  groups,
  onClose,
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

    onSave(newGroups, markers)
  }

  const onRemoveFromGroup = (marker: MarkerCount, groupName: string) => {
    const newGroups = produce(groups, (draft) => {
      const group = draft.find((g) => g.name === groupName)
      if (!group) {
        return
      }
      group.markers = group.markers.filter((m) => m.title !== marker.title)
    })

    onSave(newGroups, markers)
  }

  const onReset = () => {
    onSave([], markers)
    setSelected(undefined)
  }

  const onAddNewGroup = () => {
    const newGroup = {name: `Group ${groups.length + 1}`, markers: []}

    const newGroups = [...groups, newGroup]
    onSave(newGroups, markers)
    setSelected(newGroup)
  }

  const onRemoveGroup = (groupName: string) => {
    const newGroups = groups.filter((g) => g.name !== groupName)
    onSave(newGroups, markers)
  }

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
            const isContainedInGroup = !!groups.find((g) =>
              g.markers.find((m) => m.title === marker.title),
            )
            const Icon = isContainedInGroup ? HiCheck : HiPlus
            return (
              <button
                className="btn btn-sm btn-secondary"
                key={marker.title}
                onClick={() => onAddToGroup(marker)}
                type="button"
                disabled={isContainedInGroup}
              >
                <Icon />
                {marker.title} ({marker.count})
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
              clicking on its title, then add markers to it by clicking on the
              marker titles above.
            </p>
          )}
          {groups.map((group) => {
            const enabled = selected?.name === group.name
            const markerCount = group.markers.reduce(
              (count, m) => count + m.count,
              0,
            )
            return (
              <article
                className={clsx(
                  "card card-compact bg-base-100 border-4",
                  enabled && "border-primary",
                )}
                key={group.name}
              >
                <div className="card-body">
                  <h3
                    onClick={() => setSelected(group)}
                    className="card-title flex-grow-0 cursor-pointer hover:underline transition"
                  >
                    {group.name} ({markerCount}{" "}
                    {pluralize("marker", markerCount)})
                    <button
                      onClick={() => onRemoveGroup(group.name)}
                      className="btn btn-error btn-xs btn-square btn-ghost ml-1 text-red-500"
                    >
                      <HiTrash />
                    </button>
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
                          <HiTrash />
                        </button>
                      </li>
                    ))}
                  </ul>
                </div>
              </article>
            )
          })}
        </div>
      </section>

      <div className="flex w-full justify-between mt-4">
        <button className="btn btn-warning" onClick={onReset} type="button">
          Reset
        </button>

        <button
          onClick={() => onClose(groups, markers)}
          type="button"
          className="btn btn-success"
        >
          Save
          <HiChevronRight className="ml-1" />
        </button>
      </div>
    </div>
  )
}

const MarkerOrderModal: React.FC<{className?: string}> = ({className}) => {
  const [open, setOpen] = useState(false)
  const {state, actions} = useStateMachine({updateForm})
  const initialTitles = getMarkerCounts(state.data.markers || [])
  const stateOrder = state.data.clipOptions?.clipOrder
  const groups =
    stateOrder?.type === "fixed" ? stateOrder?.markerTitleGroups : []

  const {setValue} = useFormContext<ClipFormInputs>()

  const closeModal = () => {
    setOpen(false)
  }

  const onSaveGroups = (groups: MarkerGroup[]) => {
    const newState = {
      ...state.data.clipOptions!,
      clipOrder: {
        type: "fixed" as const,
        markerTitleGroups: groups,
      },
    }

    setValue("clipOrder.markerTitleGroups", groups)

    actions.updateForm({
      clipOptions: newState,
    })
  }

  const onClose = (groups: MarkerGroup[], markerTitles: MarkerCount[]) => {
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
    closeModal()
  }

  return (
    <>
      <button
        onClick={() => setOpen(true)}
        type="button"
        className={clsx("btn btn-primary", className)}
      >
        <HiBarsArrowDown />
        Set marker order
      </button>
      <Modal position="top" size="md" isOpen={open} onClose={closeModal}>
        <MarkerGroupsForm
          groups={groups}
          markers={initialTitles}
          onSave={onSaveGroups}
          onClose={onClose}
        />
      </Modal>
    </>
  )
}

export default MarkerOrderModal
