import {HiCheck} from "react-icons/hi2"
import {LocalFilesFormStage, StateHelpers} from "../../types/types"
import {useStateMachine} from "little-state-machine"
import {updateForm} from "../actions"
import invariant from "tiny-invariant"
import {useNavigate} from "react-router-dom"
import {useForm} from "react-hook-form"

interface Inputs {
  path: string
  recurse: boolean
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
    })
    navigate("/local/videos")
  }

  return (
    <>
      <form
        onSubmit={handleSubmit(onSubmit)}
        className="flex gap-4 items-start flex-col"
      >
        <div className="form-control">
          <label className="label">
            <span className="label-text">Local path for your videos</span>
          </label>
          <input
            type="text"
            className="input input-bordered w-96"
            placeholder="C:\Users\CoolUser\Videos\DefinitelyNotPorn"
            {...register("path", {required: true, minLength: 3})}
          />
        </div>
        <div className="form-control">
          <label className="label cursor-pointer">
            <span className="label-text mr-2">
              Look at all the subdirectories as well
            </span>
            <input
              type="checkbox"
              className="toggle"
              {...register("recurse")}
            />
          </label>
        </div>
        <button type="submit" className="btn btn-success">
          <HiCheck className="w-6 h-6 mr-2" />
          Submit
        </button>
      </form>
    </>
  )
}
