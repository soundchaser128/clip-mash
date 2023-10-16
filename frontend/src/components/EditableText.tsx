import clsx from "clsx"
import React, {useState} from "react"
import {HiCheck, HiPencil} from "react-icons/hi2"

const inputClasses =
  "grow w-full block p-0 border-0 border-b-2 border-primary appearance-none focus:outline-none focus:ring-0 focus:border-primary-focus"

interface Props {
  value?: string
  className?: string
  onSave?: (value: string) => void
}

type Mode = "edit" | "view"

const EditableText: React.FC<Props> = ({
  value: initialValue,
  className,
  onSave,
}) => {
  const [mode, setMode] = useState<Mode>("view")
  const [value, setValue] = useState<string>(initialValue ?? "")

  const onChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setValue(e.target.value)
  }

  const onSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    setMode("view")
    if (onSave) {
      onSave(value)
    }
  }

  if (mode === "edit") {
    return (
      <form onSubmit={onSubmit} className="flex grow gap-2">
        <input
          onChange={onChange}
          className={inputClasses}
          type="text"
          value={value}
        />
        <button title="Save" className="btn btn-square btn-sm btn-success">
          <HiCheck />
        </button>
      </form>
    )
  } else {
    return (
      <>
        <span className={clsx("truncate", className)}>{value}</span>{" "}
        <button className="btn btn-sm btn-square">
          <HiPencil onClick={() => setMode("edit")} title="Edit" />
        </button>
      </>
    )
  }
}

export default EditableText
