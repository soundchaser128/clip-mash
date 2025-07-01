import {HiArrowUp} from "react-icons/hi2"

const JumpToTop: React.FC = () => {
  return (
    <a
      href="#top"
      className="btn btn-circle btn-md fixed right-4 bottom-4 shadow-lg"
    >
      <HiArrowUp className="w-5 h-5" />
    </a>
  )
}

export default JumpToTop
