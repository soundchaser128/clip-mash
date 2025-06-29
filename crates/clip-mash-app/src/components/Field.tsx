import React from "react"

interface Props {
  children: React.ReactNode
  label: React.ReactNode
  name: string
}

const Field: React.FC<Props> = ({children, label, name}) => {
  return (
    <div className="form-control">
      <label htmlFor={name} className="label">
        <span className="label-text">{label}</span>
      </label>
      {children}
    </div>
  )
}

export default Field
