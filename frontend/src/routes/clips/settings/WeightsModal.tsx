import {useStateMachine} from "little-state-machine"
import React, {useMemo, useState} from "react"
import {useRevalidator} from "react-router-dom"
import {updateForm} from "../../actions"
import {HiCheck, HiCog8Tooth} from "react-icons/hi2"
import clsx from "clsx"
import {pluralize} from "../../../helpers"
import Modal from "../../../components/Modal"
import {useImmer} from "use-immer"
import {Clip} from "../../../api"

interface WeightsModalProps {
  className?: string
  clips: Clip[]
}

interface MarkerCount {
  total: number
  current: number
}

const WeightsModal: React.FC<WeightsModalProps> = ({className, clips}) => {
  const revalidator = useRevalidator()
  const {state, actions} = useStateMachine({updateForm})

  const markerCounts = useMemo(() => {
    const counts = new Map<string, MarkerCount>()
    for (const marker of state.data.selectedMarkers ?? []) {
      if (marker.selected) {
        const count = counts.get(marker.title) ?? {total: 0, current: 0}
        counts.set(marker.title, {
          total: count.total + 1,
          current: count.current,
        })
      }
    }
    for (const clip of clips) {
      const marker = state.data.selectedMarkers?.find(
        (m) => m.id === clip.markerId,
      )
      if (marker && marker.title && marker.selected) {
        const count = counts.get(marker.title) ?? {total: 0, current: 0}
        counts.set(marker.title, {
          total: count.total,
          current: count.current + 1,
        })
      }
    }

    return counts
  }, [state.data, clips])

  const [weights, setWeights] = useImmer<Array<[string, number]>>(() => {
    const markerTitles = Array.from(
      new Set(state.data.selectedMarkers?.map((m) => m.title.trim())),
    ).sort()

    if (state.data.clipWeights) {
      return state.data.clipWeights.filter(([title]) =>
        markerTitles.includes(title),
      )
    } else {
      const markerTitles = Array.from(
        new Set(
          state.data
            .selectedMarkers!.filter((m) => m.selected)
            .map((m) => m.title.trim()),
        ),
      ).sort()
      return Array.from(markerTitles).map((title) => [title, 1.0])
    }
  })

  const enabled = true
  const [open, setOpen] = useState(false)

  const onWeightChange = (title: string, weight: number) => {
    setWeights((draft) => {
      const index = draft.findIndex((e) => e[0] === title)
      if (index !== -1) {
        draft[index][1] = weight / 100
      }
    })
  }

  const onClose = () => {
    setOpen(false)
    if (enabled) {
      actions.updateForm({
        clipWeights: weights,
      })

      revalidator.revalidate()
    }
  }

  return (
    <>
      <button
        type="button"
        onClick={() => setOpen(true)}
        className={clsx("btn btn-secondary", className)}
      >
        <HiCog8Tooth className="mr-2" />
        Adjust marker ratios
      </button>
      <Modal position="top" size="fluid" isOpen={open} onClose={onClose}>
        <h1 className="text-2xl font-bold mb-2">Marker ratios</h1>
        <p className="text-sm mb-4">
          Here, you can adjust the likelihood of each marker type to be included
          in the compilation.
        </p>

        <div className="flex flex-col gap-4 items-center">
          {weights.map(([title, weight]) => {
            const count = markerCounts.get(title)
            const markerLabel = pluralize("marker", count?.total ?? 0)
            const clipLabel = pluralize("clip", count?.current ?? 0)

            return (
              <div
                className={clsx(
                  "form-control",
                  !enabled && "opacity-50 cursor-not-allowed",
                )}
                key={title}
              >
                <label className="label">
                  <span className="label-text">
                    <strong>{title}</strong> ({count?.total} {markerLabel},{" "}
                    {count?.current} {clipLabel})
                  </span>
                </label>
                <input
                  disabled={!enabled}
                  type="range"
                  min="0"
                  max="100"
                  className="range range-sm w-72"
                  step="5"
                  value={weight * 100}
                  onChange={(e) =>
                    onWeightChange(title, e.target.valueAsNumber)
                  }
                />
                <div className="w-full flex justify-between text-xs px-2">
                  <span>0%</span>
                  <span className="font-bold">{Math.round(weight * 100)}%</span>
                  <span>100%</span>
                </div>
              </div>
            )
          })}

          <button onClick={onClose} className="btn btn-success self-end">
            <HiCheck />
            Done
          </button>
        </div>
      </Modal>
    </>
  )
}

export default WeightsModal
