import {useState} from "react"
import {
  HiAdjustmentsVertical,
  HiCheck,
  HiFolderPlus,
  HiPlus,
  HiXMark,
} from "react-icons/hi2"

interface VideoFile {
  name: string
  funscript?: string
  handle: FileSystemHandle
  file: File
  blobUrl: string
}

function funscriptPath(entry: FileSystemFileHandle) {
  const {name} = entry
  const idx = name.lastIndexOf(".")
  if (idx !== -1) {
    const baseName = name.substring(0, idx)
    const funscriptName = `${baseName}.funscript`
    return funscriptName
  }
}

export default function SelectVideos() {
  const [files, setFiles] = useState<VideoFile[]>([])
  const [path, setPath] = useState("")

  const onRemoveFile = (file: VideoFile) => {
    setFiles((files) => files.filter((f) => f !== file))
  }

  const onSubmit: React.FormEventHandler = async (e) => {
    e.preventDefault()
    const response = await fetch(
      `/api/list-videos?path=${encodeURIComponent(path)}`
    )
    const json = await response.json()
    console.log(json)
  }

  return (
    <>
      <form onSubmit={onSubmit} className="flex gap-4 items-start flex-col">
        <div className="form-control">
          <label className="label">
            <span className="label-text">Local path for your videos</span>
          </label>
          <input
            type="text"
            className="input input-bordered w-96"
            value={path}
            onChange={(e) => setPath(e.target.value)}
            placeholder="C:\Users\CoolUser\Videos\DefinitelyNotPorn"
          />
        </div>
        <button type="submit" className="btn btn-success">
          Submit
        </button>
      </form>
      {files && (
        <section className="grid grid-cols-1 lg:grid-cols-3 gap-2 w-full mt-4">
          {files.map((file) => (
            <article
              className="card card-compact bg-base-100 shadow-xl"
              key={file.name}
            >
              <figure className="">
                <video className="w-full" muted src={file.blobUrl} />
              </figure>
              <div className="card-body">
                <h2 className="card-title">
                  <span className="truncate">{file.name}</span>
                </h2>
                <ul>
                  <li
                    className="tooltip"
                    data-tip="Whether the scene has an associated .funscript file."
                  >
                    <HiAdjustmentsVertical className="inline mr-2" />
                    Interactive:{" "}
                    <strong>
                      {file.funscript ? (
                        <HiCheck className="text-green-600 inline" />
                      ) : (
                        <HiXMark className="text-red-600 inline" />
                      )}
                    </strong>
                  </li>
                </ul>
              </div>

              <div className="card-actions justify-end">
                <button
                  onClick={() => onRemoveFile(file)}
                  className="btn btn-error btn-sm btn-square self-end"
                >
                  <HiXMark className="w-4 h-4" />
                </button>
              </div>
            </article>
          ))}
        </section>
      )}
    </>
  )
}
