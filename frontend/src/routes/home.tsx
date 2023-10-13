import {useStateMachine} from "little-state-machine"
import {Link, useLoaderData, useNavigate} from "react-router-dom"
import {updateForm} from "./actions"
import {useEffect, useRef, useState} from "react"
import {FormStage, FormState} from "../types/form-state"
import Layout from "../components/Layout"
import {HiFolder, HiRocketLaunch} from "react-icons/hi2"

const HomePage = () => {
  const videoId = useLoaderData() as string
  const {actions, state} = useStateMachine({updateForm})
  const [project, setProject] = useState(state.data?.fileName || "")
  const navigate = useNavigate()
  const inputRef = useRef<HTMLInputElement>(null)

  useEffect(() => {
    actions.updateForm({videoId})
  }, [actions, videoId])

  const onNext = () => {
    actions.updateForm({
      stage: FormStage.ListVideos,
      fileName: project,
    })
  }

  const onLoadProject = () => {
    inputRef.current!.click()
  }

  const onInputChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.item(0)
    if (file) {
      const text = await file.text()
      const json = JSON.parse(text) as FormState
      // TODO validate
      actions.updateForm(json)
      navigate("/library")
    }
  }

  return (
    <Layout>
      <div className="hero">
        <div className="hero-content self-center">
          <div className="max-w-md flex flex-col">
            <img src="/logo.png" className="w-40 self-center" />
            <p className="mt-2 text-lg text-center opacity-60">
              ClipMash helps you create video compilations.
            </p>
            <div className="form-control">
              <label className="label">
                <span className="label-text">Project name</span>
              </label>
              <input
                type="text"
                placeholder="Enter a project name (optional)"
                className="input input-primary input-bordered"
                value={project}
                onChange={(e) => setProject(e.target.value)}
              />
            </div>

            <div className="self-center btn-group">
              <Link
                onClick={onNext}
                className="btn btn-lg btn-primary w-64 mt-4"
                to="/library"
              >
                <HiRocketLaunch className="mr-2 w-6 h-6" />
                Start
              </Link>
            </div>
            <span className="divider">OR</span>
            <input
              accept="application/json"
              type="file"
              className="hidden"
              ref={inputRef}
              onChange={onInputChange}
            />
            <button
              onClick={onLoadProject}
              className="btn btn-lg btn-primary w-64 self-center"
            >
              <HiFolder className="mr-2 w-6 h-6" />
              Open project
            </button>
          </div>
        </div>
      </div>
    </Layout>
  )
}

export default HomePage
