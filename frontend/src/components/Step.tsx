import {Link} from "react-router-dom"
import {FormStage} from "../types/types"
import clsx from "clsx"

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

export default Step
