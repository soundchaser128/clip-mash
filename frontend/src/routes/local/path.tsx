import {HiChevronRight} from "react-icons/hi2"
import {LocalFilesFormStage, StateHelpers} from "../../types/types"
import {useStateMachine} from "little-state-machine"
import {updateForm} from "../actions"
import invariant from "tiny-invariant"
import {useNavigate} from "react-router-dom"
import {useForm} from "react-hook-form"

interface Inputs {
  path: string
  recurse: boolean
  fileName: string
}

export default function SelectVideos() {
  const {state, actions} = useStateMachine({updateForm})
  invariant(StateHelpers.isLocalFiles(state.data))
  const navigate = useNavigate()

  const {register, handleSubmit} = useForm<Inputs>({
    defaultValues: {
      path: state.data.localVideoPath,
      recurse: state.data.recurse,
    },
  })

  const onSubmit = async (values: Inputs) => {
    actions.updateForm({
      source: "localFile",
      localVideoPath: values.path,
      recurse: values.recurse,
      stage: LocalFilesFormStage.ListVideos,
      fileName: values.fileName
        ? `${values.fileName} [${state.data.id}].mp4`
        : undefined,
    })
    navigate("/local/videos")
  }

  return (
    <>
      <form
        onSubmit={handleSubmit(onSubmit)}
        className="flex gap-4 items-start flex-col self-center"
      >
        <div className="form-control">
          <label className="label">
            <span className="label-text">Compilation name</span>
          </label>
          <input
            type="text"
            className="input input-bordered w-96"
            placeholder="Enter a name for your compilation (optional)"
            {...register("fileName", {required: false})}
          />
        </div>
        <div className="form-control">
          <label className="label">
            <span className="label-text">Folder containing your videos</span>
          </label>
          <input
            required
            type="text"
            className="input input-bordered w-96"
            placeholder="C:\Users\CoolUser\Videos\DefinitelyNotPorn"
            {...register("path", {required: true, minLength: 3})}
          />
        </div>
        <div className="form-control justify-between w-full">
          <label className="label cursor-pointer">
            <span className="label-text">
              Look at all the subdirectories as well
            </span>
            <input
              type="checkbox"
              className="toggle"
              {...register("recurse")}
            />
          </label>
        </div>
        <button type="submit" className="btn btn-success self-end">
          Next
          <HiChevronRight className="w-6 h-6 ml-1" />
        </button>
      </form>
    </>
  )
}
