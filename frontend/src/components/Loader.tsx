import clsx from "clsx"
import * as React from "react"

interface Props {
  children?: React.ReactNode
  className?: string
}

const Loader: React.FC<Props> = ({children, className}) => {
  return (
    <div className={clsx("flex gap-4 items-center", className)}>
      <span className="loading loading-ring w-16" />
      {children && React.Children.count(children) >= 1 && <p>{children}</p>}
    </div>
  )
}

export default Loader
