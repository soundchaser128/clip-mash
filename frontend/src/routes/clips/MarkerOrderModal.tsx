import {useStateMachine} from "little-state-machine"
import React, {useCallback, useState} from "react"
import Modal from "../../components/Modal"
import DraggableCard from "../../components/DraggableCard"
import {useImmer} from "use-immer"
import {getSegmentStyle} from "../../helpers"
import {HiCheck, HiPlus} from "react-icons/hi2"
import {updateForm} from "../actions"
import {MarkerDto} from "../../api"
import {c} from "vitest/dist/reporters-5f784f42"

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
  onSave: () => void
}

const MarkerGroupsForm: React.FC<MarkerGroupsFormProps> = ({
  markers,
  onSave,
}) => {
  const [groups, setGroups] = useImmer<MarkerGroup[]>(
    markers.map((marker) => ({
      markers: [marker],
      name: marker.title,
    })),
  )
  const [selected, setSelected] = useState<MarkerGroup>()

  return (
    <div className="flex flex-col">
      <h1 className="text-2xl font-bold mb-2">Marker groups</h1>
      <section className="flex flex-col">
        <div className="flex flex-col gap-4">
          <ul className="flex items-start flex-row gap-1">
            {groups.map((group) => (
              <li
                className="bg-secondary text-secondary-content px-3 py-2 rounded-lg cursor-pointer hover:bg-secondary-focus"
                key={group.name}
                onClick={() => setSelected(group)}
              >
                {group.name}
              </li>
            ))}
          </ul>
        </div>
        {selected && (
          <div className="flex flex-col mt-4">
            <p>
              Selected group <strong>{selected.name}</strong>
            </p>
            <form className="flex flex-col gap-4">
              <div className="form-control">
                <label className="label">
                  <span className="label-text">Group name</span>
                </label>
                <input
                  type="text"
                  className="input input-bordered max-w-xs w-full"
                  value={selected.name}
                  onChange={(e) =>
                    setGroups((draft) => {
                      draft.find((g) => g.name === selected.name)!.name =
                        e.target.value
                    })
                  }
                />
              </div>
              <div className="form-control">
                <label className="label">
                  <span className="label-text">Add marker to group</span>
                </label>
                <div className="flex items-center gap-2">
                  <select className="select select-bordered w-full max-w-xs">
                    {markers
                      .filter(
                        (m) =>
                          !selected.markers
                            .map((m) => m.title)
                            .includes(m.title),
                      )
                      .map((marker) => (
                        <option key={marker.title}>{marker.title}</option>
                      ))}
                  </select>
                  <button
                    className="btn btn-square btn-success btn-sm"
                    type="button"
                  >
                    <HiPlus />
                  </button>
                </div>
              </div>
            </form>
          </div>
        )}
      </section>

      <button type="button" className="self-end btn btn-success mt-4">
        <HiCheck className="mr-2" />
        Save
      </button>
    </div>
  )
}

interface MarkerOrderFormProps {
  groups: MarkerGroup[]
  onSave: () => void
}

const MarkerOrderForm: React.FC<MarkerOrderFormProps> = ({onSave, groups}) => {
  return (
    <>
      <h1 className="text-2xl font-bold mb-2">Marker order</h1>
      <div className="flex flex-col gap-4 self-center">
        <ul className="flex items-start flex-row gap-1">
          {groups.map((group, idx) => (
            <li
              className="w-full text-center px-4 py-1 rounded-full cursor-pointer"
              style={getSegmentStyle(idx, groups.length)}
              key={group.name}
            >
              {group.name}
            </li>
          ))}
        </ul>
        <section className="flex flex-col"></section>
      </div>
      <button
        type="button"
        onClick={onSave}
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

  const moveCard = useCallback((dragIndex: number, hoverIndex: number) => {
    setGroups((draft) => {
      const temp = draft[dragIndex]
      draft.splice(dragIndex, 1)
      draft.splice(hoverIndex, 0, temp)
    })
  }, [])

  const onSave = () => {
    // actions.updateForm({
    //   clipOrder: {
    //     type: "fixed",
    //     markerTitleGroups: groups.map((group) =>
    //       group.markers.map((marker) => marker.title),
    //     ),
    //   },
    // })
    // onClose()
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
          <MarkerGroupsForm markers={initialTitles} onSave={() => {}} />
        )}
        {stage === "order" && (
          <MarkerOrderForm groups={groups} onSave={onSave} />
        )}
      </Modal>
    </>
  )
}

export default MarkerOrderModal
