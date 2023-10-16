import {useState} from "react"
import clsx from "clsx"

export interface Props {
  options: string[]
  onChange: (value: string) => void
  className?: string
  placeholder?: string
}

const Autocomplete: React.FC<Props> = ({options, className, placeholder}) => {
  const [open, setOpen] = useState(false)
  const [value, setValue] = useState("")

  const onItemClick = (option: string) => {
    setValue(option)
    setOpen(false)
  }

  const onInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value
    setValue(value)
  }

  const onKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter") {
      setOpen(false)
    } else {
      setOpen(true)
    }
  }

  const onClick = () => {
    setOpen(!open)
  }

  return (
    <div
      className={clsx("dropdown", {
        "dropdown-open": open,
      })}
    >
      <input
        value={value}
        onChange={onInputChange}
        placeholder={placeholder}
        onBlur={() => setOpen(false)}
        onKeyDown={onKeyDown}
        className={clsx("input", className)}
        onClick={onClick}
      />

      <div className="dropdown-content z-10 bg-white w-full">
        <ul className="menu flex-nowrap overflow-scroll bg-base-200 shadow-xl max-h-96">
          {options.map((option) => (
            <li onClick={() => onItemClick(option)} key={option}>
              <button type="button">{option}</button>
            </li>
          ))}
        </ul>
      </div>
    </div>
  )
}

export default Autocomplete
