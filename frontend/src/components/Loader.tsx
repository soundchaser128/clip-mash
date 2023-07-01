interface Props {
  children: React.ReactNode
}

const Loader: React.FC<Props> = ({children}) => {
  return (
    <div className="self-center flex gap-4 items-center mt-4">
      <span className="loading loading-ring w-16" />
      <p>{children}</p>
    </div>
  )
}

export default Loader
