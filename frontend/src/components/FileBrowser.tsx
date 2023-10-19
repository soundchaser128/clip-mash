import {HiDocument, HiFolder} from "react-icons/hi2"
import {FileSystemEntry} from "@/api"

export interface Props {
  files: FileSystemEntry[]
  onSelectItem: (file: FileSystemEntry) => void
  currentPath: string
}

const FileBrowser: React.FC<Props> = ({files, onSelectItem, currentPath}) => {
  const segments = currentPath.split("/").filter(Boolean)

  return (
    <section>
      <div className="text-sm breadcrumbs">
        <ul>
          {segments.map((path, idx) => (
            <li key={idx}>
              <a href="#">{path}</a>
            </li>
          ))}
        </ul>
      </div>
      <ul className="menu bg-base-200 rounded-box w-full">
        {files.map((file) => (
          <li key={file.name}>
            <button onClick={() => onSelectItem(file)} type="button">
              {file.type === "directory" ? (
                <HiFolder className="" />
              ) : (
                <HiDocument className="text-gray-400" />
              )}{" "}
              {file.name}
            </button>
          </li>
        ))}
      </ul>
    </section>
  )
}

export default FileBrowser
