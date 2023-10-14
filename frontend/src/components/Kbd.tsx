import clsx from "clsx"

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
        <>
          <kbd key={idx} className={clsx("kbd", className)}>
            {part}
          </kbd>
          {idx < parts.length - 1 && " "}
        </>
      ))}
    </span>
  )
}

export default Kbd
