import {Link} from "react-router-dom"
import clsx from "clsx"

interface StepProps {
  children: React.ReactNode
  stage: number
  currentStage: number
  link: string
}

const Step: React.FC<StepProps> = ({children, currentStage, stage, link}) => {
  const isActive = currentStage >= stage
  const items = isActive ? (
    <Link className="link-primary underline" to={link}>
      {children}
    </Link>
  ) : (
    children
  )

  return <li className={clsx("step", isActive && "step-primary")}>{items}</li>
}

export default Step
