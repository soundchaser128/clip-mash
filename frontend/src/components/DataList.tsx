const DataList: React.FC<{children: React.ReactNode}> = ({children}) => {
  return <dl className="flex flex-col text-sm">{children}</dl>
}

export const Description: React.FC<{children: React.ReactNode}> = ({
  children,
}) => {
  return <dt className="font-semibold">{children}</dt>
}

export const Data: React.FC<{children: React.ReactNode}> = ({children}) => {
  return <dd className="mb-2">{children}</dd>
}

export default DataList
