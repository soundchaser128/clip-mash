import clsx from "clsx"

interface Props {
  children?: React.ReactNode
  className?: string
}

const DataList: React.FC<Props> = ({children, className}) => {
  return (
    <dl className={clsx("flex flex-col text-sm", className)}>{children}</dl>
  )
}

export const Description: React.FC<Props> = ({children, className}) => {
  return <dt className={clsx("font-semibold", className)}>{children}</dt>
}

export const Data: React.FC<Props> = ({children, className}) => {
  return <dd className={clsx("mb-2", className)}>{children}</dd>
}

export default DataList
