import clsx from "clsx"

interface Props {
  children: React.ReactNode
  className?: string
  level?: 1 | 2 | 3
  spacing?: "none" | "tight" | "loose"
}

const Heading: React.FC<Props> = ({
  children,
  className,
  level = 1,
  spacing = "loose",
}) => {
  const Tag = `h${level}` as keyof JSX.IntrinsicElements
  return (
    <Tag
      className={clsx(
        "font-bold",
        {
          "text-primary": level === 1,
          "text-5xl": level === 1,
          "text-3xl": level === 2,
          "text-2xl": level === 3,
          "mb-4": spacing === "loose",
          "mb-2": spacing === "tight",
          "mb-0": spacing === "none",
        },
        className,
      )}
    >
      {children}
    </Tag>
  )
}

export default Heading
