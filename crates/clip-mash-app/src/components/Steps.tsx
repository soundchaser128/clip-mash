import Step from "./Step"

interface StepInfo {
  stage: number
  link: string
  content: React.ReactNode
}

interface Props {
  steps: StepInfo[]
  currentStage: number
}

const Steps: React.FC<Props> = ({steps, currentStage}) => {
  return (
    <ul className="steps steps-vertical lg:steps-horizontal self-center mb-4">
      {steps.map((s, index) => (
        <Step
          key={index}
          link={s.link}
          currentStage={currentStage}
          stage={s.stage}
        >
          {s.content}
        </Step>
      ))}
    </ul>
  )
}

export default Steps
