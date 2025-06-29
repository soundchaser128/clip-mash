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

const PageSizeSelect: React.FC = () => {
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
        <option>15</option>
        <option>30</option>
        <option>60</option>
        <option>120</option>
        <option>240</option>
        <option>480</option>
      </select>
    </div>
  )
}

export default PageSizeSelect
