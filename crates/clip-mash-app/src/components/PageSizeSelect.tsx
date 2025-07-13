import useDebouncedSetQuery from "@/hooks/useDebouncedQuery"
import useLocalStorage from "@/hooks/useLocalStorage"

const DEFAULT_ITEMS_PER_PAGE = 30

export function getPageSize(queryParams: URLSearchParams): number {
  const localStorage = window.localStorage.getItem("pageSize")
  const queryValue = queryParams.get("size")

  if (queryValue) {
    return Number(queryValue)
  } else if (localStorage) {
    return Number(localStorage)
  } else {
    return DEFAULT_ITEMS_PER_PAGE
  }
}

interface PageSizeSelectProps {
  numberOfColumns: number
}

const PageSizeSelect: React.FC<PageSizeSelectProps> = ({numberOfColumns}) => {
  const {addOrReplaceParams} = useDebouncedSetQuery()
  const [perPage, setPerPage] = useLocalStorage(
    "pageSize",
    DEFAULT_ITEMS_PER_PAGE,
  )

  const onPerPageChange = (value: number) => {
    setPerPage(value)
    addOrReplaceParams([
      ["size", value.toString()],
      ["page", "0"],
    ])
  }

  const pageSizeOptions = [
    4 * numberOfColumns,
    8 * numberOfColumns,
    12 * numberOfColumns,
    16 * numberOfColumns,
    20 * numberOfColumns,
    24 * numberOfColumns,
    48 * numberOfColumns,
  ]

  return (
    <div className="flex items-center gap-1">
      <label className="label">
        <span className="label-text">Items per page</span>
      </label>
      <select
        value={perPage}
        onChange={(e) => onPerPageChange(Number(e.target.value))}
        className="select select-sm select-bordered"
      >
        {pageSizeOptions.map((size) => (
          <option key={size} value={size}>
            {size}
          </option>
        ))}
      </select>
    </div>
  )
}

export default PageSizeSelect
