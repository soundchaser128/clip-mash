import clsx from "clsx"
import {HiOutlineStar, HiStar} from "react-icons/hi2"

interface Props {
  rating: number
  maxRating: number
  className?: string
}

const Rating: React.FC<Props> = ({rating, maxRating, className}) => {
  const numbers = []
  for (let i = 1; i <= maxRating; i++) {
    numbers.push(i)
  }

  return (
    <div className={clsx("inline-flex items-center gap-0.5", className)}>
      {numbers.map((n) => {
        const checked = n <= rating
        const C = checked ? HiStar : HiOutlineStar
        return <C key={n} className="text-yellow-400 w-5 h-5" />
      })}
    </div>
  )
}

export default Rating
