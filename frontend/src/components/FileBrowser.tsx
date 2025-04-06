import {HiDocument, HiFolder} from "react-icons/hi2"
import {FileSystemEntry} from "@/api"
import clsx from "clsx"
import {formatBytes} from "@/helpers/formatting"
import {Control, Controller, FieldValues, Path} from "react-hook-form"

export interface Props<T extends FieldValues> {
  files: FileSystemEntry[]
  name: Path<T>
  onSelectItem: (entry: FileSystemEntry) => void
  control: Control<T>
  drives: string[]
}

function FileBrowser<T extends FieldValues>({
  files,
  onSelectItem,
  name,
  control,
  drives,
}: Props<T>) {
  const handleSelectItem = (entry: FileSystemEntry) => {
    onSelectItem(entry)
  }

  return (
    <section className="w-full grow flex max-h-[55vh] flex-col">
      <div className="form-control">
        <label htmlFor={name} className="label">
          <span className="label-text">Path</span>
        </label>
        <Controller
          name={name}
          control={control}
          render={({field}) => (
            <input
              type="text"
              className="input input-bordered mb-4"
              required
              {...field}
            />
          )}
        />
      </div>

      {drives.length > 0 && (
        <ul className="menu bg-base-200 rounded-box w-full flex-nowrap mb-4">
          {drives.map((drive) => (
            <li key={drive}>
              <button
                onClick={() =>
                  onSelectItem({
                    fileName: drive,
                    fullPath: drive,
                    type: "directory",
                  })
                }
                type="button"
              >
                {drive}
              </button>
            </li>
          ))}
        </ul>
      )}

      <ul className="menu bg-base-200 rounded-box w-full overflow-y-scroll flex-nowrap">
        {files.map((file) => (
          <li
            key={file.fileName}
            className={clsx({
              disabled: file.type === "file",
            })}
          >
            <button onClick={() => handleSelectItem(file)} type="button">
              <span className="truncate">
                {file.type === "directory" ? (
                  <HiFolder className="inline" />
                ) : (
                  <HiDocument className="inline text-gray-400" />
                )}{" "}
                {file.fileName}
              </span>
              {file.type === "file" && (
                <span className="badge">{formatBytes(file.size)}</span>
              )}
            </button>
          </li>
        ))}
      </ul>
    </section>
  )
}

export default FileBrowser
