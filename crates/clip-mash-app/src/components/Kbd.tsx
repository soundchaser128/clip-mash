import clsx from "clsx"
import React from "react"

interface Props {
  keys: string
  separator?: "+" | " "
  className?: string
}

const Kbd: React.FC<Props> = ({keys, className, separator = " "}) => {
  const parts = keys.split(separator)
  return (
    <span className="mr-2">
      {parts.map((part, idx) => (
        <React.Fragment key={idx}>
          <kbd className={clsx("kbd", className)}>{part}</kbd>
          {idx < parts.length - 1 && " "}
        </React.Fragment>
      ))}
    </span>
  )
}

export default Kbd
