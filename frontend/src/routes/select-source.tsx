import {useState} from "react"
import {HiAdjustmentsVertical, HiCheck, HiPlus, HiXMark} from "react-icons/hi2"

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

export default function SelectSource() {
  const [files, setFiles] = useState<VideoFile[]>([])

  const onClick = async () => {
    const result = await window.showDirectoryPicker()
    const entries = []
    for await (const entry of result.values()) {
      if (entry.kind === "file") {
        entries.push(entry)
      }
    }

    const videos: VideoFile[] = []
    for (const entry of entries) {
      if (entry.name.endsWith(".mp4")) {
        const funscript = entries.find((e) => e.name === funscriptPath(e))?.name
        const file = await entry.getFile()
        videos.push({
          name: entry.name,
          funscript,
          file,
          handle: entry,
          blobUrl: URL.createObjectURL(file),
        } satisfies VideoFile)
      }
    }
    setFiles((v) => [...v, ...videos])
  }

  return (
    <>
      <div className="flex justify-between items-center">
        <h1 className="text-3xl font-bold">Select videos</h1>

        <button onClick={onClick} className="btn btn-success self-start">
          <HiPlus className="w-6 h-6 mr-2" />
          Add folder
        </button>
      </div>

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
                <h2 className="card-title">{file.name}</h2>
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
            </article>
          ))}
        </section>
      )}
    </>
  )
}
