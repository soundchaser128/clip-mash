interface Props extends React.AnchorHTMLAttributes<HTMLAnchorElement> {
  href: string
  children: React.ReactNode
}

const ExternalLink: React.FC<Props> = ({href, children, ...props}) => {
  return (
    <a
      {...props}
      href={href}
      target="_blank"
      rel="noopener noreferrer"
      className={props.className || "link link-primary"}
    >
      {children}
    </a>
  )
}

export default ExternalLink
