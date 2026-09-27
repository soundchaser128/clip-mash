interface Page {
  pageNumber: number
  pageSize: number
  totalItems: number
  content: unknown[]
}

interface Props {
  page: Page
  startIndex?: 0 | 1
  className?: string
}

const PageInfo: React.FC<Props> = ({page, className, startIndex = 0}) => {
  const startRange = (page.pageNumber - startIndex) * page.pageSize + 1
  const endRange = startRange + page.content.length - 1

  return (
    <p className={className}>
      Showing videos{" "}
      <strong>
        {startRange}-{endRange}
      </strong>{" "}
      of <strong>{page.totalItems}</strong>.
    </p>
  )
}

export default PageInfo
