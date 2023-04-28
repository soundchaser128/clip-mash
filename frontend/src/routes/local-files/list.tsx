import {useStateMachine} from "little-state-machine"
import invariant from "tiny-invariant"
import {LocalVideo, StateHelpers} from "../../types/types"
import {HiAdjustmentsVertical, HiCheck, HiXMark} from "react-icons/hi2"

export default function ListVideosv() {
  const {state} = useStateMachine()
  invariant(StateHelpers.isLocalFiles(state.data))
  const videos = state.data.videos!

  const onRemoveFile = (video: LocalVideo) => {
    // TODO
  }

  return (
    <>
      <section className="grid grid-cols-1 lg:grid-cols-3 gap-2 w-full mt-4">
        {videos.map((file) => (
          <article
            className="card card-compact bg-base-100 shadow-xl"
            key={file.fileName}
          >
            <figure className="">
              <video className="w-full" muted src={`/api/video/${file.id}`} />
            </figure>
            <div className="card-body">
              <h2 className="card-title">
                <span className="truncate">{file.fileName}</span>
              </h2>
              <ul>
                <li
                  className="tooltip"
                  data-tip="Whether the scene has an associated .funscript file."
                >
                  <HiAdjustmentsVertical className="inline mr-2" />
                  Interactive:{" "}
                  <strong>
                    {file.interactive ? (
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
    </>
  )
}
