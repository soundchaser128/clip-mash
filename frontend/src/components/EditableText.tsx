import React, {useEffect, useState} from "react"
import {HiPencil} from "react-icons/hi2"

const inputClasses =
  "grow w-full block p-0 border-0 border-b-2 border-primary appearance-none focus:outline-none focus:ring-0 focus:border-primary-focus"

interface Props extends React.InputHTMLAttributes<HTMLInputElement> {
  value?: string
  className?: string
  mode?: Mode
}

type Mode = "edit" | "view"

const EditableText: React.FC<Props> = ({
  value,
  className,
  mode: modeOverride,
  ...rest
}) => {
  const [mode, setMode] = useState<Mode>("view")

  useEffect(() => {
    if (modeOverride) {
      setMode(modeOverride)
    }
  }, [modeOverride])

  if (mode === "edit") {
    return (
      <input {...rest} className={inputClasses} type="text" value={value} />
    )
  } else {
    return (
      <>
        <span className={className}>{value}</span>{" "}
        <HiPencil
          onClick={() => setMode("edit")}
          title="Edit"
          className="w-4 h-4 cursor-pointer inline"
        />
      </>
    )
  }
}

export default EditableText
