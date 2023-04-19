import clsx from "clsx"
import {useStateMachine} from "little-state-machine"
import React, {useEffect} from "react"
import {
  Link,
  LoaderFunction,
  Outlet,
  json,
  useLoaderData,
  useNavigate,
  useNavigation,
} from "react-router-dom"
import {HiXMark} from "react-icons/hi2"
import {FormStage} from "../types/types"
import {resetForm} from "./actions"
import {getUrl} from "./select/root"

export const loader: LoaderFunction = async () => {
  const response = await fetch("/api/config")
  if (response.ok) {
    const config = await response.json()
    return config
  } else {
    const error = await response.text()
    throw json({error, request: "/api/config"}, {status: 500})
  }
}

export const styles = {
  root: "min-h-screen flex flex-col justify-between transition",
  main: "flex flex-col container ml-auto mr-auto px-1",
}

export const Footer = () => {
  return (
    <footer className="w-full text-center text-sm font-light flex flex-col gap-1 my-4">
      <p>
        Made by{" "}
        <a href="https://soundchaser128.xyz" className="link">
          soundchaser128
        </a>
      </p>
      <p>
        This project is open source and available on{" "}
        <a
          className="link"
          href="https://github.com/soundchaser128/stash-compilation-maker"
        >
          GitHub
        </a>
      </p>
    </footer>
  )
}

interface StepProps {
  children: React.ReactNode
  currentStage: FormStage
  activeStage: FormStage
  link: string
}

const Step: React.FC<StepProps> = ({
  children,
  currentStage,
  activeStage,
  link,
}) => {
  const isActive = currentStage >= activeStage
  const items = isActive ? (
    <Link className="link-primary underline" to={link}>
      {children}
    </Link>
  ) : (
    children
  )

  return <li className={clsx("step", isActive && "step-primary")}>{items}</li>
}

export default function Root() {
  const {state, actions} = useStateMachine({resetForm})
  const stage = state.data.stage
  const navigate = useNavigate()
  const navigation = useNavigation()
  const isLoading = navigation.state === "loading"
  const config = useLoaderData()
  const configExists = config !== null

  useEffect(() => {
    if (!configExists) {
      navigate("/config")
    }
  }, [configExists])

  const onReset = () => {
    if (
      confirm(
        "Are you sure you want to reset the form and return to the start?"
      )
    ) {
      actions.resetForm()
      navigate("/")
    }
  }

  let criteria = "criteria"
  if (state.data.selectMode) {
    criteria = state.data.selectMode
  }

  return (
    <div className={clsx(styles.root, isLoading && "opacity-25")}>
      <main className={styles.main}>
        <section className="py-4 flex flex-col">
          <h1 className="text-4xl font-bold mb-4 text-center">
            Stash Compilation Generator
          </h1>
          <div className="self-center flex gap-2 mb-4">
            <button onClick={onReset} className="btn btn-sm btn-error">
              <HiXMark className="w-5 h-5 mr-2" />
              Reset
            </button>
          </div>
          <ul className="steps steps-vertical lg:steps-horizontal self-center mb-4">
            <Step
              currentStage={stage}
              activeStage={FormStage.SelectMode}
              link="/"
            >
              Choose mode
            </Step>
            <Step
              currentStage={stage}
              activeStage={FormStage.SelectCriteria}
              link={state.data.selectMode ? getUrl(state.data.selectMode) : ""}
            >
              Select {criteria}
            </Step>
            <Step
              currentStage={stage}
              activeStage={FormStage.SelectMarkers}
              link="/select-markers"
            >
              Select markers
            </Step>
            <Step
              currentStage={stage}
              activeStage={FormStage.VideoOptions}
              link="/video-options"
            >
              Select video options
            </Step>
            <Step
              currentStage={stage}
              activeStage={FormStage.PreviewClips}
              link="/clips"
            >
              Preview clips
            </Step>
            <Step
              currentStage={stage}
              activeStage={FormStage.Wait}
              link="/progress"
            >
              Wait for video
            </Step>
          </ul>
          <Outlet />
        </section>
      </main>
      <Footer />
    </div>
  )
}
