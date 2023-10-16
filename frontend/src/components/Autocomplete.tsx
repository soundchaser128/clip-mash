import {useState} from "react"
import clsx from "clsx"
import {
  Control,
  Controller,
  FieldError,
  FieldValues,
  Path,
} from "react-hook-form"

export interface Props {
  options?: string[]
  onChange: (value: string) => void
  className?: string
  placeholder?: string
  fetchItems: (prefix: string) => Promise<string[]>
}

const Autocomplete: React.FC<Props> = ({
  options: initialOptions,
  className,
  placeholder,
  fetchItems,
}) => {
  const [open, setOpen] = useState(false)
  const [value, setValue] = useState("")
  const [options, setOptions] = useState(initialOptions || [])

  const onItemClick = (option: string) => {
    setValue(option)
    setOpen(false)
  }

  const onInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value
    setValue(value)
  }

  const onKeyDown = async (e: React.KeyboardEvent<HTMLInputElement>) => {
    const newItems = await fetchItems(e.currentTarget.value)
    setOptions(newItems)

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

interface ControlledProps<T extends FieldValues> {
  error?: FieldError
  control: Control<T>
  name: Path<T>
  options?: string[]
  placeholder?: string
  className?: string
  fetchItems: (prefix: string) => Promise<string[]>
}

function ControlledAutocomplete<T extends FieldValues>({
  control,
  options,
  error,
  name,
  placeholder,
  className,
  fetchItems,
}: ControlledProps<T>) {
  return (
    <Controller
      control={control}
      name={name}
      render={({field}) => {
        return (
          <Autocomplete
            className={className}
            placeholder={placeholder}
            options={options}
            fetchItems={fetchItems}
            {...field}
          />
        )
      }}
    />
  )
}

export default ControlledAutocomplete
