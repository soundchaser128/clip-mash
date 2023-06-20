import clsx from "clsx"
import {
  Control,
  Controller,
  FieldError,
  FieldValues,
  Path,
} from "react-hook-form"

interface Props<T extends FieldValues> {
  error?: FieldError
  control: Control<T>
  name: Path<T>
}

function TimestampInput<T extends FieldValues>({
  control,
  error,
  name,
}: Props<T>) {
  return (
    <Controller
      control={control}
      name={name}
      render={({field}) => {
        return (
          <input
            type="text"
            className={clsx(
              "input grow input-bordered",
              error && "border-error focus:outline-none"
            )}
            {...field}
            required
          />
        )
      }}
    />
  )
}

export default TimestampInput
