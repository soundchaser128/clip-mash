import React from "react"

interface Props {
  children: React.ReactNode
  label: React.ReactNode
}

const Field: React.FC<Props> = ({children, label}) => {
  return (
    <div className="form-control">
      <label className="label">
        <span className="label-text">{label}</span>
      </label>
      {children}
    </div>
  )
}

export default Field
