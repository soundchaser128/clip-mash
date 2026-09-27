import type React from "react"

interface Props {
  children: React.ReactNode
  label: React.ReactNode
  name: string
}

const Field: React.FC<Props> = ({children, label, name}) => {
  return (
    <div className="fieldset">
      <label htmlFor={name} className="label">
        <span className="text-sm">{label}</span>
      </label>
      {children}
    </div>
  )
}

export default Field
