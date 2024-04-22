import useDebouncedSetQuery from "@/hooks/useDebouncedQuery"
import useLocalStorage from "@/hooks/useLocalStorage"

const DEFAULT_ITEMS_PER_PAGE = 30

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
      </select>
    </div>
  )
}

export default PageSizeSelect
