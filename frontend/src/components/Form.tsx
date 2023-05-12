import clsx from "clsx"
import React from "react"

interface Props extends React.FormHTMLAttributes<HTMLFormElement> {
  children?: React.ReactNode
}

const Form: React.FC<Props> = ({className, children, ...rest}) => {
  return (
    <form {...rest} className={clsx("max-w-lg w-full flex", className)}>
      {children}
    </form>
  )
}

export default Form
