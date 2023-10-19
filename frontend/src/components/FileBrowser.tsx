import {HiDocument, HiFolder} from "react-icons/hi2"
import {FileSystemEntry} from "@/api"
import clsx from "clsx"
import {formatBytes} from "@/helpers"

export interface Props {
  files: FileSystemEntry[]
  onSelectItem: (file: FileSystemEntry) => void
  currentPath: string
  onPathChange: (path: string) => void
}

const FileBrowser: React.FC<Props> = ({
  files,
  onSelectItem,
  currentPath,
  onPathChange,
}) => {
  return (
    <section className="w-full flex-grow">
      <div className="form-control">
        <label className="label">
          <span className="label-text">Path</span>
        </label>
        <input
          type="text"
          value={currentPath}
          className="input input-bordered mb-4"
          onChange={(e) => onPathChange(e.target.value)}
        />
      </div>

      <ul className="menu bg-base-200 rounded-box w-full">
        {files.map((file) => (
          <li
            key={file.fileName}
            className={clsx({
              disabled: file.type === "file",
            })}
          >
            <button onClick={() => onSelectItem(file)} type="button">
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
